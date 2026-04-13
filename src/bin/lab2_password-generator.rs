use std::io::{self, Write};
use rand::{Rng, rng};
const start: u8 = 'a' as u8;
const end: u8 = 'z' as u8;
const size: usize = (end - start + 1) as usize;

fn generate_chars(chars : &mut [char; size]){
    let mut char_idx = start;

    for i in 0..size {
        chars[i] = char_idx as char;
        char_idx += 1;
    }
}
fn generate_random_char(chars : &[char;size]) -> char{
    let mut rng = rand::rng();
    let rand_number = rng.random_range(0..size);
    chars[rand_number]
}
fn input() -> usize {
    let mut line = String::new();
    print!("Enter length of the password: ");
    io::stdout().flush();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to readline");
    line.trim().parse().expect("input is not int")
}
fn main() {
    let mut chars = ['0'; size];
    generate_chars(&mut chars);
    for idx in 0..5{
        let password_leng = input();
        for password_idx in 0..password_leng{
            print!("{}",generate_random_char(&chars) );
        }
        println!("\n");
    }
}
