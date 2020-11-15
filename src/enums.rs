pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}



pub fn var_enum() {
    let direction = Direction::Down;

    match direction {
        Direction::Down => println!("DOWN!"),
        Direction::Up => println!("UP"),
        Direction::Left => println!("LEFT"),
        Direction::Right => println!("RIGHT"),
    }
}