// use std::io::Write;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("argument not provided");
        return;
    }

    println!("filename: {:?}", &args[1]);
    if let Ok(file) = std::fs::File::open(&args[1]) {
        let reader = std::io::BufReader::new(file);
        
        for line_result in reader.lines() {
            match line_result {
                Ok(line) => {
                    if !line.trim().is_empty() {
                        println!("{:?}", line);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading line: {:?}", e);
                }
            }
        }
    } else {
        eprintln!("error opening file")
    }

    // let res = writeln!(std::io::stdout().lock(), "Variables: {}, {}, {}", a, b, c);

    // match res {
    //     Ok(_) => { println!("success"); }
    //     Err(e) => { eprintln!("{:?}", e); }
    // }
}
