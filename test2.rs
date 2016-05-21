use std::thread;
use std::process;

pub struct PanicGuard;

impl PanicGuard {
    pub fn new() -> PanicGuard {
        print!("new PanicGuard\n");
        PanicGuard
    }
}

impl Drop for PanicGuard {
    fn drop(&mut self) {
        print!("omg?\n");
        if thread::panicking() {
            print!("omg!\n");
            process::exit(101);
        }
    }
}

pub fn panicking(x: i32) -> i32 {
    print!("entered\n");
    let _guard = PanicGuard::new();
    print!("pre-panic\n");
    panic!("panic!");
    return x;
}

pub fn main() {
    print!("entered main\n");
    print!("panicking? {}\n", panicking(123));
}
