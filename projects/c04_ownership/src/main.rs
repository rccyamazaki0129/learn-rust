fn main() {
    {
        let st = "Hello";
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{}", st);
        println!("{}", s);
    }

    {
        let mut x = 5;
        let y = x;
        x += 1;
        println!("x: {}, y: {}", x, y);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1: {}", s1);
        println!("s2: {}", s2);
    }

    {
        let s = String::from("hello");
        takes_ownership(s.clone());
        let s2 = s;

        let x = 5;
        makes_copy(x);
        let x2 = x;
    }

    {
        let s1 = gives_ownership();
        println!("s1: {}", s1);

        let s2 = String::from("hello");
        println!("s2: {}", s2);

        let s3 = takes_and_gives_back(s2);
        println!("s3: {}", s3);
    }

    {
        let s1 = String::from("hello");
        let (s2, len) = calc_length(s1);
        println!("The length of '{}' is {}.", s2, len);
    }

    {
        let s1 = String::from("Hello");
        let len = calc_length2(&s1);
        println!("The length of '{}' is {}.", s1, len);
    }

    {
        let mut s = String::from("hello");
        change(&mut s);
        println!("s: {}", s);
    }

    {
        let mut s = String::from("Hello");
        let r1 = &s;
        let r2 = &s;
        //let r3 = &mut s;
        println!("r1: {}, r2: {}", r1, r2);
    }

    {
        let reference_to_nothing = dangle();
    }

    {
        let sentence = String::from("hello world");
        let pos = first_word(&sentence[..]);
        let s2 = sentence.clone();
        println!("The first word is : {}", pos);
        println!("The sentense was {}", s2);
    }

    {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        println!("The array is: {:?}", a);
        println!("The slice is: {:?}", slice);
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' || item == b',' {
            return &s[..i];
        }
    }
    &s[..]
}

fn dangle() -> String {
    let s = String::from("Hello");
    s
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

fn calc_length2(s: &String) -> usize {
    s.len()
}

fn calc_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}