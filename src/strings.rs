pub fn normalized_list(data: String) -> Vec<String> {
    let mut strings: Vec<String> = Vec::new();

    for word in data.split(",") {
        let normalized = word.get(1..word.len() - 1).unwrap();
        strings.push(String::from(normalized));
    }

    return strings;
}

pub fn word_value(str: &str) -> usize {
    let mut word_value: usize = 0;
    for c in str.chars() {
        let alphabet_index = c as usize - 'A' as usize;
        word_value += alphabet_index + 1;
    }

    return word_value;
}