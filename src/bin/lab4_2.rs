#[derive(Debug)]
enum Direction {
    Up,
    Down
}

#[derive(Debug)]
enum UIEvent {
    ButtonClicked,
    Scroll(Direction),
    KeyPressed(char),
    MouseClicked { x: i32, y: i32 }
}

fn call(event : UIEvent) {
    use UIEvent::*;
    match event {
        ButtonClicked => println!("Button clicked"), // simple match
        Scroll(x) => println!("Scroll {:?}", x), // attribute extraction
        KeyPressed(ch) => { // whole block
            let up_ch = ch.to_uppercase();
            println!("Key pressed: {}", up_ch);
        },
        MouseClicked { x, y } => println!("Mouse clicked at ({}, {})", x, y), // attribute extraction
    }
}
fn main() {
    use UIEvent::*;
    
    let clicked = ButtonClicked;
    let scroll = Scroll(Direction::Down);
    let key_pressed = KeyPressed('b');
    call(clicked);
    call(scroll);
    call(key_pressed);
}