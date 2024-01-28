def _tag_mirror_impl(ctx):
    #if not ctx.read_json("file.json")["NO_UPDATE"]:
    print(call_stack())
    ctx.copy("src/", "")


    #tags = ctx.get_git_tags();
    #ctx.replace(
    #    search = "UNIQUE_SIGIL",
    #    replace = "new_value"
    #)
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
    transform = tag_mirror(),
)
