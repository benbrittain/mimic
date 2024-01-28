use allocative::Allocative;
use anyhow::Result;
use git2::Repository;
use starlark::environment::Module;
use starlark::eval::Evaluator;
use starlark::starlark_module;
use starlark::values::none::NoneType;
use starlark::values::starlark_value;
use starlark::values::Freeze;
use starlark::values::Freezer;
use starlark::values::Trace;
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

mod transform;
pub use transform::*;

mod context;
pub use context::*;

//use crate::WorkflowStore;

#[starlark_module]
pub fn starlark_workflow(builder: &mut GlobalsBuilder) {
    fn workflow<'v>(
        name: String,
        origin: Origin,
        destination: Destination,
        transform: Transform<'v>,
        eval: &mut Evaluator<'v, '_>,
    ) -> anyhow::Result<NoneType> {
        let workflow = eval.heap().alloc(Workflow {
            name: name.clone(),
            origin,
            destination,
            transform,
        });
        eval.module().set(&name, workflow);
        Ok(NoneType)
    }
}

#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative, Trace)]
pub struct Workflow<'v> {
    name: String,
    origin: Origin,
    destination: Destination,
    transform: Transform<'v>,
}

#[starlark_value(type = "workflow")]
impl<'v> StarlarkValue<'v> for Workflow<'v> {}

impl<'v> Display for Workflow<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "workflow")
    }
}

impl Display for FrozenWorkflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "frozen workflow")
    }
}

impl<'v> AllocValue<'v> for Workflow<'v> {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc_complex(self)
    }
}

impl<'v> Freeze for Workflow<'v> {
    type Frozen = FrozenWorkflow;
    fn freeze(self, freezer: &Freezer) -> anyhow::Result<Self::Frozen> {
        todo!()
    }
}

#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative)]
pub struct FrozenWorkflow {}

#[starlark_value(type = "workflow")]
impl<'v> StarlarkValue<'v> for FrozenWorkflow {
    type Canonical = Workflow<'v>;
}
//let frozen_impl = self.implementation.freeze(freezer)?; FrozenValue
//let docs = self.docs; Option<String>
//let id = match self.id.into_inner() { BxlFunctionLabel
//    Some(x) => x, BxlFunctionLabel
//    None => return Err(BxlError::BxlNotAssigned(self.bxl_path.to_string()).into()),
//};
//let bxl_id = Arc::new(id); Arc<BxlFunctionLabel>

//Ok(FrozenBxlFunction {
//    implementation: frozen_impl,
//    cli_args: self.cli_args,
//    bxl_id,
//    docs,
//})

fn print_time(time: &git2::Time, prefix: &str) {
    let (offset, sign) = match time.offset_minutes() {
        n if n < 0 => (-n, '-'),
        n => (n, '+'),
    };
    let (hours, minutes) = (offset / 60, offset % 60);
    let seconds = time.seconds();

    let datetime: chrono::DateTime<chrono::Utc> =
        chrono::DateTime::from_timestamp(seconds, 0).unwrap();

    // Format the datetime how you want
    let newdate = datetime.format("%Y-%m-%d %H:%M:%S");

    println!("{prefix}{newdate} {sign}{hours:02}{minutes:02}");
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
    print_time(&author.when(), "Date:   ");
    println!();

    for line in String::from_utf8_lossy(commit.message_bytes()).lines() {
        println!("    {}", line);
    }
    println!();
}

fn find_last_migrated_commit_sha(repo: &git2::Repository) -> Result<Option<String>> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    let revwalk = revwalk.filter_map(|id| {
        let id = id.unwrap();
        let commit = repo.find_commit(id).unwrap();
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

impl<'a> Workflow<'a> {
    pub fn execute(&'a self, module: &'a Module, eval: &mut Evaluator<'a, '_>) -> Result<()> {
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

        let mut eval = Evaluator::new(&module);
        for object in revwalk {
            let treeish = object?;
            origin_repo
                .checkout_tree(&treeish, Some(git2::build::CheckoutBuilder::new().force()))?;
            let commit = treeish.peel_to_commit()?;
            let message = format!(
                "{}\nGitOrigin-RevId: {}",
                commit.message().unwrap(),
                commit.id()
            );
            let author_sig = commit.author();
            let commiter_sig = commit.committer();

            let ctx = Context::new(origin_repo.workdir().unwrap(), dest_repo.workdir().unwrap())?;
            let heap = module.heap();

            // RUN THE TRANSFORMS
            // TODO pass params in as well
            eval.eval_function(self.transform.implementation, &[heap.alloc(ctx)], &[])
                .unwrap();

            let parent_commit = find_last_commit(&dest_repo)?;
            //print_commit(&parent_commit);
            let mut index = dest_repo.index()?;
            index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
            index.write()?;
            let oid = index.write_tree()?;
            dest_repo.commit(
                Some("HEAD"),
                &author_sig,
                &commiter_sig,
                &message,
                &dest_repo.find_tree(oid)?,
                &[&parent_commit],
            )?;
        }
        Ok(())
    }
}
