
enum IpAddressVer {
    Ver4,
    Ver6
}

#[derive(Debug)]
enum Phone {
    Ios(String),
    Andriod(String),
}

// Fix the errors
enum Number {
    _Zero,
    One,
    _Two,
}

enum Number1 {
    _Zero = 0,
    One,
    _Two,
}

// C-like enum
enum Number2 {
    _Zero,
    One,
    _Two,
}

enum Message {
    _Quit,
    Move { x: i32, y: i32 },
    _Write(String),
    _ChangeColor(i32, i32, i32),
}

fn main() {
    let _four = IpAddressVer::Ver4;
    let _six = IpAddressVer::Ver6;

    let my_phone = Phone::Ios("Ipone11".to_string());
    let his_phone = Phone::Andriod("Samsung".to_string().to_uppercase());

    println!("I have {:?} and he has {:?}", my_phone, his_phone);

        // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as i32, Number1::One as i32);
    assert_eq!(Number1::One as i32, Number2::One as i32);

    // println!("Success!")
    let msg = Message::Move{ x: 3, y: 4 };

    if let Message::Move { x: a, y: b } = msg {
        assert_eq!(a, b);
    } else {
        panic!("Don't run");
    }
}