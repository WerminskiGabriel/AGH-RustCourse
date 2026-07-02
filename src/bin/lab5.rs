fn ex1() {
    let ref_x;
    {
        let x = 5;
        ref_x = x;
    }
    println!("ex1\n ref_x={}", ref_x);
}

fn ex2() {
    fn len_longer_array(a: &[i32], b: &[i32]) -> usize {
        if a.len() > b.len() { a.len() } else { b.len() }
    }
    fn longer_array<'t>(a: &'t [i32], b: &'t [i32]) -> &'t [i32] {
        if a.len() > b.len() { a } else { b }
    }

    let a: [i32; 5] = [5; 5];
    let b = [6; 6];
    let len = len_longer_array(&a, &b);
    let longer = longer_array(&a, &b);

    print!("ex2\n size:{}\n arr:{:?}", len, longer);
}

fn ex3() {
    #[derive(Debug)]
    struct Introduction<'t> {
        intro: &'t str,
    }
    impl<'t> Introduction<'t> {
        fn print(&self) {
            println!("{}", self.intro);
        }
    }

    fn get_sample_text() -> &'static str {
        "just text"
    }

    let text =
        String::from("Introduction to a long text. The rest of long text with many sentences.");
    let intro = text
        .split('.')
        .next()
        .expect("Could not find a first sentence.");
    let i = Introduction { intro };
    print!("\nex3\n {:?}\n {}", i, i.intro);

    
}
fn main() {
    ex1();
    ex2();
    ex3();
}
