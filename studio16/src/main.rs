// studio16

fn normalize_char(c: char) -> char {
    match c {
        'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' | 'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => 'a',
        'Ç' | 'ç' => 'c',
        'È' | 'É' | 'Ê' | 'Ë' | 'è' | 'é' | 'ê' | 'ë' => 'e', // Covers all È to ë cases
        'Ì' | 'Í' | 'Î' | 'Ï' | 'ì' | 'í' | 'î' | 'ï' => 'i',
        'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' | 'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' => 'o',
        'Ù' | 'Ú' | 'Û' | 'Ü' | 'ù' | 'ú' | 'û' | 'ü' => 'u',
        'Ý' | 'ý' | 'ÿ' => 'y',
        _ => c, // Return the original character for all others
    }
}

// Function to check if a string slice is a palindrome
fn is_palindrome(s: &str) -> bool {
    // Filter out non-alphanumeric characters, normalize, convert to lowercase, and collect the original string
    let original: String = s
        .chars()
        .map(normalize_char) // Normalize accented characters
        .filter(|c| c.is_ascii_alphanumeric()) // Keep only alphanumeric characters
        .map(|c| c.to_ascii_lowercase()) // Convert to lowercase
        .collect();

    // Filter out non-alphanumeric characters, normalize, reverse, convert to lowercase, and collect the reversed string
    let reversed: String = s
        .chars()
        .map(normalize_char) // Normalize accented characters
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .rev()
        .collect();

    original == reversed  // Compare for equality
}

fn main() {
    // Declare the string slice
    let my_string: &str = "Hello, world!";

    // Count uppercase ASCII letters
    let uppercase_count = my_string
        .chars()
        .filter(|c| c.is_ascii_uppercase())
        .count();

    // Count lowercase ASCII letters
    let lowercase_count = my_string
        .chars()
        .filter(|c| c.is_ascii_lowercase())
        .count();

    // Compute other characters
    let other_count = my_string.len() - (uppercase_count + lowercase_count);

    // Reverse the original string slice
    let reversed_string: String = my_string.chars().rev().collect();

    // Print all values
    println!("String slice: {}", my_string);
    println!("Uppercase letters: {}", uppercase_count);
    println!("Lowercase letters: {}", lowercase_count);
    println!("Other characters: {}", other_count);
    println!("Reversed string: {}", reversed_string);

    // Test strings for palindrome check
    let test_strings = [
        "kayak",
        "administration",
        "racecar",
        "hello",
        "madam",
        "a7 6b b67a",
        "Madam, I'm Adam.",
        "Eh! Ça va, la vache?",
        "Ésope reste ici et se repose.",
    ];

    // Loop through each test string and check if it's a palindrome
    for &test_string in &test_strings {
        let is_palindrome_result = is_palindrome(test_string);  // Check palindrome
        println!("Is \"{}\" a palindrome? {}", test_string, is_palindrome_result);
    }
}