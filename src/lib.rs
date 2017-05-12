pub fn justify(input: &str, width: usize) -> Vec<String> {
    let mut line_words: Vec<String> = Vec::new();
    let mut lines: Vec<String> = Vec::new();

    for word in input.split_whitespace() {
        let line_length: usize = line_words.iter().map(|word| word.len() + 1).sum();
        if word.len() + line_length <= width {
            println!("push: {}", word);
            line_words.push(word.to_string());
        } else {
            lines.push(justify_line(line_words, width));
            line_words = Vec::new();
        }
    }

    lines.push(justify_line(line_words, width));

    lines
}

fn justify_line(words: Vec<String>, width: usize) -> String {
    let len_without_spaces: usize = words.iter().map(|word| word.len()).sum();

    // assert!(len_without_spaces <= width);

    let spaces_to_add: usize = width - len_without_spaces;
    // spaces can be added between each word
    let space_positions = (0..(words.len() - 2)).collect::Vec<usize>();

}


#[cfg(test)]
mod tests {
    use justify;

    #[test]
    fn it_adds_spaces_to_fit_width() {
        let input = "Foo foo";
        let result = justify(input, 8);

        assert_eq!(result, vec!["Foo  foo".to_string()]);
    }
}
