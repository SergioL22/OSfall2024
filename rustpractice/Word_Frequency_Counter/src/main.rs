fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut max_word = String::new();
    let mut max_count = 0;

    for i in 0..words.len() {
        let mut count = 1; // Start with count 1 for the current word
        for j in i+1..words.len() {
            if words[i] == words[j] {
                count += 1;
            }
        }

        // Update the max_word and max_count if the current word has a higher count
        if count > max_count {
            max_word = words[i].to_string();
            max_count = count;
        }
    }

    (max_word, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}