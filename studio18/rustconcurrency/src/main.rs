// use std::io::BufRead;
use std::io::Read;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() -> std::io::Result<()> {
    static IS_DONE: AtomicBool = AtomicBool::new(false);
    // let handler = std::thread::spawn(|| {
    //     println!("Hi, world!");
    // });

    // println!("Hello, world!");
    // let _res = handler.join();

    // let mut threads = Vec::new();
    let mut args: Vec<String> = std::env::args().collect();
    if !args.is_empty() {
        args.remove(0);
    }

    // for arg in args {
    //     threads.push(std::thread::spawn(move || {
    //         println!("file name: {:?}", arg);

    //         if let Ok(file) = std::fs::File::open(arg) {
    //             let reader = std::io::BufReader::new(file);
                
    //             for line_result in reader.lines() {
    //                 match line_result {
    //                     Ok(line) => {
    //                         if !line.trim().is_empty() {
    //                             println!("{:?}", line);
    //                         }
    //                     }
    //                     Err(e) => {
    //                         eprintln!("Error reading line: {:?}", e);
    //                     }
    //                 }
    //             }
    //         }
    //     }));
    // }

    // for handler in threads {
    //     handler.join().unwrap();
    // }

    let listener = std::net::TcpListener::bind("127.0.0.1:8080")?;

    let client_handle = std::thread::spawn(move || {
        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("connection established at {:?}", addr);
                    let mut buf = String::new();
                    match stream.read_to_string(&mut buf) {
                        Ok(_) => {
                            println!("{:?}", &buf);
                        }
                        Err(e) => {
                            eprintln!("error reading {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
            if IS_DONE.load(Ordering::Relaxed) == true {
                break;
            }
        }
    });

    let mut stream = std::net::TcpStream::connect("127.0.0.1:8080")?;
    for arg in args {
        stream.write_all(arg.as_bytes())?;
    }
    
    stream.shutdown(std::net::Shutdown::Both)?;
    IS_DONE.store(true, Ordering::Relaxed);
    let _ = std::net::TcpStream::connect("127.0.0.1:8080")?;

    client_handle.join().unwrap();
    Ok(())
}
