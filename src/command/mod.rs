pub mod echo;

pub const NEWLINE: &str = "\n";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_lines() {
        let count = 3 as usize;
        let mut buf = String::new();

        for _ in 0..count {
            buf.push_str(NEWLINE);
        }

        assert_eq!(count, buf.lines().count())
    }
}
