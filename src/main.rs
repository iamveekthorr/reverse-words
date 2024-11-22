fn main() {
    // Given an input string s, reverse the order of the words.

    // A word is defined as a sequence of non-space characters. The words in s will be separated by at least one space.

    // Return a string of the words in reverse order concatenated by a single space.

    // Note that s may contain leading or trailing spaces or multiple spaces between two words. The returned string should only have a single space separating the words. Do not include any extra spaces.

    let s = String::from("  hello world  ");
    reverse_words(s);
}

fn reverse_words(s: String) -> String {
    // Step 1: Convert the string into a mutable character array
    let mut chars: Vec<char> = s.trim().chars().collect();

    let len = chars.len();
    // Step 2: Reverse the entire array
    reverse(&mut chars, 0, len - 1);

    // Step 3: Reverse each word within the reversed array
    let mut start = 0;
    for end in 0..=chars.len() {
        if end == chars.len() || chars[end] == ' ' {
            reverse(&mut chars, start, end - 1);
            start = end + 1;
        }
    }

    // Step 4: Remove extra spaces and join characters back to a string
    let mut cleaned = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        // Skip spaces if there are multiple
        if chars[i] == ' ' && (i == 0 || chars[i - 1] == ' ') {
            i += 1;
            continue;
        }
        cleaned.push(chars[i]);
        i += 1;
    }

    // Convert cleaned character vector back to a string
    cleaned.into_iter().collect()
}

fn reverse(chars: &mut Vec<char>, mut left: usize, mut right: usize) {
    while left < right {
        chars.swap(left, right);
        left += 1;
        right -= 1;
    }
}
