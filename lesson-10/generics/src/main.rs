struct Point<X, Y> {
    x: X,
    y: Y,
}

impl<X, Y> Point<X, Y> {
    fn mixup<A, B>(self, other: Point<A, B>) -> Point<X, B> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let point1 = Point { x: 5, y: 10 };
    let point2 = Point { x: "Hello", y: 'c' };
    let point3 = point1.mixup(point2);
    println!("Point 3 - x:{}, y:{}", point3.x, point3.y);
}
