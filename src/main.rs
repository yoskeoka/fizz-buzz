fn main() {
    println!("{}", fizzbuzz(1));
}

fn fizzbuzz(n: i32) -> String {
    return n.to_string();
}

#[test]
fn fizzbuzz1() {
    assert_eq!("1", fizzbuzz(1));
    assert_eq!("2", fizzbuzz(2));
    // assert_eq!("fizz", fizzbuzz(3));
}
