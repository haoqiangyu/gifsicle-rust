use std::ffi::CString;

fn main() {
    let args: Vec<_> = std::env::args().map(|arg| CString::new(arg).unwrap()).collect();
    let argv: Vec<_> = args.iter().map(|a| a.as_ptr()).collect();

    unsafe {
        gifsicle::gifsicle_main(argv.len() as _, argv.as_ptr());
    }
}
