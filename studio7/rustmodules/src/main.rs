
mod code {
    #[cfg(oldexercise)] pub const CONSTANT: usize = 3;
    #[cfg(oldexercise)] pub static CONSTANT: usize = 3;
    #[cfg(oldexercise)] pub static mut CONSTANT: usize = 3;

    use std::sync::atomic::AtomicUsize;

    pub static VAL: AtomicUsize = AtomicUsize::new(5);
}

fn main() {
    use std::sync::atomic::Ordering;
    use crate::code::VAL;

    #[cfg(oldexercise)]
    {
        unsafe {
            CONSTANT = 10;
            println!("constant: {}", CONSTANT);
        }
    }
    
    println!("atomic before store: {}", VAL.load(Ordering::SeqCst));
    VAL.store(10, Ordering::SeqCst);
    println!("atomic after store: {}", VAL.load(Ordering::SeqCst));
}
