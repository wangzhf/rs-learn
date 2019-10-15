
enum Direction {
    East,
    West,
    South,
    North
}

// match的每个分支返回值类型必须一致，或者某个分支panic
pub fn test_match() {
    let dict = Direction::North;
    match dict {
        Direction::North => println!("This is North"),
        Direction::South | Direction::West => println!("South or West"),
        _ => panic!("Not support the direction"),
    }

    let new_dict = match dict {
        Direction::West => "West",
        Direction::North | Direction::South => {
            "North or South"
        },
        _ => panic!("Not support the direction"),
    };
    println!("{}", new_dict);
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

pub fn test_match2() {
    let action = Action::Say("Hello".to_string());
    match action {
        Action::Say(s) => {
            println!("{}", s);
        },
        Action::MoveTo(x, y) => {
            println!("point from (0, 0) move to ({}, {})", x, y);
        },
        Action::ChangeColorRGB(r, g, _) => {
            println!("change color into '(r: {}, g: {}, b: 0)', 'b' has been ignored", r, g);
        }
    }
}

