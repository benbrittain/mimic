use allocative::Allocative;
use starlark::eval::Evaluator;
use starlark::starlark_module;
use starlark::values::ValueTyped;
use starlark::environment::Methods;
use starlark::environment::MethodsBuilder;
use starlark::environment::MethodsStatic;
use starlark::values::type_repr::StarlarkTypeRepr;
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

//use crate::core::{Destination, Origin};
//
//#[starlark_module]
//pub fn starlark_transform(builder: &mut GlobalsBuilder) {
//    fn transform<'v>(
//        #[starlark(require = named)] r#impl: StarlarkCallable<'v>,
//        //        #[starlark(require = named)] attrs: DictOf<'v, &'v str, &'v Value>,
//        //eval: &mut Evaluator<'v, '_>,
//    ) -> anyhow::Result<Context<'v>> {
//        Context::new(r#impl)
//        //    attrs,
//        //    eval,
//        //)
//    }
//}
#[derive(Debug, ProvidesStaticType, Trace, NoSerialize, Allocative)]
pub struct Context;

impl<'v> UnpackValue<'v> for Context {
    fn unpack_value(value: starlark::values::Value<'v>) -> Option<Self> {
        value.downcast_ref::<Self>().map(|value| Self {
            //implementation: value.implementation.clone(),
            //r#ref: value.r#ref.clone(),
            //url: value.url.clone(),
        })
    }
}

impl Context {
    pub fn new() -> anyhow::Result<Context> {
        Ok(Context { })
    }
}

impl<'v> AllocValue<'v> for Context {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc_simple(self)
    }
}

#[starlark_value(type = "transform")]
impl<'v> StarlarkValue<'v> for Context {
    fn get_type_starlark_repr() -> Ty {
        Ty::function(vec![Param::kwargs(Ty::any())], Ty::none())
    }
    fn get_methods() -> Option<&'static Methods> {
        static RES: MethodsStatic = MethodsStatic::new();
        RES.methods(context_methods)
    }
}

//#[starlark_module]
//fn context_methods(builder: &mut MethodsBuilder) {
//    fn copy<'v>(
//        this: &Context,
//        eval: &mut Evaluator<'v, '_>,
//    ) -> anyhow::Result<()> { //ValueTyped<'v, Value<'v>>> {
//        todo!()
//        //copy_file_impl(eval, this, dest, src, CopyMode::Copy, OutputType::Directory)
//    }
//
//}

//impl<'v> Freeze for Context<'v> {
//    type Frozen = FrozenContext;
//    fn freeze(self, _freezer: &Freezer) -> anyhow::Result<Self::Frozen> {
//        todo!()
//    }
//}
//
//#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative)]
//pub struct FrozenContext {}
//
//starlark_simple_value!(FrozenContext);
//#[starlark_value(type = "rule")]
//impl<'v> StarlarkValue<'v> for FrozenContext {
//    type Canonical = Context<'v>;
//}
//
impl<'v> Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "transform")
    }
}
//
//impl Display for FrozenContext {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        writeln!(f, "transform")
//    }
//}

struct RefContext<'v>(&'v Context);

impl<'v> StarlarkTypeRepr for RefContext<'v> {
    fn starlark_type_repr() -> Ty {
        Context::starlark_type_repr()
    }
}

impl<'v> UnpackValue<'v> for RefContext<'v> {
    fn unpack_value(value: Value<'v>) -> Option<Self> {
        Some(RefContext(
            value.downcast_ref::<Context>().unwrap(),
        ))
    }
}
#[starlark_module]
fn context_methods(
    builder: &mut MethodsBuilder) {
    fn copy<'v>(
        this: RefContext,
        #[starlark(require = pos)] dest: Value<'v>,
        #[starlark(require = pos)] src: Value<'v>) -> anyhow::Result<i32> {
        eprintln!("COPYING!");
        Ok(3)
    }

    fn success<'v>(this: RefContext) -> anyhow::Result<bool> {
        Ok(true)
    }
    fn fail<'v>(this: RefContext, _error: String) -> anyhow::Result<bool> {
        // TODO log error
        Ok(false)
    }
}
