fn notMain() {
    {
        let x: f64 = 2.422539959;
        let y: f32 = 3.0;
        let bits = x.to_bits();
        println!("x: {x}, y: {y}, bits: {:064b}", bits);
        println!("x: {x}, y: {y}, bits: {:x}", bits);
    }

    {
        let sum = 5 + 10;
        let diff = 43.2 - 13.5;
        let product = 4 * 30;
        let quotient = 24.4 / 2.4;
        let remainder = 43 % 5;
        println!("sum: {sum}, diff: {diff}, product: {product}, quotient: {quotient}, remainder: {remainder}");
    }

    {
        let _t = true;
        let _f: bool = false;
    }

    {
        let c = 'x';
        let japanese = 'あ';
        let emoji = '😍';
        println!("c: {c}, japanese: {japanese}, emoji: {emoji}");
    }

    {
        let tupple: (i32, f64, &str) = (13, 5.322, "テスト");
        let (x, y, z) = tupple;
        let japanese = tupple.2;
        println!("tupple: {tupple:?}, x: {x}, y: {y}, z: {z}");
        println!("japanese: {japanese}");
    }

    {
        let arr = [1, 3, 5, 7, 9];
        println!("arr: {:?}", arr);

        let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
        println!("months: {}", months[11]);

        let arr2 = [3; 5];
        println!("arr2: {:?}", arr2);
    }

    {
        let a = [1, 2, 3, 4, 5];
        println!("array: {:?}", a);
        println!("Please enter an index: ");

        let mut index = String::new();

        std::io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];

        println!("The value at index {index} is: {element}");
    }

}
