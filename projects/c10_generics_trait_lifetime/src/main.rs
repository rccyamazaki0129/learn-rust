fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T
}

struct Point2<A, T, U, Z> {
    a: T,
    b: U,
    c: Z,
    d: A
}

impl<A, T, U, Z> Point2<A, T, U, Z> {
    fn c_func_original(&self) -> &Z {
        &self.c
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if largest < item {
            largest = item;
        }
    }
    largest
}

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None
}

mod summary;
use summary::Tweet;
use summary::NewsArticle;
use summary::Summary;

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify2<T: Summary>(item: &T) {
    println!("notify2: Breaking news! {}", item.summarize());
}

fn notify3<T>(item: &T) -> i32 
    where T: Summary {
    println!("notify3: Breaking news! {}", item.summarize());
    0
}

fn notify4<T>(item: &T)
    where T: Summary {
    println!("notify4: Breaking news! {}", item.summarize());
}

fn main() {

    {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);

        let result = largest(&char_list);
        println!("The largest char is {} <T>", result);
    }

    {
        let p = Point { x: 5, y: 10};
        let f = Point { x: 1.22, y: 33.0 };
        let c = Point { x: "c1", y: "a5" };
        let dist = f.distance_from_origin();
    }

    {
        let data = Point2 {a: 10, b: 'C', d: 1.1, c: 10 };
        let r = data.c_func_original();
    }

    {
        let integer = Option_i32::Some(5);
        let flo = Option_f64::Some(3.0);
    }

    {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false
        };

        println!("1 new tweet: {}", tweet.summarize());
    }

    {
        let article = NewsArticle {
            headline: "Penguins win the Stanley Cup Championship!".to_string(),
            location: "Pittsburgh, PA, USA".to_string(),
            author: "Iceburgh".to_string(),
            content: "The Pittsburgh Penguins once again are the best hockey team in the NHL.".to_string()
        };

        println!("New article available! {}", article.summarize());

        println!("{}", article.summarize_author());
    
        notify(&article);
        notify2(&article);
    }

}