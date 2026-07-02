
fn increment(n: Option<i32>) -> Option<i32> {
    match n {
        Some(n) => Some(n + 1),
        None => None,
    }
}

fn sum(x : Option<i32>, y : Option<i32>) -> Option<i32> {
    Some(x? + y? )
}

fn main() {
    let x = sum( Some(2),Some(3));
    print!("{:?}", x );

    assert_eq!(x.unwrap(), 5 );
}