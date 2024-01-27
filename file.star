def mirror_tags(ctx):
  return ctx.git.tags

def _tag_mirror_impl(ctx):
  tags = ctx.git.get_tags()

def tag_mirror():
  return git.dynamic_tags(impl = _tag_mirror_impl) #, params = {} )

core.workflow(
    name = "test-migration",
    origin = git.origin(
      url = "https://github.com/google/copybara-in.git",
      ref = "master",
    ),
    destination = git.destination(
      url = "https://github.com/google/copybara-out.git",
      branch = "master",
      tags = tag_mirror(),
    ),
)
