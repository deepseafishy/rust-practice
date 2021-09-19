enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct Point<T, U> {
    x: T,
    y: U,
}

struct Point2<T> {
    x: T,
    y: T,
}

impl<T> Point2<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point3<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point3<T, U> {
    fn mixup<V, W>(self, other: Point3<V, W>) -> Point3<T, W> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    /*
      Generic types
    */
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    
    let p = Point2 { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    
    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    /*
      
    */
}

