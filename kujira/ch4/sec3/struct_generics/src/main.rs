#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: std::ops::AddAssign,
{
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    // 加算
    fn add(&mut self, pt: Point<T>) {
        self.x += pt.x;
        self.y += pt.y;
    }
}
fn main() {
    let mut pt = Point::new(10, 10);
    println!("{:?}", pt);

    pt.add(Point::new(20, 30));
    println!("{:?}", pt);
}
