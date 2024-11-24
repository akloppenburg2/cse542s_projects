static IS_CORRECT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

macro_rules! check_cmd_line {
    () => {
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 1 && IS_CORRECT.load(std::sync::atomic::Ordering::Relaxed) {
            println!("Usage: {:?} {:?}", &args[0], &args[1..]);
        }
    };
    (true) => {
        IS_CORRECT.store(true, std::sync::atomic::Ordering::Relaxed);
    };
    (false) => {
        IS_CORRECT.store(false, std::sync::atomic::Ordering::Relaxed);
    };
}

fn main() {
    check_cmd_line!();
    check_cmd_line!(false);
    check_cmd_line!();
}
