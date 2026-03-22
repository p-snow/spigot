use std::fs;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;

const DOMAIN_SOCKET_PATH: &str = "/tmp/spigot.sock";

fn main() -> std::io::Result<()> {
    // Remove existing socket if it already exists
    if fs::metadata(DOMAIN_SOCKET_PATH).is_ok() {
        if let Err(remove_err) = fs::remove_file(DOMAIN_SOCKET_PATH) {
            panic!("Failed to remove existing socket: {remove_err}");
        }
    }

    let listener = UnixListener::bind(DOMAIN_SOCKET_PATH)?;

    println!("Socket HTTP server listening on {DOMAIN_SOCKET_PATH}");

    // Accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Got a client connection");
                thread::spawn(|| handle_client(stream));
            },
            Err(error) => {
                eprintln!("Error listening: {error}");
                break;
            },
        }
    }

    Ok(())
}

fn handle_client(mut stream: UnixStream) -> std::io::Result<()> {
    // Read request from client
    let mut buffer = [0u8; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    if bytes_read == 0 {
        eprintln!("Client disconnected without sending data");
        return Ok(());
    }

    let received_request = std::str::from_utf8(&buffer[..bytes_read])
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    println!("Received request:\n{received_request}");

    use spigot::execute_command;
    let response = execute_command(received_request);

    // Prepare HTTP response
    stream.write_all(b"HTTP/1.1 200 OK\r\n")?;
    stream.write_all(b"Content-Type: text/plain\r\n")?;

    match response {
        Some(response_body) => {
            let content_length = response_body.len();
            stream.write_all(format!("Content-Length: {content_length}\r\n").as_bytes())?;
            stream.write_all(b"\r\n")?;
            stream.write_all(response_body.as_bytes())?;
        },
        None => {
            stream.write_all(b"Content-Length: 0\r\n")?;
            stream.write_all(b"\r\n")?;
            eprintln!("Warning: HTTP response issued with zero-length body");
        },
    }

    Ok(())
}
