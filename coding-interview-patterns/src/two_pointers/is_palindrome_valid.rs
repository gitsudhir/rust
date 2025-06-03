pub fn is_palindrome_valid(s: &str) -> bool {
    // edge case
    if s.len() == 0 || s.len() == 1 {
        return true;
    }

    let chars: Vec<_> = s.chars().collect();
    let mut first_pointer = 0;
    let mut last_pointer = s.len() - 1;
    while first_pointer < last_pointer {
        // check the char is alphabet or number
        let first_char = chars[first_pointer];
        if !first_char.is_alphanumeric() {
            first_pointer += 1;
            continue;
        }
        let last_char = chars[last_pointer];
        if !last_char.is_alphanumeric() {
            last_pointer -= 1;
            continue;
        }

        // if the char of first pointer and last pointer is not equal then its false
        if first_char.to_ascii_lowercase() != last_char.to_ascii_lowercase() {
            return false;
        }

        // shrink the pointers
        first_pointer += 1;
        last_pointer -= 1;
    }
    true
}
