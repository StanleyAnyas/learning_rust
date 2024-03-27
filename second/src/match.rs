fn main() {
    let dice_rolled = 7;
    match dice_rolled {
        7 => take_hat(dice_rolled),
        3 => give_hat(dice_rolled),
        other => move_player(other),
    }

    let can_be_none: Option<String> = Some(String::from("value"));
    if let Some(str) = &can_be_none {
        println!("{:?} is not none but is {:?}", &can_be_none, &str);
    }

    let day_of_week = "Monday";

    match day_of_week {
        "Sunday" => println!("It is a rest day"),
        "Friday" => println!("It is the day before weekend"),
        "Saturday" => println!("And the weekend starts"),
        _ => println!("It is week days"),
    }
}

fn give_hat(dice_rolled: usize) {
    println!("hat given because you got {dice_rolled}")
}

fn take_hat(dice_rolled: usize) {
    println!("hat taken because you got {dice_rolled}")
}

fn move_player(num_of_positions: usize) {
    println!("you have {num_of_positions} positions to move")
}
