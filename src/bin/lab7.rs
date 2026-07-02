// new empty vector

fn vectors() {
    let v: Vec<i32> = Vec::new();
    print!("ex1\n {:?}\n", v);

    let mut v = vec![1, 2, 3];
    print!(" {:?}\n", v);

    let e1: &i32 = &v[1];

    let e2 = v.get(2);
    print!(" {:?}\n {:?}\n", e1, e2);

    // adding values to vector
    v.push(4);
    v.push(5);

    // removing values
    let e = v.pop();
    print!(" {:?}\n", e.unwrap());

    let mut v = vec![1, 2, 3];
    let first = &v[0];
    println!(" {}\n", *first);
    v.push(4);

    let v = vec![1, 2, 3];
    for e in v {
        print!("{} ", e);
    }
    println!("");
    // cannot use v here
    // let eo = &v[0]; // will cause an error

    let v = vec![1, 2, 3];
    for e in &v {
        // need to use reference (otherwise the v value is moved) (e: &i32)
        print!("{} ", e); // an implicit dereference is performed here
    }
    println!("");

    let mut v = vec![1, 2, 3];
    for e in &mut v {
        // mutable borrow
        *e *= 2; // remember to use dereference operator
        print!("{} ", e);
    }
}

fn main() {
    vectors();
}
