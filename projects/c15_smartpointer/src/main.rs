enum List {
    Cons(i32, Box<List>),
    Nil
}

use List::{ Cons, Nil };

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomerSmartPointer with data {}", self.data);
    }
}

use std::rc::Rc;

enum List2 {
    Cons(i32, Rc<List2>),
    Nil
}

fn main() {
    {
        let list = Cons(1,
            Box::new(Cons(2,
                Box::new(Cons(3,
                    Box::new(Nil))))));
    }

    {
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }

    {
        let c = CustomSmartPointer { data: String::from ("my stuff")};
        let mut d = CustomSmartPointer { data: String::from ("other stuff")};
        println!("CustomerSmartPointers created.");
        drop(d);
    }

    {
        let a = Rc::new(List2::Cons(5, Rc::new(List2::Cons(10, Rc::new(List2::Nil)))));
        println!("count after creating a: {}", Rc::strong_count(&a));
        let b = List2::Cons(3, Rc::clone(&a));
        println!("count after creating b: {}", Rc::strong_count(&a));
        {
            let c = List2::Cons(4, Rc::clone(&a));
            println!("count after creating c: {}", Rc::strong_count(&a));
        }
        println!("count after destroying c: {}", Rc::strong_count(&a));
    }

}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

