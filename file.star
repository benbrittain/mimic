def _tag_mirror_impl(ctx):
    pass
#    ctx.run(move("src/", ""))

def tag_mirror():
  return git.dynamic_tags(impl = _tag_mirror_impl) #, params = {} )

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
