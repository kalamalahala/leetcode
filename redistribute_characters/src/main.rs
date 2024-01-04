fn main() {
    let words = vec![
        "caaaaa".to_string(),
        "aaaaaaaaa".to_string(),
        "a".to_string(),
        "bbb".to_string(),
        "bbbbbbbbb".to_string(),
        "bbb".to_string(),
        "cc".to_string(),
        "cccccccccccc".to_string(),
        "ccccccc".to_string(),
        "ccccccc".to_string(),
        "cc".to_string(),
        "cccc".to_string(),
        "c".to_string(),
        "cccccccc".to_string(),
        "c".to_string()
    ];
    println!("Can redistribute? {}", redistribute_possible(words));
}


// you are given an array of strings 'words'
// in one operation, pick two distinct indices i and j, where words[i] is a non-empty string
// and move any character from words[i] to any position in words[j]
// return true if you can make every string in words equal using any number of operations
// otherwise, return false

fn redistribute_possible(words: Vec<String>) -> bool {
    if words.is_empty() {
        return true;
    }

    let word_list_ref = &words;
    
    // if there is only one word, it is already equal to itself
    let word_count = words.len();
    if word_count == 1 {
        return true;
    }
    
    // count the number of characters in the words
    let mut char_count = 0;
    for word in &words {
        char_count += word.len();
    }

    // if the number of characters is not divisible by the number of words, it is not possible
    if char_count % word_count != 0 {
        return false;
    }

    // count each occurence of each character in the words
    let mut char_occurences = [0; 26];
    for word in word_list_ref {
        for character in word.chars() {
            let char_index = character as usize - 'a' as usize;
            char_occurences[char_index] += 1;
        }
    }

    if char_occurences.iter().any(|&count| (count > word_count && count % word_count != 0) || (count < word_count && count > 0)) {
        return false;
    }

    // if all characters occur the same number of times, it is possible
    true

}