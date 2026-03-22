use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[test]
fn test_socket_server_responds_to_valid_request() {
    // Skip this test in CI environments since it requires a running socket
    // Intended to be run when the server can be started locally
    let _ = std::fs::remove_file("/tmp/test-socket.sock");

    // Start the server in the background
    let mut server_process = Command::new("cargo")
        .args(["run", "--bin", "spigot"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start server process");

    // Allow server time to initialize and create socket
    thread::sleep(Duration::from_secs(2));

    // Attempt to connect to the socket
    let mut stream = UnixStream::connect("/tmp/spigot.sock")
        .expect("Failed to connect to socket at /tmp/spigot.sock");

    // Send a valid HTTP GET request
    let request_payload = "GET /info/fs/avail?file=/ HTTP/1.1\r\nHost: localhost\r\n\r\n";
    stream
        .write_all(request_payload.as_bytes())
        .expect("Failed to send request");

    // Read the server's response
    let mut response_buffer = [0u8; 1024];
    let bytes_received = stream
        .read(&mut response_buffer)
        .expect("Failed to read server response");

    let response_text = String::from_utf8_lossy(&response_buffer[..bytes_received]);

    // Verify we received a successful HTTP response
    assert!(
        response_text.starts_with("HTTP/1.1 200 OK"),
        "Expected 200 OK response, got: {response_text}"
    );

    // Clean up: terminate the server process
    let _ = server_process.kill();
    let _ = server_process.wait();
}
