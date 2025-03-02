hello-rust-apt
==============

- `webhello/`: Source for Rust "web Hello" program, a web server that
  replies to `GET /hello`.
- `docker/`: Docker configurations for build and test containers
- `.build/docker/build`: Build of webhello.
- `.build/docker/test`: Test of apt-packaged version of webhello.

XXX Explain why we use dent (easy experimentation inside container)

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
