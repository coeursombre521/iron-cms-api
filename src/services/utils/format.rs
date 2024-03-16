pub fn format_error_string(error_identifier: &str, custom_message: &str) -> String {
    format!("§§{}§§ - {}", error_identifier, custom_message)
}