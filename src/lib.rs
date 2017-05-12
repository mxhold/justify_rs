extern crate rand;
use rand::{sample, weak_rng, Rng, XorShiftRng, SeedableRng};
// pub fn justify(input: &str, width: usize) -> Vec<String> {
//     let mut line_words: Vec<String> = Vec::new();
//     let mut lines: Vec<String> = Vec::new();
//
//     for word in input.split_whitespace() {
//         let line_length: usize = line_words.iter().map(|word| word.len() + 1).sum();
//         if word.len() + line_length <= width {
//             println!("push: {}", word);
//             line_words.push(word.to_string());
//         } else {
//             lines.push(justify_line(line_words, width));
//             line_words = Vec::new();
//         }
//     }
//
//     lines.push(justify_line(line_words, width));
//
//     lines
// }

fn get_rng(seed: Option<[u32; 4]>) -> XorShiftRng {
    match seed {
        Some(seed) => SeedableRng::from_seed(seed),
        None => weak_rng(),
    }
}

pub fn justify_line(line: &str, desired_length: usize, seed: Option<[u32; 4]>) -> String {
    let words: Vec<&str> = line.split_whitespace().collect();

    let word_length: usize = words.iter().map(|word| word.len()).sum();
    let spaces_to_add = desired_length - word_length;

    let word_count: usize = words.len();
    let space_position_count = word_count - 1;

    let spaces_for_each_space_position = spaces_to_add / space_position_count;
    let extra_spaces = spaces_to_add % space_position_count;

    let space_positions = 0..space_position_count;
    let mut rng = get_rng(seed);
    let positions_for_extra_spaces = sample(&mut rng, space_positions, extra_spaces);

    let mut result = String::new();

    let (last_word, all_but_last_word) = words.split_last().unwrap();

    for (index, word) in all_but_last_word.iter().enumerate() {
        result.push_str(word);
        result.push_str(&" ".repeat(spaces_for_each_space_position));
        if positions_for_extra_spaces.contains(&index) {
            result.push_str(" ");
        }
    }

    result.push_str(last_word);

    result
}


#[cfg(test)]
mod tests {
    use justify_line;

    // #[test]
    // fn it_adds_spaces_to_fit_width() {
    //     let input = "123 456";
    //     let result = justify_line(input, 8);
    //
    //     assert_eq!(result, "123  456");
    // }

    #[test]
    fn it_adds_extra_spaces_randomly() {
        let input = "123 456 789";
        let result = justify_line(input, 12, Some([1, 2, 3, 4]));

        assert_eq!(result, "123  456 789");

        let input = "123 456 789";
        let result = justify_line(input, 12, Some([2, 2, 3, 4]));

        assert_eq!(result, "123 456  789");
    }
}
