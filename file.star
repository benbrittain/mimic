def _tag_mirror_impl(ctx):
    ctx.copy("src/", "")
    return ctx.success()

def tag_mirror():
  return transform(impl = _tag_mirror_impl) #, params = {} )

workflow(
    name = "test-migration",
    origin = git.origin(
      url = "https://github.com/benbrittain/buckle",
      ref = "main",
    ),
    destination = git.destination(
      url = "file:///home/ben/workspace/buckle",
      branch = "mirror",
    ),
#    transforms = tag_mirror(), #core.move("src", ""),
)

tag_mirror()
