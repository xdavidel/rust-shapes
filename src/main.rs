mod shapes;

use shapes::Shape;

fn main() {
    let c = shapes::Circle {
        x: 0.0,
        y: 0.0,
        radius: 5.0,
    };

    let r = shapes::Rectangle {
        width: 10.0,
        height: 2.0,
    };

    let t = shapes::Triangle {
        a: 18.0,
        b: 30.0,
        c: 24.0,
    };

    println!("{:?}, Area: {}, Perimeter : {}", c, c.area(), c.perimeter());
    println!("{:?}, Area: {}, Perimeter : {}", r, r.area(), r.perimeter());
    println!("{:?}, Area: {}, Perimeter : {}", t, t.area(), t.perimeter());
}
