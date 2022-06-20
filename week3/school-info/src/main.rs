use generator_mod::new as generater_dummy_data;
use school_info::School;
pub mod generator_mod;

fn main() {
    let students: Vec<&str> = vec![
        "Alice", "Bob", "Charle", "Deep", "Elise", "Fade", "Geoge", "Hilari",
    ];
    let grades_number: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let dummy = generater_dummy_data::<u32>(&students, &grades_number);

    // let grades_string: Vec<&str> = vec!["A+", "B-", "C+", "D", "F"];
    // let dummy = generater_dummy_data::<&str>(&students, &grades_string);

    let school = School::with_template(dummy);
    println!("school: {:#?}", school);
}