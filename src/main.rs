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

use crate::core::{starlark_transform, Context, starlark_workflow};
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
        .with(starlark_workflow)
        .with(starlark_transform)
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
    let transform = workflow
        .downcast_ref::<core::Transform>()
        .expect("code should return a transform");
    dbg!(transform);
    let res = {
        let heap = module.heap();
        let mut eval = Evaluator::new(&module);
        eval.eval_function(
            transform.implementation,
            &[heap.alloc(Context::new()?)],
            &[]
            //&[heap.alloc(4), heap.alloc(2), heap.alloc(1)],
            //&[("x", heap.alloc(8))],
        )?
    };
    dbg!(res);
        //transform.execute()?;
    //let transform = transform
    //    .downcast_ref::<core::Workflow>()
    //    .expect("code should return a workflow");
    //workflow.execute()?;
    dbg!(&*store.0.borrow());
    Ok(())
}
