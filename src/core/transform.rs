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
pub fn starlark_transform(builder: &mut GlobalsBuilder) {
    fn transform<'v>(
        #[starlark(require = named)] r#impl: StarlarkCallable<'v>,
        //        #[starlark(require = named)] attrs: DictOf<'v, &'v str, &'v Value>,
        //eval: &mut Evaluator<'v, '_>,
    ) -> anyhow::Result<Transform<'v>> {
        Transform::new(r#impl)
        //    attrs,
        //    eval,
        //)
    }
}
#[derive(Debug, ProvidesStaticType, Trace, NoSerialize, Allocative)]
pub struct Transform<'v> {
    pub implementation: Value<'v>,
}

impl<'v> UnpackValue<'v> for Transform<'v> {
    fn unpack_value(value: starlark::values::Value<'v>) -> Option<Self> {
        value.downcast_ref::<Self>().map(|value| Self {
            implementation: value.implementation.clone(),
            //r#ref: value.r#ref.clone(),
            //url: value.url.clone(),
        })
    }
}

impl<'v> Transform<'v> {
    fn new(implementation: StarlarkCallable<'v>) -> anyhow::Result<Transform<'v>> {
        Ok(Transform {
            implementation: implementation.0,
        })
    }
}

impl<'v> AllocValue<'v> for Transform<'v> {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc_complex(self)
    }
}

#[starlark_value(type = "transform")]
impl<'v> StarlarkValue<'v> for Transform<'v> {
    fn get_type_starlark_repr() -> Ty {
        Ty::function(vec![Param::kwargs(Ty::any())], Ty::none())
    }
}

impl<'v> Freeze for Transform<'v> {
    type Frozen = FrozenTransform;
    fn freeze(self, _freezer: &Freezer) -> anyhow::Result<Self::Frozen> {
        todo!()
    }
}

#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative)]
pub struct FrozenTransform {}

starlark_simple_value!(FrozenTransform);
#[starlark_value(type = "rule")]
impl<'v> StarlarkValue<'v> for FrozenTransform {
    type Canonical = Transform<'v>;
}

impl<'v> Display for Transform<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "transform")
    }
}

impl Display for FrozenTransform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "transform")
    }
}
