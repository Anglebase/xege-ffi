use std::io::stdin;

fn main() {
    unsafe {
        xege_ffi::ege_initgraph(640, 480, 0);
        stdin().lines().next().unwrap().unwrap();
        xege_ffi::ege_closegraph();
    }
}
