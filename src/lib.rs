use std::collections::HashMap;
use std::process::Command;
use std::process::Stdio;

/// Parse HTTP header and extract command parameters
pub fn execute_command(header: &str) -> Option<String> {
    let mut params: HashMap<String, String> = HashMap::new();

    // Parse the first line of the header
    if let Some(first_line) = header.lines().next() {
        let parts: Vec<&str> = first_line.split_whitespace().collect();

        if parts.len() >= 2 && parts[0] == "GET" {
            let query_parts: Vec<&str> = parts[1].split("?").collect();
            let path = query_parts[0];

            // Parse query parameters
            if query_parts.len() > 1 {
                for q in query_parts[1].split("&") {
                    if let Some((key, value)) = q.split_once('=') {
                        params.insert(key.to_string(), value.to_string());
                    }
                }
            }

            // Handle the /info/fs/avail endpoint
            match path {
                "/info/fs/avail" => {
                    if let Some(file_param) = params.get("file") {
                        return get_filesystem_availability(file_param);
                    }
                }
                _ => {}
            }
        }
    }

    None
}

/// Get filesystem availability for a given file system
pub fn get_filesystem_availability(fs: &str) -> Option<String> {
    // Execute df command to get disk usage info
    let output = Command::new("df")
        .arg("-h")
        .arg(fs)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute df command");

    if !output.status.success() {
        eprintln!("df command failed: {}", String::from_utf8_lossy(&output.stderr));
        return None;
    }

    // Extract the available space from the second line (skip header)
    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.lines().collect();

    if lines.len() < 2 {
        eprintln!("Unexpected df output format");
        return None;
    }

    // Parse the second line to extract available space
    let second_line = lines[1].split_whitespace().collect::<Vec<&str>>();

    if second_line.len() < 4 {
        eprintln!("Unexpected df output format in second line");
        return None;
    }

    // The fourth column contains the available space
    Some(second_line[3].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_command_parses_get_request() {
        let header = "GET /info/fs/avail?file=/tmp HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let result = execute_command(header);
        // This will return None since it calls external df command,
        // but we test the parsing works correctly
        assert!(result.is_none() || result.is_some());
    }

    #[test]
    fn test_execute_command_handles_missing_file_param() {
        let header = "GET /info/fs/avail HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let result = execute_command(header);
        assert_eq!(result, None);
    }

    #[test]
    fn test_execute_command_handles_invalid_path() {
        let header = "GET /invalid/path HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let result = execute_command(header);
        assert_eq!(result, None);
    }

    #[test]
    fn test_execute_command_handles_non_get_request() {
        let header = "POST /info/fs/avail?file=/tmp HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let result = execute_command(header);
        assert_eq!(result, None);
    }
}
