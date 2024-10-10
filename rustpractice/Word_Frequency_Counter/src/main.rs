fn find_most_frequent_word(text: &str, result_word: &mut String, result_count: &mut usize) {
    let words: Vec<&str> = text.split_whitespace().collect();
    
    for word in &words {
        let count = words.iter().filter(|&&w| w == *word).count(); 
        if count > *result_count {
            *result_count = count;
            *result_word = word.to_string();
        }
    }
}

fn main() {
    let input_text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    
    let mut most_frequent_word = String::new();
    let mut highest_count = 0;

    find_most_frequent_word(input_text, &mut most_frequent_word, &mut highest_count);

    println!("Most frequent word: '{}', Count: {}", most_frequent_word, highest_count);
}