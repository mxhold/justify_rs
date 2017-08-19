// extern crate rand;
// use rand::{sample, weak_rng, XorShiftRng, SeedableRng};
// fn get_rng(seed: Option<[u32; 4]>) -> XorShiftRng {
//     match seed {
//         Some(seed) => SeedableRng::from_seed(seed),
//         None => weak_rng(),
//     }
// }

pub fn justify(input: &str, desired_line_length: usize) -> String {
    let mut output = String::new();

    let mut line: Vec<String> = Vec::new();

    let mut word = String::new();

    // TODO: split by graphemes instead
    for character in input.chars() {
        if character == ' ' {
            let line_word_length: usize = line.iter().map(|w| w.len()).sum();
            let line_length_with_spaces = line_word_length + line.len().saturating_sub(1);
            let word_length = word.len();

            if line_length_with_spaces + word_length <= desired_line_length {
                println!("[1] line: {:?}, word: {:?}", line, word);
                line.push(word.clone());
                word.clear();
            } else {
                println!("[2] line: {:?}, word: {:?}", line, word);
                let mut spaces_to_add = desired_line_length - line_word_length;
                let space_positions = line.len() - 1;

                let spaces_for_each = spaces_to_add / space_positions;
                let mut remainder = spaces_to_add % space_positions;

                let mut joined_line = String::new();

                let final_word = line.pop().unwrap();
                for word in &line {
                    joined_line.push_str(&word);
                    for _ in 0..spaces_for_each {
                        joined_line.push(' ');
                    }
                    if remainder > 0 {
                        joined_line.push(' ');
                        remainder -= 1;
                    }
                }
                joined_line.push_str(&final_word);
                joined_line.push('\n');
                output.push_str(&joined_line);
                line.clear();
                line.push(word.clone());
                word.clear();
            }
        } else {
            word.push(character);
        }

    }

    println!("[3] line: {:?}, word: {:?}", line, word);
    line.push(word.clone());
    let mut joined_line = line.join(" ");
    joined_line.push('\n');
    output.push_str(&joined_line);

    output
}


#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod tests {
    use justify;

    #[test]
    fn it_justifies() {
        let output = justify("hello world my name is computer", 16);
        assert_eq!(output, "hello  world  my\nname is computer\n");
    }
}
