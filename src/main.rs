use clap::Parser;
use starlark::{
    environment::{GlobalsBuilder, Module},
    eval::Evaluator,
    syntax::{AstModule, Dialect},
    values::{ProvidesStaticType, ValueLike},
};
use std::{cell::RefCell, path::PathBuf};

mod core;
mod git;

use crate::core::starlark_workflow;
use crate::git::starlark_git;

#[derive(Debug, ProvidesStaticType, Default)]
pub struct Store(pub RefCell<Vec<String>>);

/// Migrate repository history
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The starlark entrypoint file
    #[arg(short, long)]
    input: PathBuf,

    /// When a new migration is set up, there may already
    /// be history on the remote. This informs the migration
    /// where to start the migration from within the origin.
    #[arg(long, name = "initial-revision")]
    initial_revision: Option<String>,
}

fn main() -> Result<(), starlark::Error> {
    let args = Args::parse();
    let ast = AstModule::parse_file(&args.input, &Dialect::Standard)?;

    // We build our globals adding some functions we wrote
    let globals = GlobalsBuilder::new()
        .with_struct("core", starlark_workflow)
        .with_struct("git", starlark_git)
        .build();
    let module = Module::new();
    let store = Store::default();
    let workflow = {
        let mut eval = Evaluator::new(&module);
        // We add a reference to our store
        eval.extra = Some(&store);
        eval.eval_module(ast, &globals)?
    };
    let workflow = workflow
        .downcast_ref::<core::Workflow>()
        .expect("code should return a workflow");
    workflow.execute()?;
    dbg!(&*store.0.borrow());
    Ok(())
}
