# 说明
- 原始项目地址为 https://gitlab.com/kornelski/gifsicle-rust
- 修改了vendor也就是gifsicle源码的git指向，参考 https://github.com/ImageOptim/ImageOptim/tree/main/gifsicle 
- 目的是跟随ImageOptim的压缩效果，上述源码位置的代码同样质量下可以将gif压缩得更小

# [Rust][r] wrapper for [gifsicle][g]

This crate compiles gifsicle's [C codebase][c] and exposes it as an unsafe Rust library.

It can be used for [lossy GIF compression][l], e.g. see [gifski](https://gif.ski).

[r]: https://www.rust-lang.org/
[g]: https://www.lcdf.org/gifsicle/
[c]: https://github.com/kohler/gifsicle
[l]: https://kornel.ski/lossygif
