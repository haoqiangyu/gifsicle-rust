# [Rust][r] wrapper for [gifsicle][g]

This crate compiles gifsicle's [C codebase][c] and exposes it as an unsafe Rust library.

It can be used for [lossy GIF compression][l], e.g. see [gifski](https://gif.ski).

[r]: https://www.rust-lang.org/
[g]: https://www.lcdf.org/gifsicle/
[c]: https://github.com/kohler/gifsicle
[l]: https://kornel.ski/lossygif
