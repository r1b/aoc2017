pub fn solve_captcha(input: &'static str, star: usize) -> u32 {
    let digits: Vec<char>  = input.chars().collect();
    let lookahead: usize = match star {
        1 => 1,
        2 => digits.len() / 2,
        _ => panic!("Idk"),
    };
    let mut sum: u32 = 0;

    for (index, digit) in digits.iter().enumerate() {
        if candidate_digit(&digits, lookahead, index) == *digit {
            sum += digit.to_digit(10).unwrap();
        }
    }

    sum
}

fn candidate_digit(input: &Vec<char>, lookahead: usize, index: usize) -> char {
    input[(index + lookahead) % input.len()]
}
