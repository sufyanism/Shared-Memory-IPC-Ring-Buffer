use libc::{fork, wait};

fn main() {
    unsafe {
        let pid = fork();

        if pid == 0 {
            println!("Child process (consumer) started");
        } else {
            println!("Parent process (producer) started");
            wait(std::ptr::null_mut());
        }
    }
}
