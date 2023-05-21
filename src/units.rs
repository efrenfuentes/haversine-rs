
/// The unit of measurement to use when calculating the distance between two points.
#[derive(Debug, Clone)]
pub enum Unit {
    Kilometers,
    Miles,
    Meters,
}

impl Unit {
    /// Returns the radius of the earth in the units specified.
    ///
    /// # Examples
    ///
    /// ```
    /// use haversine_rs::units::Unit;
    ///
    /// let mut unit = Unit::Kilometers;
    /// assert!(unit.earth_radius() == 6_371.0);
    ///
    /// unit = Unit::Miles;
    /// assert!(unit.earth_radius() == 3_959.0);
    ///
    /// unit = Unit::Meters;
    /// assert!(unit.earth_radius() == 6_371_000.0);
    /// ```
    pub fn earth_radius(self) -> f64 {
        match self {
            Unit::Kilometers => 6_371.0,
            Unit::Miles => 3_959.0,
            Unit::Meters => 6_371_000.0,
        }
    }
}
