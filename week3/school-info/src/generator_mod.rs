use rand::Rng;
use std::collections::HashMap;

pub fn new<T>(students: &Vec<&str>, grades: &Vec<T>) -> HashMap<String, T>
where
    T: Copy,
{
    let mut result = HashMap::<String, T>::new();

    for student in students {
        result.insert(student.to_string(), pick_one(&grades));
    }

    result
}

fn pick_one<T>(data: &Vec<T>) -> T
where
    T: Copy,
{
    let data_len = data.len();
    let scoped_random_number: usize = random() % data_len;
    data[scoped_random_number]
}

fn random() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen::<usize>()
}