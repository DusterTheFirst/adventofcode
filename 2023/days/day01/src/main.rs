use regex::Regex;

fn main() {
    let file = std::fs::read_to_string("input/day1")
        .expect("day1 input should exist in input directory");

    let digit_word_regex =
        Regex::new("one|two|three|four|five|six|seven|eight|nine|[1-9]").unwrap();

    let sum: u32 = file
        .lines()
        .map(|line| {
            let mut digits = std::iter::successors(digit_word_regex.find(line), |previous_match| {
                digit_word_regex.find_at(line, previous_match.start() + 1)
            })
            .map(|digit_match| match digit_match.as_str() {
                "1" | "one" => 1,
                "2" | "two" => 2,
                "3" | "three" => 3,
                "4" | "four" => 4,
                "5" | "five" => 5,
                "6" | "six" => 6,
                "7" | "seven" => 7,
                "8" | "eight" => 8,
                "9" | "nine" => 9,
                _ => unreachable!(),
            });

            let first_digit = digits.next().unwrap();
            let second_digit = digits.last().unwrap_or(first_digit);
            println!("{line}: {first_digit}{second_digit}");

            first_digit * 10 + second_digit
        })
        .sum();

    println!("{sum}");
}
