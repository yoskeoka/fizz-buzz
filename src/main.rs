fn main() {
    for n in 1..200 {
        println!("{}", fizzbuzz(n));
    }
}

fn fizzbuzz(n: i32) -> String {
    if n % 15 == 0 {
        return "fizzbuzz".to_string();
    }
    if n % 5 == 0 {
        return "buzz".to_string();
    }
    if n % 3 == 0 {
        return "fizz".to_string();
    }
    return n.to_string();
}

#[test]
fn fizzbuzz1() {
    assert_eq!("1", fizzbuzz(1));
    assert_eq!("2", fizzbuzz(2));
    assert_eq!("fizz", fizzbuzz(3));
    assert_eq!("buzz", fizzbuzz(5));
    assert_eq!("fizzbuzz", fizzbuzz(15));
}
