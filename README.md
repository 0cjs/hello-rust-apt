hello-rust-apt
==============

This repo is an example of building, packaging and testing a simple "Hello,
world" server. It's intended to create a Debian package installable via
`dpkg -i` or `apt`.

Development branches should be named `dev/NAME/…` where _name_ is a short
tag indicating the creator/owner of a branch, and _…_ is anything you find
suitable. They should be rebased when necessary to make your sequence of
commits tell a clear "story" for review.

cjs's notes [Creating and Building Debian Packages][bld-debian] will
probably be useful for understanding the packaging.


Overview
--------

The top level `Test` script will do everything; it requires only a minimal
Python and Docker to be installed. It accepts the following options:
- `-C`: Do a "big" clean, removing everything and rebuilding all from
  scratch.
- `-c`: Do a "small" clean for a faster rebuild by removing all but
  framework items unlikely ever to change, such as the pactivate bootstrap.
- `-v`: Make the builds more verbose.
- `-V`: Make the builds very verbose (particularly Docker builds).
- `-i prefix`: Change the "$USER-hra" prefix used on images/containers
  below to _prefix._

For ease of experimentation and confirmation of dependencies (both system-
and project-supplied), this does its work in two Docker images/containers:
- `$USER-hra-build`: Installs all dependencies required to compile and
  package the software, compiles and packages it, and runs a basic test.
  This is done entirely via `docker build`; a container of the same name is
  started from the image so you can interactively test and debug it.
- `hra-test`: Installs the package from above, starts the server, and
  confirms that it's working.

Both of the above use `hello-server-test` to test the server; that
simply connects to it and confirms it's returning the right response.

The containers are left running for six hours so you can enter them (using
`dent` from the virtual environment or just `docker exec -it`) to poke
around, debug things, etc. Running `Test` again will kill and remove any
existing containers created by this system and start new ones.

### Files and Directories

- `webhello/`: Source for Rust "web Hello" program, a web server that
  replies to `GET /hello`.
- `test-hello-server`: Connects to the server, confirming that it's running
  and giving a correct response.
- `docker/`: Docker configurations for build and test containers
- `package/`: Debian packaging scripts and configuration (non-standard format).
- `.build/docker/*`: Docker contexts for the image builds.


Details
-------

### Python virtualenv and `dent`

The `Test` script will build a Python virtualenv in `.build/virtualenv/`
and install [`dent`] in it (along with anything else in
`requirements.txt`). This is useful for entering the containers created by
`Test`: use `.build/virtalenv/bin/dent hra-bujild` or similar. (This
basically runs something along the lines of, `docker exec -i
--detach-keys=ctrl-@,ctrl-d -t hra-build bash -l`.)

(The persistent containers `Test` creates are very similar to those that
`dent` creates; we do not use `dent` for this because we have some further
special configuration that's easier to do with a custom `Dockerfile`.)



<!-------------------------------------------------------------------->
[`dent`]: https://pypi.org/project/dent/
[bld-debian]: https://github.com/0cjs/sedoc/blob/master/os/linux/distro/bld-debian.md
