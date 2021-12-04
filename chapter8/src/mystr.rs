
// Assumes non-zero string length
pub fn pig_latin(string: &String) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let chr = string.chars().nth(0).unwrap();
    let mut word = string.clone();

    if vowels.contains(&chr) {
        // starts with vowel
        word.push_str("-hay");
    } else {
        // does not start with vowel
        word.remove(0);
        word.push_str(&format!("-{}{}", chr, "ay")[..]);
    }

    word
}