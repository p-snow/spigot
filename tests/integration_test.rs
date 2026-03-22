use std::os::unix::net::UnixStream;
use std::io::{Write, Read};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[test]
fn test_socket_server_responds_to_valid_request() {
    // Skip this test on CI environments since we need a real socket
    // This test is meant to run when the server can actually be started
    let _ = std::fs::remove_file("/tmp/test-socket.sock");

    // Start the server in background
    let mut server_process = Command::new("cargo")
        .args(["run", "--bin", "socket-http-server"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start server");

    // Give the server time to start
    thread::sleep(Duration::from_millis(100));

    // Try to connect to socket
    let mut stream = UnixStream::connect("/tmp/machine-info.sock").expect("Failed to connect to socket");

    // Send a valid HTTP request
    let request = "GET /info/fs/avail?file=/ HTTP/1.1\r\nHost: localhost\r\n\r\n";
    stream.write_all(request.as_bytes()).expect("Failed to send request");

    // Read the response
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).expect("Failed to read response");

    // Verify we got a valid response (it will be empty since df command runs in test env)
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    assert!(response.starts_with("HTTP/1.1 200 OK"));

    // Kill the server
    server_process.kill().expect("Failed to kill server");
}
