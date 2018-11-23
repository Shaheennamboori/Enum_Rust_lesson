enum Direction {
    Up,
    Down,
    Left,
    Right
}
fn main() {
    let p_direction : Direction = Direction::Down;

    match p_direction {
        Direction::Up => println!("we are heading up!"),
        Direction::Down => println!("We are going down"),
        Direction::Left => println!("Going left"),
        Direction::Right => println!("Go die bitch"),
    }
}
