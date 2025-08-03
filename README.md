# xege-ffi

This crate provides Rust FFI bindings for the C++ graphics library `xege`. It enables users to call `xege`'s functionalities within Rust. `xege` is a beginner-oriented C++ graphics library offering features like graphics drawing, event handling, and other functionalities. It is a widely used third-party library for programming language and computer graphics learning.

# Why EGE?

EGE (Easy Graphics Engine) is a simple graphics library for Windows. It is similar to BGI (graphics.h) and is designed for beginners in C/C++ programming (Now, it can also be used in Rust). Its goal is to serve as a replacement for the BGI library from Turbo C (TC).

Its usage is very close to graphics.h from TC. For beginners, it is simple, friendly, and easy to learn. It is free, open-source, and features an intuitive interface. Even those with no prior graphics programming experience can quickly master basic drawing.

# Example

```rust
use std::io::stdin;

fn main() {
    unsafe {
        xege_ffi::ege_initgraph(640, 480, 0);
        stdin().lines().next().unwrap().unwrap();
        xege_ffi::ege_closegraph();
    }
}
```

# License

This project is licensed under the MIT license.