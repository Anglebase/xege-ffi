# xege-ffi

This crate provides Rust FFI bindings for the C++ graphics library `xege`. It enables users to call `xege`'s functionalities within Rust. `xege` is a beginner-oriented C++ graphics library offering features like graphics drawing, event handling, and other functionalities. It is a widely used third-party library for programming language and computer graphics learning.

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