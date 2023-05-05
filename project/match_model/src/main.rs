enum Direction {
    East,
    West,
    North,
    South,
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        }
        _ => println!("West"),
    };

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("Point from (0,0) move to ({},{})", x, y);
            }
            Action::ChangeColorRGB(r, g, b) => {
                println!("Change color into '(r:{},g:{},b:{}')", r, g, b);
            }
        }
    }

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        5 => println!("five"),
        _ => (),
    }

    let v = Some(3u8);
    if let Some(a) = v {
        println!("{a}");
    }

    let foo = 'f';
    assert!(matches!(foo,'A'..='Z'|'a'..='z'));
    let bar = Some(4);
    assert!(matches!(bar,Some(x)if x >2));

    let age = Some(30);
    println!("匹配前 age is {:?}", age);
    if let Some(age) = age {
        println!("匹配中 age is {:?}", age);
    }
    println!("匹配后 age is{:?}", age);
}
