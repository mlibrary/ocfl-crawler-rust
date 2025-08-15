
/// Search for a pattern in a file and display the lines that contain it.


pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).expect("could not write line");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this tests fail")
    }
}
