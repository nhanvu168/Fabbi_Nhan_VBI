use school_info as school;

#[test]
fn school_with_noone() {
    let new_school = school::School::<u32>::new();
    assert_eq!(new_school.grades(), vec![]);
}

#[test]
fn school_with_a_student() {
    let mut school = school::School::<u32>::new();
    school.add("Lee", 2);

    assert_eq!(school.grades(), vec![2])
}

#[test]
fn school_with_a_student_advanced() {
    let mut school = school::School::<&str>::new();
    school.add("Lee", "A+");

    assert_eq!(school.grades(), vec!["A+"])
}

#[test]
fn school_with_two_students() {
    let mut school = school::School::<u32>::new();
    school.add("Lee", 2);
    school.add("Nancy", 3);

    assert_eq!(school.grades(), vec![2, 3])
}

#[test]
fn school_with_two_students_advanced() {
    let mut school = school::School::<&str>::new();
    school.add("Lee", "A");
    school.add("Nancy", "B");

    assert_eq!(school.grades(), vec!["A", "B"])
}

#[test]
fn school_with_three_students() {
    let mut school = school::School::<u32>::new();
    school.add("Bob", 4);
    school.add("Alice", 4);
    school.add("Tom", 5);

    assert_eq!(school.grade(4), vec!["Alice", "Bob"])
}

#[test]
fn school_with_three_students_advanced() {
    let mut school = school::School::<&str>::new();
    school.add("Bob", "F");
    school.add("Alice", "F");
    school.add("Tom", "C");

    assert_eq!(school.grade("F"), vec!["Alice", "Bob"])
}