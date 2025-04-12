fn main() {
    {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {}", count);
            let mut remaining = 10;
            
            loop {
                println!("remaining = {}", remaining);
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {}", count);
    }

    {
        let mut number = 3;

        while number != 0 {
            println!("{}!", number);
            number -= 1;
        }
        println!("LIFTOFF!!!");
    }

    {
        let arr = [10, 20, 30, 40, 50];
        let mut index = 0;
        while index < arr.len() {
            println!("arr[{index}] = {}", arr[index]);
            index += 1;
        }
    }

    {
        let arr = [10, 20, 30, 40, 50];
        for element in arr {
            println!("the value is : {element}");
        }
    }

    {
        let arr = [10, 20, 30, 40, 50];
        for number in (1..=4).rev() {
            println!("the value(rev) is : {}", arr[number]);
        }
    }
}