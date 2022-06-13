use regex::Regex;
use std::io::stdin;

#[derive(PartialEq)]
enum SearchType {
    Absolute,
    Relative,
    Regex,
}
const GIVEN_SENTENCE: &str =
    "https://ars.els-cdn.com/content/image/1-s2.0-S0960982203005347-mmc6.Txtx";
// const GIVEN_SENTENCE_TO_SEARCH_REGEX: &str = "This is foo and FOO foo as well as FoO.";

fn main() {
    let input = input("Enter something to search", None);
    let raw_search_type = input_with_loop(
        "\nEnter type of search",
        Some("\t1: Absolute\n\t2: Relative\n\t3: Regex"),
    );

    let number_of_word = count_word_with_specific_type(&input, _enum(&raw_search_type));
    println!("Number of matches: {:#?}", number_of_word);
}

/**
 * Basic input
 */
fn input(message: &str, instruction: Option<&str>) -> String {
    let mut line: String = String::new();
    println!("{}:", message);

    match instruction {
        Some(ins) => println!("{}", ins),
        None => (),
    }

    stdin().read_line(&mut line).unwrap();
    line.pop();

    line
}

/**
 * Loop until enter the right value
 */
fn input_with_loop(message: &str, instruction: Option<&str>) -> String {
    let absolute: &str = "1";
    let relative: &str = "2";
    let regex: &str = "3";

    loop {
        let mut line = String::new();
        println!("{}:", message);

        match instruction {
            Some(_instruction) => println!("{}", _instruction),
            None => (),
        }

        stdin().read_line(&mut line).unwrap();
        line.pop().unwrap();

        if line == absolute || line == relative || line == regex {
            return line;
        } else {
            println!("Try again!\n")
        }
    }
}

/**
 * Main function to implement the searching functionality
 */
fn count_word_with_specific_type(input: &str, search_type: SearchType) -> u32 {
    let mut number_of_matches: u32 = 0;
    let mut _given_sentence: Vec<char> = Vec::new();
    let mut _input: Vec<char> = Vec::new();

    match search_type {
        SearchType::Relative => {
            _given_sentence = split_to_letter(&GIVEN_SENTENCE.to_lowercase());
            _input = split_to_letter(&input.to_lowercase());
        }
        SearchType::Absolute => {
            _given_sentence = split_to_letter(&GIVEN_SENTENCE);
            _input = split_to_letter(&input);
        }
        SearchType::Regex => {
            return search_by_regex(input);
        }
    }

    for frame in _given_sentence.windows(_input.len()) {
        if frame == _input {
            number_of_matches = number_of_matches + 1;
        }
    }

    number_of_matches
}

/**
 * Search by regex
 */
fn search_by_regex(regex: &str) -> u32 {
    // regex = (?i)foo // sample input regex
    let _regex: Regex = Regex::new(regex).unwrap();
    _regex.find_iter(GIVEN_SENTENCE).count() as u32
}

/**
 * Split input string into a Vector
 */
fn split_to_letter(value: &str) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();

    for char in value.chars() {
        result.push(char);
    }

    result
}

/**
 * Convert raw_string to enum type
 */
fn _enum(raw_string: &str) -> SearchType {
    match raw_string {
        "1" => SearchType::Absolute,
        "2" => SearchType::Relative,
        "3" => SearchType::Regex,
        _ => panic!("Value not in range!"),
    }
}