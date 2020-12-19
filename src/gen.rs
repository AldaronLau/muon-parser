/// Get the indentation level in a MuON file.
///
/// Always returns 2, 3 or 4
fn indentation(data: &str) -> Option<u8> {
    for line in data.lines() {
        if !line.starts_with("  ") {
            continue;
        }
        if !line.get[2..].starts_with(" ") {
            return Some(2);
        } else if !line.get[3..].starts_with(" ") {
            return Some(3);
        } else {
            return Some(4);
        }
    }
    None
}
