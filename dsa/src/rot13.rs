use crate::utils;

pub fn rot13(plain_text: &str) -> String {
    let mut encoded = String::new();

    for ch in plain_text.chars() {
        let conv = if ch.is_uppercase() {
            utils::shift(ch, 13_u8, 'A')
        } else if ch.is_lowercase() {
            utils::shift(ch, 13_u8, 'a')
        } else {
            ch
        };

        encoded.push(conv)
    }

    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot13() {
        let mut testcases: Vec<(&'static str, &'static str)> = Vec::new();

        testcases.push(("abcdefghi", "nopqrstuv"));
        testcases.push(("ABCDEFGHIJKLM", "NOPQRSTUVWXYZ"));

        for (plain, want) in testcases {
            let got = rot13(plain);
            assert_eq!(got, want);
        }
    }
}
