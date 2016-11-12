afio
---

[Documents](https://valarauca.github.io/afio/afio/index.html)

This crate is 2 functions that wrap the [fnctl](http://man7.org/linux/man-pages/man2/fcntl.2.html) function from C to modify File Descriptors to be non-blocking. It also offers bindings to [mio EventedFd](https://docs.rs/mio/0.6.1/mio/unix/struct.EventedFd.html) to make construction easier.

To use this crate, well don't be on Windows. OSX/MacOS, Linux, FreeBSD, OpenBSD, etc. are supported. Add the below to your `Cargo.toml` to use this crate.

     [dependencies]
     afio = "0.1"
