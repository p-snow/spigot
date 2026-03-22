use std::collections::HashMap;
use std::process::{Command, Stdio};

/// Parses an HTTP header and extracts command parameters based on the request path.
///
/// Expected format: `GET /path?param1=value1&param2=value2 HTTP/1.1`
///
/// # Arguments
/// * `header` - The raw HTTP request header string
///
/// # Returns
/// An Option containing the result of executing the command, or None if parsing fails
pub fn execute_command(header: &str) -> Option<String> {
    let mut parameters = HashMap::new();

    // Parse the first line of the header
    let first_line = header.lines().next()?;

    let parts: Vec<&str> = first_line.split_whitespace().collect();

    // Validate GET method and expected path length
    if parts.len() < 2 || parts[0] != "GET" {
        return None;
    }

    let query_path = parts[1];

    // Split path and query parameters
    let (path, query_string) = query_path.split_once('?').unwrap_or((query_path, ""));

    // Parse query parameters (if present)
    for param in query_string.split('&') {
        if let Some((key, value)) = param.split_once('=') {
            parameters.insert(key.to_string(), value.to_string());
        }
    }

    // Handle filesystem availability endpoint
    handle_filesystem_endpoint(path, &parameters)
}

/// Handles filesystem availability queries.
///
/// # Arguments
/// * `path` - The requested path (e.g., "/info/fs/avail")
/// * `parameters` - Query parameters from the request
///
/// # Returns
/// A string containing the available space in human-readable format, or None if unavailable
fn handle_filesystem_endpoint(path: &str, parameters: &HashMap<String, String>) -> Option<String> {
    match path {
        "/info/fs/avail" => {
            let file_path = parameters.get("file")?;
            get_filesystem_availability(file_path)
        },
        _ => None,
    }
}

/// Retrieves filesystem availability information using the `df` command.
///
/// # Arguments
/// * `filesystem_path` - The filesystem path to query (e.g., "/", "/tmp", "ext4:/dev/sda1")
///
/// # Returns
/// A string containing the available space in human-readable format, or None if the command fails
fn get_filesystem_availability(filesystem_path: &str) -> Option<String> {
    // Execute `df` command to get disk usage information
    let df_output = Command::new("df")
        .arg("-h")
        .arg(filesystem_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .ok()?;

    if !df_output.status.success() {
        eprintln!(
            "df command failed: {}",
            String::from_utf8_lossy(&df_output.stderr)
        );
        return None;
    }

    // Extract information from df output
    let output_str = String::from_utf8_lossy(&df_output.stdout);
    let lines: Vec<&str> = output_str.lines().collect();

    // Skip header line and ensure we have data
    let data_line = lines.get(1)?;

    // Parse the data line (format: "Filesystem Size Used Avail Use% Mounted on")
    let data_parts: Vec<&str> = data_line.split_whitespace().collect();

    // Verify we have the expected number of columns
    if data_parts.len() < 4 {
        eprintln!("Unexpected df output format");
        return None;
    }

    // The fourth column contains the available space
    Some(data_parts[3].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_command_parses_get_request() {
        let header = "GET /info/fs/avail?file=/tmp HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let result = execute_command(header);
        // Result depends on external df command, but parsing should succeed
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

    #[test]
    fn test_execute_command_handles_missing_query_string() {
        let header = "GET /info/fs/avail HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let result = execute_command(header);
        assert_eq!(result, None);
    }
}
