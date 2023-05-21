# haversine-rs

Provides some helpers functions to calculate the distance between two points on Earth using the Haversine formula. Also can find the bearing between two points, and get a point at a given distance and bearing from a given point.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
haversine_rs = "0.1.0"
```

## Usage

```rust
use haversine_rs::point::Point;
use haversine_rs::units::Unit;
use haversine_rs::distance;

fn main() {
    let p1 = Point::new(40.7767644, -73.9761399);
    let p2= Point::new(40.771209, -73.9673991);

    let distance = distance(p1, p2, Unit::Miles);
    let bearing = bearing(p1, p2);

    let p3 = point_at(p1, 1.0, 90.0, Unit::Miles);

    println!("Distance: {} miles", distance);
    println!("Bearing: {} degrees", bearing);
    println!("Point at 1 mile and 90 degrees: {:?}", p3);
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
