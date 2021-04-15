
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y
        }
    }
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![3, 6, 8, 2, 4];
    let result = largest(&number_list);
    println!("{}", result);

    let char_list = vec!['c', 'a', 'g', 'z'];
    let result = largest(&char_list);
    println!("{}", result);

    let a = Point{x: 2, y: 3};
    println!("{:?}", a);
    println!("{}", a.x());

    let b = Point{x: 2.0, y: 3.0};
    println!("{}", b.distance_from_origin());

    let c = Point2{x: 1, y: 2.0};
    let d = Point2{x: 'a', y: 'c'};
    println!("{:?}", c.mixup(d))
}
