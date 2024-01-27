use allocative::Allocative;
use anyhow::Result;
use starlark::starlark_module;
use starlark::values::starlark_value;
use starlark::{
    any::ProvidesStaticType,
    environment::GlobalsBuilder,
    values::{AllocValue, Heap, NoSerialize, StarlarkValue, Value},
};
use std::fmt::{self, Display};

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
struct Workflow {
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

impl Workflow {
    pub fn execute(&self) -> Result<()> {
        Ok(())
    }
}
