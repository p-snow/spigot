use std::os::unix::net::UnixListener;
use std::os::unix::net::UnixStream;
use std::thread;
use std::fs;
use std::io::Read;
use std::io::Write;

static DOMAIN_SOCK: &str = "/run/socket-http-server/machine-info.sock";

fn main() -> std::io::Result<()> {
    // remove existing socket up-front
    if fs::metadata(&DOMAIN_SOCK).is_ok() {
        if let Err(_err) = fs::remove_file(&DOMAIN_SOCK) {
            panic!("Error removing file: {}", _err);
        }
    }

    let listener = UnixListener::bind(DOMAIN_SOCK)?;

    println!("Socket HTTP server listening on {}", DOMAIN_SOCK);

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // connection succeeded
                println!("Got a client");
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                // connection failed
                eprintln!("Error listening: {}", err);
                break;
            }
        }
    }
    Ok(())
}

fn handle_client(mut stream: UnixStream) -> std::io::Result<()> {
    // Read data from the client
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read == 0 {
        eprintln!("Client disconnected");
        return Ok(());
    }

    let received = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Received request: {}", received);

    let message = socket_http_server::execute_command(&received);

    // Send HTTP response
    stream.write_all(b"HTTP/1.1 200 OK\r\n")?;
    stream.write_all(b"Content-Type: text/plain\r\n")?;

    match message {
        Some(s) => {
            let content = format!("{}", s);
            stream.write_all(format!("Content-Length: {}\r\n", content.len()).as_bytes())?;
            stream.write_all(b"\r\n")?;
            stream.write_all(content.as_bytes())?;
        }
        None => {
            stream.write_all(b"Content-Length: 0\r\n")?;
            stream.write_all(b"\r\n")?;
            eprintln!("HTTP response issued with zero length");
        }
    }

    Ok(())
}
