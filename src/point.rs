/// A point on the earth's surface.
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
}

impl Point {
    /// Creates a new point with the given latitude and longitude.
    ///
    /// # Examples
    ///
    /// ```
    /// use haversine_rs::point::Point;
    ///
    /// let point = Point::new(40.7128, 74.0060);
    ///
    /// assert!(point.latitude == 40.7128);
    /// assert!(point.longitude == 74.0060);
    /// ```
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    /// Converts the latitude and longitude of the point to radians.
    ///
    /// # Examples
    ///
    /// ```
    /// use haversine_rs::point::Point;
    ///
    /// let point = Point::new(40.7128, 74.0060);
    /// let radians = point.to_radians();
    ///
    /// assert!(radians.latitude == 0.7105724077059474);
    /// assert!(radians.longitude == 1.2916483662309235);
    /// ```
    pub fn to_radians(&self) -> Self {
        Self {
            latitude: self.latitude.to_radians(),
            longitude: self.longitude.to_radians(),
        }
    }

    /// Converts the latitude and longitude of the point to degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// use haversine_rs::point::Point;
    ///
    /// let point = Point::new(0.7088734439698185, 1.2915436464758032);
    /// let degrees = point.to_degrees();
    ///
    /// assert!(degrees.latitude == 40.61545654837404);
    /// assert!(degrees.longitude == 73.99999999999996);
    /// ```
    pub fn to_degrees(&self) -> Self {
        Self {
            latitude: self.latitude.to_degrees(),
            longitude: self.longitude.to_degrees(),
        }
    }
}
