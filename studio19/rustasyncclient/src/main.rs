use async_std::net::TcpStream;
use async_std::io::WriteExt;

fn main() {
    match async_std::task::block_on(TcpStream::connect("127.0.0.1:8080")) {
        Ok(mut stream) => {
            match async_std::task::block_on(stream.write_all("connection established!".as_bytes())) {
                Ok(_) => {
                    println!("successfully wrote to server");
                }
                Err(e) => {
                    eprintln!("error writing to server {:?}", e);
                }
            }
            
        }
        Err(e) => {
            eprintln!("error connecting to server: {:?}", e);
        }
    }
}
