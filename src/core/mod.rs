use allocative::Allocative;
use anyhow::{anyhow, Result};
use git2::Repository;
use starlark::starlark_module;
use starlark::values::starlark_value;
use starlark::{
    any::ProvidesStaticType,
    environment::GlobalsBuilder,
    values::{AllocValue, Heap, NoSerialize, StarlarkValue, Value},
};
use std::{
    fmt::{self, Display},
    io,
    path::{Path, PathBuf},
};

mod origin;
pub use origin::*;

mod destination;
pub use destination::*;

#[starlark_module]
pub fn starlark_workflow(builder: &mut GlobalsBuilder) {
    fn workflow(
        name: String,
        origin: Origin,
        destination: Destination,
    ) -> anyhow::Result<Workflow> {
        Ok(Workflow {
            name,
            origin,
            destination,
        })
    }
}

#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative)]
pub struct Workflow {
    name: String,
    origin: Origin,
    destination: Destination,
}

#[starlark_value(type = "workflow")]
impl<'v> StarlarkValue<'v> for Workflow {}

impl Display for Workflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "workflow")
    }
}

impl<'v> AllocValue<'v> for Workflow {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc_simple(self)
    }
}

fn print_commit(commit: &git2::Commit) {
    println!("commit {}", commit.id());

    if commit.parents().len() > 1 {
        print!("Merge:");
        for id in commit.parent_ids() {
            print!(" {:.8}", id);
        }
        println!();
    }

    let author = commit.author();
    println!("Author: {}", author);
    //print_time(&author.when(), "Date:   ");
    //println!();

    for line in String::from_utf8_lossy(commit.message_bytes()).lines() {
        println!("    {}", line);
    }
    println!();
}

pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    std::fs::create_dir_all(&destination)?;
    for entry in std::fs::read_dir(&source)? {
        let entry = entry?;
        let mut source_git = source.as_ref().to_path_buf();
        source_git.push(".git");
        if entry.path().as_path().starts_with(source_git) {
            continue;
        }
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn find_last_migrated_commit_sha(repo: &git2::Repository) -> Result<Option<String>> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    let revwalk = revwalk.filter_map(|id| {
        let id = id.unwrap();
        let commit = repo.find_commit(id).unwrap();
        dbg!(&commit);
        Some(Ok::<_, anyhow::Error>(commit))
    });

    for object in revwalk {
        let commit = object?;
        if let Some(body) = commit.message() {
            for (k, v) in git2::message_trailers_strs(body)?.iter() {
                if k == "GitOrigin-RevId" {
                    return Ok(Some(v.to_string()));
                }
            }
        }
    }
    return Ok(None);
}

fn get_repo(url: &str, branch: &str) -> Result<Repository> {
    // TODO fetch callbacks
    let callbacks = git2::RemoteCallbacks::new();
    let mut fo = git2::FetchOptions::new();

    fo.remote_callbacks(callbacks);

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);
    builder.branch(branch);

    let mut repo_path = PathBuf::from("cache/");
    repo_path.push(format!("{url}-{branch}").replace("/", "-"));
    let repo = builder.clone(url, &repo_path)?;
    Ok(repo)
}

fn find_last_commit(repo: &Repository) -> Result<git2::Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(git2::ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

impl Workflow {
    pub fn execute(&self) -> Result<()> {
        let origin_repo = get_repo(&self.origin.url, &self.origin.r#ref)?;
        let dest_repo = get_repo(&self.destination.url, &self.destination.branch)?;
        let start_rev = find_last_migrated_commit_sha(&dest_repo)?
            .unwrap_or_else(|| todo!("implement '--initial-revision'"));

        // Find last migrated object

        let mut revwalk = origin_repo.revwalk()?;
        revwalk.push_range(&format!("{start_rev}..HEAD"))?;
        revwalk.set_sorting(git2::Sort::REVERSE)?;
        let revwalk = revwalk.filter_map(|id| {
            let id = id.unwrap();
            let object = origin_repo
                .find_object(id, Some(git2::ObjectType::Commit))
                .unwrap();
            Some(Ok::<_, anyhow::Error>(object))
        });

        for object in revwalk {
            let treeish = object?;
            origin_repo
                .checkout_tree(&treeish, Some(git2::build::CheckoutBuilder::new().force()))?;
            let commit = treeish.peel_to_commit()?;
            let message = commit.message().unwrap();

            // TODO RUN THE TRANSFORMS HERE INSTEAD
            copy_recursively(origin_repo.workdir().unwrap(), dest_repo.workdir().unwrap())?;

            let sig = git2::Signature::now("Mimic", "migrate@mimic.dev")?;
            let parent_commit = find_last_commit(&dest_repo)?;
            print_commit(&parent_commit);
            let mut index = dest_repo.index()?;
            index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
            index.write()?;
            let oid = index.write_tree()?;
            dest_repo.commit(
                Some("HEAD"),
                &sig,
                &sig,
                &message,
                &dest_repo.find_tree(oid)?,
                &[&parent_commit],
            )?; // parents
        }

        //let dest_repo = Repository::clone(&self.destination.url, "./cache/dest")?;
        Ok(())
    }
}
