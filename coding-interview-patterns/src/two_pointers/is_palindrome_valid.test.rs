mod two_pointers {
    pub mod is_palindrome_valid;
}

use two_pointers::is_palindrome_valid::is_palindrome_valid;

fn main() {
    let nums = "No 'x' in Nixon";
    let result = is_palindrome_valid(nums);
    println!("Result: is true {:?}", result);

    assert_eq!(is_palindrome_valid("a dog! a panic in a pagoda."), true); // classic palindrome with punctuation
    assert_eq!(is_palindrome_valid("abc123"), false); // clearly not a palindrome
    assert_eq!(is_palindrome_valid(""), true); // empty string is trivially a palindrome
    assert_eq!(is_palindrome_valid("a"), true); // single character is a palindrome
    assert_eq!(is_palindrome_valid("9"), true); // digit as a single character

    assert_eq!(is_palindrome_valid("!!!@@@###"), true); // after removing all, it's empty

    assert_eq!(is_palindrome_valid("1a2!@#a1"), true); // cleaned: "1a2a1"
    assert_eq!(is_palindrome_valid("No 'x' in Nixon"), true); // cleaned: "noxinnixon"

    assert_eq!(is_palindrome_valid("race a car"), false);
    assert_eq!(is_palindrome_valid("Palindrome"), false);

    assert_eq!(is_palindrome_valid("Eva, can I see bees in a cave?"), true);
    println!("Done")
}
