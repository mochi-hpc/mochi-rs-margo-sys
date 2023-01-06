# margo-sys

This Rust library provides a FFI bindings to the
[mochi-margo](https://github.com/mochi-hpc/mochi-margo) library.

## Requirements

This crate requires margo (see instructions [here](https://mochi.readthedocs.io/en/latest/installing.html))
and clang (see instructions [here](https://rust-lang.github.io/rust-bindgen/requirements.html)).
Margo must be loaded (this crate uses pkg-config to locate its library and headers).

For example, on Debian and with [Spack](https://spack.io/), the following can be used.

```console
$ spack install mochi-margo
$ spack load mochi-margo
$ sudo apt install llvm-dev libclang-dev clang
$ git clone https://github.com/mochi-hpc/mochi-rs-margo-sys.git
$ cd mochi-rs-margo-sys
$ cargo build
```
