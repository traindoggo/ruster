use crate::utils;

pub fn caesar(plain_text: &str, rot: i32) -> String {
    let mut encoded = String::new();
    let rot = (rot % 26 + 26) as u8;

    for ch in plain_text.chars() {
        let conv = if ch.is_uppercase() {
            utils::shift(ch, rot, 'A')
        } else if ch.is_lowercase() {
            utils::shift(ch, rot, 'a')
        } else {
            ch
        };

        encoded.push(conv);
    }

    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_1() {
        let mut testcases: Vec<(&'static str, &'static str)> = Vec::new();

        testcases.push(("a", "b"));
        testcases.push(("z", "a"));
        testcases.push(("A", "B"));
        testcases.push(("Z", "A"));
        testcases.push(("ZZZZ", "AAAA"));
        testcases.push(("zyxw", "azyx"));

        for (plain, want) in testcases {
            let got = caesar(plain, 1);
            assert_eq!(got, want);
        }
    }

    #[test]
    fn test_caesar_26() {
        let mut testcases: Vec<(&'static str, &'static str)> = Vec::new();

        testcases.push(("a", "a"));
        testcases.push(("ZWSX", "ZWSX"));
        testcases.push(("aAaA", "aAaA"));

        for (plain, want) in testcases {
            let got = caesar(plain, 26);
            assert_eq!(got, want);
        }
    }

    #[test]
    fn test_caesar_neg_1() {
        let mut testcases: Vec<(&'static str, &'static str)> = Vec::new();

        testcases.push(("a", "z"));
        testcases.push(("A", "Z"));
        testcases.push(("AaAa", "ZzZz"));
        testcases.push(("Hello World", "Gdkkn Vnqkc"));

        for (plain, want) in testcases {
            let got = caesar(plain, -1);
            assert_eq!(got, want);
        }
    }

    #[test]
    fn test_caesar_neg_25() {
        let testcases: Vec<&str> = vec!["hello", "world"];

        for plain in testcases {
            assert_eq!(caesar(plain, 1), caesar(plain, -25));
            assert_eq!(caesar(plain, 0), caesar(plain, -26));
        }
    }
}
