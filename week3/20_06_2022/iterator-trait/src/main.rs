use iterator_trait::fibonacci_numbers;

fn main() {
    // let mut fibonacci_seq = fibonacci_numbers().into_iter();
    // assert_eq!(fibonacci_seq.next(), Some(1));
    // assert_eq!(fibonacci_seq.next(), Some(1));
    // assert_eq!(fibonacci_seq.next(), Some(2));
    // assert_eq!(fibonacci_seq.next(), Some(3));
    // assert_eq!(fibonacci_seq.next(), Some(5));
    // assert_eq!(fibonacci_seq.next(), Some(8));
    // assert_eq!(fibonacci_seq.next(), Some(13));
    // assert_eq!(fibonacci_seq.next(), Some(21));
    // assert_eq!(fibonacci_seq.next(), Some(34));
    // assert_eq!(fibonacci_seq.next(), Some(55));
    // assert_eq!(fibonacci_seq.next(), Some(89));
    // assert_eq!(fibonacci_seq.next(), Some(144));
    // assert_eq!(fibonacci_seq.next(), Some(233));
    // assert_eq!(fibonacci_seq.next(), Some(377));

    for number in fibonacci_numbers().into_iter() {
        println!("number: {:#?}", number);
    }
}