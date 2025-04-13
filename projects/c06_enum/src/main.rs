// enumでIPv4, IPv6を定義する
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(self) {

    }
}

#[derive(Debug)]
enum UsState {
    Albama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    {
        // ipv4, ipv6にそれぞれIPアドレスを保持させる
        let ipv4 = IpAddrKind::V4(
            0, 23, 13, 44
        );
        let ipv6 = IpAddrKind::V6(
            String::from("::1")
        );

        println!("ipv4: {:#?}", ipv4);
        println!("ipv6: {:#?}", ipv6);
    }

    {
        let mes = Message::Write(String::from("hello"));
        mes.call();
    }

    {
        let some_number = Some(5);
        let some_string = Some("A string");
        let absent_number: Option<i32> = None;

        let x: i8 = 5;
        let y: Option<i8> = Some(5);
        let sum = x + y.unwrap();
        println!("sum: {}", sum);
    }

    {
        let coin = Coin::Dime;
        let cents = value_in_cents(coin);
        println!("penny in cent: {}", cents);

        let coin = Coin::Quarter(UsState::Alaska);
        let cents = value_in_cents(coin);
        println!(" in cent: {}", cents);
    }

    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + i)
            }
        }

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }

    {
        let some_u8_value = 15;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => println!("i do not know..."),
        }
    }

    {
        let maybe_three = Some(3u8);
        if let Some(3) = maybe_three {
            println!("three!!");
        }
    }

    {
        let coin = Coin::Quarter(UsState::Albama);
        let mut count = 0;
        match coin {
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
            },
            _ => ()
        }
    }

    {
        let coin = Coin::Quarter(UsState::Alaska);
        let mut count = 0;
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}", state);
        } else {
            count += 1;
        }
    }
}
