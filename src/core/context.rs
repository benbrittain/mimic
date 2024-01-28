use allocative::Allocative;
use starlark::environment::Methods;
use starlark::environment::MethodsBuilder;
use starlark::environment::MethodsStatic;
use starlark::eval::Evaluator;
use starlark::starlark_module;
use starlark::values::type_repr::StarlarkTypeRepr;
use starlark::values::ValueTyped;
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
use std::path::{Path, PathBuf};

#[derive(Debug, ProvidesStaticType, Trace, NoSerialize, Allocative)]
pub struct Context {
    origin_path: PathBuf,
    dest_path: PathBuf,
}

impl<'v> UnpackValue<'v> for Context {
    fn unpack_value(value: starlark::values::Value<'v>) -> Option<Self> {
        value.downcast_ref::<Self>().map(|value| Self {
            origin_path: value.origin_path.clone(),
            dest_path: value.dest_path.clone(),
        })
    }
}

impl Context {
    pub fn new(origin_path: &Path, dest_path: &Path) -> anyhow::Result<Context> {
        Ok(Context {
            origin_path: origin_path.to_path_buf(),
            dest_path: dest_path.to_path_buf(),
        })
    }
}

impl<'v> AllocValue<'v> for Context {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc_simple(self)
    }
}

#[starlark_value(type = "context")]
impl<'v> StarlarkValue<'v> for Context {
    fn get_type_starlark_repr() -> Ty {
        Ty::function(vec![Param::kwargs(Ty::any())], Ty::none())
    }
    fn get_methods() -> Option<&'static Methods> {
        static RES: MethodsStatic = MethodsStatic::new();
        RES.methods(context_methods)
    }
}

impl<'v> Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

struct RefContext<'v>(&'v Context);

impl<'v> StarlarkTypeRepr for RefContext<'v> {
    fn starlark_type_repr() -> Ty {
        Context::starlark_type_repr()
    }
}

impl<'v> UnpackValue<'v> for RefContext<'v> {
    fn unpack_value(value: Value<'v>) -> Option<Self> {
        Some(RefContext(value.downcast_ref::<Context>().unwrap()))
    }
}
#[starlark_module]
fn context_methods(builder: &mut MethodsBuilder) {
    fn copy<'v>(
        this: RefContext,
        #[starlark(require = pos)] src: Value<'v>,
        #[starlark(require = pos)] dest: Value<'v>,
    ) -> anyhow::Result<bool> {
        let mut origin_path = this.0.origin_path.clone();
        origin_path.push(src.to_str());
        let mut dest_path = this.0.dest_path.clone();
        dest_path.push(dest.to_str());
        copy_recursively(origin_path, dest_path)?;
        Ok(true)
    }

    fn success<'v>(this: RefContext) -> anyhow::Result<bool> {
        Ok(true)
    }
    fn fail<'v>(this: RefContext, _error: String) -> anyhow::Result<bool> {
        // TODO log error
        Ok(false)
    }
}

pub fn copy_recursively(
    source: impl AsRef<Path>,
    destination: impl AsRef<Path>,
) -> std::io::Result<()> {
    std::fs::create_dir_all(&destination)?;
    for entry in std::fs::read_dir(&source)? {
        let entry = entry?;
        let mut source_git = source.as_ref().to_path_buf();
        source_git.push(".git");
        if entry.path().as_path().starts_with(source_git) {
            continue;
        }
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
