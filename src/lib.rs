pub mod units;
pub mod point;

use point::Point;
use units::Unit;

/// Calculates the haversine of the given angle.
///
/// # Examples
///
/// ```
/// use haversine_rs::haversine;
///
/// let theta = 0.0;
/// assert!(haversine(theta) == 0.0);
///
/// let theta = 1.0;
/// assert!(haversine(theta) == 0.22984884706593015);
/// ```
pub fn haversine(theta: f64) -> f64 {
    (theta / 2.0).sin().powi(2)
}

/// Calculates the distance between two points.
///
/// # Examples
///
/// ```
/// use haversine_rs::point::Point;
/// use haversine_rs::units::Unit;
/// use haversine_rs::distance;
///
/// let point_a = Point::new(40.7767644, -73.9761399);
/// let point_b = Point::new(40.771209, -73.9673991);
///
/// let distance = distance(point_a, point_b, Unit::Miles);
///
/// assert!(distance == 0.5971169354597881);
/// ```
pub fn distance(point_a: Point, point_b: Point, unit: Unit) -> f64 {
    let r = unit.earth_radius();

    let point_a = point_a.to_radians();
    let point_b = point_b.to_radians();

    let delta_latitude = point_b.latitude - point_a.latitude;
    let delta_longitude = point_b.longitude - point_a.longitude;

    let a = haversine(delta_latitude) + point_a.latitude.cos()
        * point_b.latitude.cos() * haversine(delta_longitude);

    2.0 * r * a.sqrt().atan2((1.0 - a).sqrt())
}

/// Calculates the bearing between two points.
///
/// # Examples
///
/// ```
/// use haversine_rs::point::Point;
/// use haversine_rs::bearing;
///
/// let point_a = Point::new(40.7767644, -73.9761399);
/// let point_b = Point::new(40.771209, -73.9673991);
///
/// let bearing = bearing(point_a, point_b);
///
/// assert!(bearing == 130.00282723561608);
/// ```
pub fn bearing(point_a: Point, point_b: Point) -> f64 {
    let point_a = point_a.to_radians();
    let point_b = point_b.to_radians();

    let delta_longitude = point_b.longitude - point_a.longitude;

    let y = delta_longitude.sin() * point_b.latitude.cos();
    let x = point_a.latitude.cos() * point_b.latitude.sin()
        - point_a.latitude.sin() * point_b.latitude.cos() * delta_longitude.cos();

    let result = y.atan2(x).to_degrees();

    if result < 0.0 {
        result + 360.0
    } else {
        result
    }
}

/// Calculates the point at the given distance and bearing from the origin point.
///
/// # Examples
///
/// ```
/// use haversine_rs::point::Point;
/// use haversine_rs::units::Unit;
/// use haversine_rs::find_point;
///
/// let origin = Point::new(40.7767644, -73.9761399);
/// let distance = 0.5971169354597881;
/// let bearing = 130.00282723561608;
///
/// let point = find_point(origin, distance, bearing, Unit::Miles);
///
/// println!("{:?}", point);
/// assert!(point.latitude == 40.771209);
/// assert!(point.longitude == -73.9673991);
/// ```
pub fn find_point(origin: Point, distance: f64, bearing: f64, unit: Unit) -> Point {
    let r = unit.earth_radius();

    let origin = origin.to_radians();

    let delta = distance / r;

    let theta = bearing.to_radians();

    let latitude = (origin.latitude.sin() * delta.cos()
        + origin.latitude.cos() * delta.sin() * theta.cos()).asin();

    let longitude = origin.longitude
        + (theta.sin() * delta.sin() * origin.latitude.cos()).atan2(delta.cos() - origin.latitude.sin() * latitude.sin());

    Point::new(latitude.to_degrees(), longitude.to_degrees())
}
