fn main() {
    let first_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let second_arr = [6, 8, 10];

    let result = is_sublist(&second_arr, &first_arr);

    println!("result: {:#?}", result);
}

fn is_sublist(array: &[i32], other_array: &[i32]) -> bool {
    if array.len() == 0 {
        return true;
    }

    for frame in other_array.windows(array.len()) {
        if frame == array {
            return true;
        }
    }

    return false;
}

#[test]
fn test_empty_array_is_sublist() {
    let first_array: [i32; 0] = [];
    let second_array: [i32; 3] = [6, 8, 10];

    assert_eq!(is_sublist(&first_array, &second_array), true)
}

#[test]
fn test_first_array_is_included_in_second_array() {
    let first_array: [i32; 2] = [6, 8];
    let second_array: [i32; 3] = [6, 8, 10];

    assert_eq!(is_sublist(&first_array, &second_array), true)
}

#[test]
fn test_first_array_is_not_included_in_second_array() {
    let first_array: [i32; 2] = [6, 9];
    let second_array: [i32; 3] = [6, 8, 10];

    assert_eq!(is_sublist(&first_array, &second_array), false)
}

#[test]
fn test_first_array_is_not_included_in_second_array_2() {
    let first_array: [i32; 4] = [6, 8, 10, 11];
    let second_array: [i32; 3] = [6, 8, 10];

    assert_eq!(is_sublist(&first_array, &second_array), false)
}