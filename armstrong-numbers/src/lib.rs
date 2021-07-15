use std::u32;

pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let n_digits = num_string.len();
    let arms_strong = num_string
        .chars()
        .map(|x| x.to_digit(10).unwrap_or(0).pow(n_digits as u32))
        .sum::<u32>();
    arms_strong == num
}
