pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn caesar(input: &str, shift: i32) -> String {
    let mut result = String::new();
    let alphabet_len = ALPHABET.len() as i32;

    let shift = ((shift % alphabet_len) + alphabet_len)% alphabet_len;

    for ch in input.chars() {
        if ch.is_ascii_lowercase() {
            let pos = ch as u8 - b'a';
            let new_pos = ( pos as i32 + shift) % alphabet_len;
            result.push((b'a' + new_pos as u8) as char);
        } else if ch.is_ascii_uppercase() {
            let pos = ch as u8 - b'A';
            let new_pos = (pos as i32 + shift) % alphabet_len;
            result.push((b'A' + new_pos as u8) as char);
        } else {
            result.push(ch);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_shift_three() {
        assert_eq!(caesar("Hello, World!", 3), "Khoor, Zruog!");
    }

    #[test]
    fn shift_minus_one() {
        assert_eq!(caesar("abc", -1), "zab");
    }

    #[test]
    fn shift_twenty_seven_wraps_to_one() {
        assert_eq!(caesar("xyz", 27), "yza");
    }

    #[test]
    fn empty_input() {
        assert_eq!(caesar("", 5), "");
    }

    #[test]
    fn shift_zero_is_identity() {
        assert_eq!(caesar("Rust!", 0), "Rust!");
    }

    #[test]
    fn shift_twenty_six_is_identity() {
        assert_eq!(caesar("abc", 26), "abc");
    }

    #[test]
    fn non_letters_preserved() {
        assert_eq!(caesar("1 2 3 !", 5), "1 2 3 !");
    }

    #[test]
    fn large_negative_shift_wraps() {
        assert_eq!(caesar("abc", -27), "zab");
    }
}
