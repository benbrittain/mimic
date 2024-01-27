use allocative::Allocative;

use starlark::{
    any::ProvidesStaticType,
    values::{
        starlark_value, AllocValue, Heap, NoSerialize, StarlarkValue, UnpackValue, Value, ValueLike,
    },
};
use std::fmt::{self, Display};

#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative)]
pub struct Destination {
    pub url: String,
    pub branch: String,
    pub tags: Vec<String>,
}
#[starlark_value(type = "origin")]
impl<'v> StarlarkValue<'v> for Destination {}

impl<'v> UnpackValue<'v> for Destination {
    fn unpack_value(value: starlark::values::Value<'v>) -> Option<Self> {
        value.downcast_ref::<Self>().map(|value| Self {
            branch: value.branch.clone(),
            url: value.url.clone(),
            tags: value.tags.clone(),
        })
    }
}

impl Display for Destination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "destination TODO")
    }
}

impl<'v> AllocValue<'v> for Destination {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc_simple(self)
    }
}
