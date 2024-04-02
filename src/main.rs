trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}

struct Square {
    side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side.powi(2)
    }
}

fn print_area<T: Area>(shape: T) {
    println!("The area of the shape is: {}", shape.area());
}

fn main() {
    println!("Hello, world!");
    let circle = Circle { radius: 3.0 };
    let triangle = Triangle {
        base: 2.0,
        height: 6.0,
    };
    let square = Square { side: 4.0 };

    print_area(circle);
    print_area(square);
    print_area(triangle);
}
