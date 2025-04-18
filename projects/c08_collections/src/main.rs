fn main() {
    {
        let v: Vec<i32> = Vec::new();
        let mut v = vec![1, 2, 3];
        v.push(4);
        v.push(5);
        v.push(6);
        {
            let y = vec![2, 4, 6];
            println!("y vector: {:?}", y);
        }
        println!("x vector: {:?}", v);

        let third = &v[2];
        match v.get(2) {
            Some(third) => {
                println!("the third element is {}", third);
            },
            None => {
                println!("There is no third element.");
            }
        }

        //let invalid_access = v[100];
        let invalid_access = v.get(100);
        if let None = invalid_access {
            println!("vector::get() is good.");
        }

        let mut v = vec![1, 2, 3, 4, 5, 6];
        for i in &mut v {
            *i += 10;
            println!("{i}");
        }

        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String)
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12)
        ];

        for cell in row {
            match cell {
                SpreadsheetCell::Int(value) => {
                    println!("type: Int, value: {value}");
                },
                SpreadsheetCell::Float(value) => {
                    println!("type: Float, value: {value}");
                },
                SpreadsheetCell::Text(value) => {
                    println!("type: Text, value: {value}");
                },
            }
        }
    }

    {
        let data = "initial contents";
        let s = data.to_string();
        let s = "initial contents".to_string();
        let mut s = String::from("initial contents");

        s.push_str(", amazing!");
        
        let mut s1 = "foo".to_string();
        let s2 = "bar";
        s1.push_str(s2);
        println!("{s1}, s2 : {s2}");
    }

    {
        let s1 = String::from("foo");
        let s2 = String::from("bar!");
        let s3 = s1 + &s2;
        //println!("{s1} {s2} {s3}");
        let s4 = s3 + &s2;
        //println!("{s3} {s2} {s4}");
    }

    {
        let s1 = "tic".to_string();
        let s2 = "tac".to_string();
        let s3 = "toe".to_string();
        let s4 = format!("{}-{}-{}", s1, s2, s3);
        println!("{s4}");
    }

    {
        let message = String::from("こんにちは！");
        let len = message.len();
        println!("{message}, len: {len}");

        for letter in message.chars() {
            println!("{letter}");
        }

        for b in message.bytes() {
            println!("0x{b:02x}");
        }
    }
}
