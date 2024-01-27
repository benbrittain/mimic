use allocative::Allocative;

use starlark::{
    any::ProvidesStaticType,
    values::{AllocValue, Heap, NoSerialize, StarlarkValue, UnpackValue, Value, ValueLike},
};
use starlark_derive::starlark_value;
use std::fmt::{self, Display};

#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative)]
pub struct Origin {
    pub url: String,
    pub r#ref: String,
}
#[starlark_value(type = "origin")]
impl<'v> StarlarkValue<'v> for Origin {}

impl<'v> UnpackValue<'v> for Origin {
    fn unpack_value(value: starlark::values::Value<'v>) -> Option<Self> {
        value.downcast_ref::<Self>().map(|value| Self {
            r#ref: value.r#ref.clone(),
            url: value.url.clone(),
        })
    }
}

impl<'v> AllocValue<'v> for Origin {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc_simple(self)
    }
}

impl Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "origin TODO")
    }
}

//
//#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative)]
//pub struct Destination {
//    pub url: String,
//    pub branch: String,
//    pub tags: Vec<String>,
//}
//#[starlark_value(type = "origin")]
//impl<'v> StarlarkValue<'v> for Destination {}
//
//impl<'v> UnpackValue<'v> for Destination {
//    fn unpack_value(value: starlark::values::Value<'v>) -> Option<Self> {
//        value.downcast_ref::<Self>().map(|value| Self {
//            branch: value.branch.clone(),
//            url: value.url.clone(),
//            tags: value.tags.clone(),
//        })
//    }
//}
//
//impl Display for Destination {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        write!(f, "destination TODO")
//    }
//}
//
//impl<'v> AllocValue<'v> for Destination {
//    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
//        heap.alloc_simple(self)
//    }
//}
