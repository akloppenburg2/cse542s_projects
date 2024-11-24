use async_std::net::TcpListener;
use async_std::prelude::*;


async fn run_server() -> std::io::Result<String> {
    match TcpListener::bind("127.0.0.1:8080").await {
        Ok(listener) => {
            println!("binding succeeded");
            let mut connection = listener.incoming();
            let mut buf = [0u8; 100];
            while let Some(stream) = connection.next().await {
                match stream?.read(&mut buf).await {
                    Ok(read) => {
                        println!("received {:?} bytes: {:?}", read, std::str::from_utf8(&buf[..read]).unwrap());
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Ok("binding succeeded".to_string())
        }
        Err(e) => {
            Err(e)
        }
    }
}

fn main() -> std::io::Result<()> {
    let bind_future = run_server();

    match async_std::task::block_on(bind_future) {
        Ok(mesg) => {
            println!("{:?}", mesg);
        }
        Err(_) => {
            println!("binding failed");
        }
    }
    

    Ok(())
}
