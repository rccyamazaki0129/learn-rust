#[derive(Debug)]
struct Rect {
    width: i32,
    height: i32
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn is_bigger_than(&self, target: &Rect) -> bool {
        target.area() < self.area()
    }

    // 関連関数: 渡された長さで正方形を作成する
    fn create_square(width: i32) -> Rect {
        Rect{width: width, height: width}
    }
}

fn main() {
    {
        // タプルを使ってwidth, heightを指定する
        let rec = (30, 50);
        // 長方形の面積を求める関数をprint関数内で呼び出して結果を確認する
        println!("Rec area: {}", calc_area(rec));
    }

    {
        // 構造体を使ってwidth, heightを指定する
        let rect = Rect{
            width: 30, height: 50
        };
        // 長方形の面積を求める関数をprint関数内で呼び出して結果を確認する
        println!("Rect area: {}", calc_rect_area(&rect));
        // rectのフィールドをprintしたい
        println!("rect: {:#?}", rect);
    }

    {
        let rect = Rect{width: 40, height: 10};
        let rect2 = Rect{width: 39, height: 10};
        println!("is rect bigger than rect2 ?: {}", rect.is_bigger_than(&rect2));
        println!("rect area: {}", rect.area());
        println!("rect2 area: {}", rect2.area());
    }

    {
        let sq = Rect::create_square(50);
        println!("square: {:?}", sq);
    }
}

fn calc_rect_area(rect: &Rect) -> i32 {
    rect.width * rect.height
}

fn calc_area(rec: (i32, i32)) -> i32 {
    rec.0 * rec.1
}