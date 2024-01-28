use allocative::Allocative;
use starlark::starlark_module;
use starlark::{
    any::ProvidesStaticType,
    environment::GlobalsBuilder,
    starlark_simple_value,
    typing::{Param, Ty},
    values::{
        starlark_value, typing::StarlarkCallable, AllocValue, Freeze, Freezer, Heap, NoSerialize,
        StarlarkValue, Trace, UnpackValue, Value, ValueLike,
    },
};
use std::fmt::{self, Display};

use crate::core::{Destination, Origin};

#[starlark_module]
pub fn starlark_git(builder: &mut GlobalsBuilder) {
    fn origin(url: String, r#ref: String) -> anyhow::Result<Origin> {
        Ok(Origin { url, r#ref })
    }

    fn destination(
        url: String,
        branch: String,
        //tags: DynamicTagCallable,
    ) -> anyhow::Result<Destination> {
        Ok(Destination {
            url,
            branch,
            tags: vec![],
        })
    }

    fn dynamic_tags<'v>(
        #[starlark(require = named)] r#impl: StarlarkCallable<'v>,
        //        #[starlark(require = named)] attrs: DictOf<'v, &'v str, &'v Value>,
        //eval: &mut Evaluator<'v, '_>,
    ) -> anyhow::Result<DynamicTagCallable<'v>> {
        DynamicTagCallable::new(r#impl)
        //    attrs,
        //    eval,
        //)
    }
}
#[derive(Debug, ProvidesStaticType, Trace, NoSerialize, Allocative)]
pub struct DynamicTagCallable<'v> {
    implementation: Value<'v>,
}

impl<'v> UnpackValue<'v> for DynamicTagCallable<'v> {
    fn unpack_value(value: starlark::values::Value<'v>) -> Option<Self> {
        value.downcast_ref::<Self>().map(|value| Self {
            implementation: value.implementation.clone(),
            //r#ref: value.r#ref.clone(),
            //url: value.url.clone(),
        })
    }
}

impl<'v> DynamicTagCallable<'v> {
    fn new(implementation: StarlarkCallable<'v>) -> anyhow::Result<DynamicTagCallable<'v>> {
        Ok(DynamicTagCallable {
            implementation: implementation.0,
        })
    }
}

impl<'v> AllocValue<'v> for DynamicTagCallable<'v> {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc_complex(self)
    }
}

#[starlark_value(type = "dynamic_tag")]
impl<'v> StarlarkValue<'v> for DynamicTagCallable<'v> {
    fn get_type_starlark_repr() -> Ty {
        Ty::function(vec![Param::kwargs(Ty::any())], Ty::none())
    }
}

impl<'v> Freeze for DynamicTagCallable<'v> {
    type Frozen = FrozenDynamicTagCallable;
    fn freeze(self, _freezer: &Freezer) -> anyhow::Result<Self::Frozen> {
        todo!()
    }
}

#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative)]
pub struct FrozenDynamicTagCallable {}

starlark_simple_value!(FrozenDynamicTagCallable);
#[starlark_value(type = "rule")]
impl<'v> StarlarkValue<'v> for FrozenDynamicTagCallable {
    type Canonical = DynamicTagCallable<'v>;
}

impl<'v> Display for DynamicTagCallable<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "dynamic_tag")
    }
}

impl Display for FrozenDynamicTagCallable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "dynamic_tag")
    }
}
