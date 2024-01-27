use clap::Parser;
use starlark::{
    environment::{GlobalsBuilder, Module},
    eval::Evaluator,
    syntax::{AstModule, Dialect},
    values::ProvidesStaticType,
};
use std::{cell::RefCell, path::PathBuf};

mod core;
mod git;

use crate::core::starlark_workflow;
use crate::git::starlark_git;

#[derive(Debug, ProvidesStaticType, Default)]
pub struct Store(pub RefCell<Vec<String>>);

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: PathBuf,
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
    {
        let mut eval = Evaluator::new(&module);
        // We add a reference to our store
        eval.extra = Some(&store);
        dbg!(eval.eval_module(ast, &globals)?);
    }
    dbg!(&*store.0.borrow());
    Ok(())
}
