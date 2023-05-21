# haversine-rs

Provides some helpers functions to calculate the distance between two points on Earth using the Haversine formula. Also can find the bearing between two points, and get a point at a given distance and bearing from a given point.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
haversine_rs = "0.2.1"
```

## Usage

```rust
use haversine_rs::point::Point;
use haversine_rs::units::Unit;
use haversine_rs::distance;
use haversine_rs::bearing;
use haversine_rs::find_point;

fn main() {
    let p1 = Point::new(40.7767644, -73.9761399);
    let p2= Point::new(40.771209, -73.9673991);

    let distance = distance(p1, p2, Unit::Miles);
    let bearing = bearing(p1, p2);

    let p3 = find_point(p1, 1.0, 90.0, Unit::Miles);

    println!("Distance: {} miles", distance);
    println!("Bearing: {} degrees", bearing);
    println!("Point at 1 mile and 90 degrees: {:?}", p3);
}
```

### Distance between multiple points

```rust
use haversine_rs::point::Point;
use haversine_rs::units::Unit;
use haversine_rs::distance_vec;

fn main() {
    let p1 = Point::new(40.7767644, -73.9761399);
    let p2 = Point::new(40.773987, -73.971769);
    let p3 = Point::new(40.771209, -73.9673991);

    let distance = distance_vec(vec![point_x, point_y, point_z], Unit::Miles);

    println!("Distance: {} miles", distance);
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
