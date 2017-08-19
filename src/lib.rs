// extern crate rand;
// use rand::{sample, weak_rng, XorShiftRng, SeedableRng};
// fn get_rng(seed: Option<[u32; 4]>) -> XorShiftRng {
//     match seed {
//         Some(seed) => SeedableRng::from_seed(seed),
//         None => weak_rng(),
//     }
// }

pub fn justify(input: &str, desired_line_length: u8) -> String {
    let mut output = String::new();

    let mut line: Vec<String> = Vec::new();
    let mut line_length: u8 = 0;

    let mut word = String::new();
    let mut word_length: u8 = 0;

    // TODO: split by graphemes instead
    for character in input.chars() {
        if character == ' ' {
            if line_length + word_length <= desired_line_length {
                line.push(word.clone());
                line_length += word_length + 1;
                word_length = 0;
                word.clear();
            } else {
                let mut joined_line = line.join(" ");
                joined_line.push('\n');
                output.push_str(&joined_line);
                line.clear();
                line_length = 0;
            }
        } else {
            word.push(character);
            word_length += 1;
        }

    }

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
        assert_eq!(output, "hello  world  my\nname is computer");
    }
}
