use crate::RandomGeojsonError;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    pub min_lon: f64,
    pub max_lon: f64,
    pub min_lat: f64,
    pub max_lat: f64,
}

pub const WGS84_BOUNDS: Bounds = Bounds {
    min_lon: -180.0,
    max_lon: 180.0,
    min_lat: -90.0,
    max_lat: 90.0,
};

pub const WEB_MERCATOR_BOUNDS: Bounds = Bounds {
    min_lon: -180.0,
    max_lon: 180.0,
    min_lat: -85.05112878,
    max_lat: 85.05112878,
};

fn random_coords(crs: &Crs) -> (f64, f64) {
    let mut rng = rand::rng();
    let bounds = crs.bounds();
    let longitude = rng.random_range(bounds.min_lon..bounds.max_lon);
    let latitude = rng.random_range(bounds.min_lat..bounds.max_lat);
    (longitude, latitude)
}

pub enum RandomGeometry {
    Point(Vec<f64>),
    LineString(Vec<Vec<f64>>),
    Polygon(Vec<Vec<Vec<f64>>>),
}

impl RandomGeometry {
    /// Creates a random Point geometry.
    pub fn random_point(crs: &Crs) -> RandomGeometry {
        let (lon, lat) = random_coords(crs);
        RandomGeometry::Point(vec![lon, lat])
    }

    /// Creates a random LineString geometry with a random number of points.
    pub fn random_linestring(crs: &Crs) -> Self {
        let num_points = rand::rng().random_range(2..10);
        let coords: Vec<Vec<f64>> = (0..num_points)
            .map(|_| {
                let (lon, lat) = random_coords(crs);
                vec![lon, lat]
            })
            .collect();
        RandomGeometry::LineString(coords)
    }

    /// Creates a random Polygon geometry with a random number of points.
    pub fn random_polygon(crs: &Crs) -> Self {
        let num_points = rand::rng().random_range(3..10);
        let mut coords: Vec<Vec<f64>> = (0..num_points)
            .map(|_| {
                let (lon, lat) = random_coords(crs);
                vec![lon, lat]
            })
            .collect();

        // Close the ring by repeating the first point
        if let Some(first) = coords.first().cloned() {
            coords.push(first);
        }

        RandomGeometry::Polygon(vec![coords])
    }
}

pub enum Crs {
    WGS84,
    WebMercator,
}

impl Crs {
    pub fn bounds(&self) -> Bounds {
        match self {
            Crs::WGS84 => WGS84_BOUNDS,
            Crs::WebMercator => WEB_MERCATOR_BOUNDS,
        }
    }
}

impl std::str::FromStr for Crs {
    type Err = RandomGeojsonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "wgs84" | "4326" => Ok(Crs::WGS84),
            "webmercator" | "web_mercator" | "3857" => Ok(Crs::WebMercator),
            _ => Err(RandomGeojsonError::InvalidArgument(format!(
                "Invalid coordinate system: {}",
                s
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crs_from_str_valid() {
        assert!(matches!("wgs84".parse::<Crs>(), Ok(Crs::WGS84)));
        assert!(matches!("4326".parse::<Crs>(), Ok(Crs::WGS84)));
        assert!(matches!("webmercator".parse::<Crs>(), Ok(Crs::WebMercator)));
        assert!(matches!("web_mercator".parse::<Crs>(), Ok(Crs::WebMercator)));
        assert!(matches!("3857".parse::<Crs>(), Ok(Crs::WebMercator)));
    }

    #[test]
    fn test_crs_from_str_invalid() {
        let result = "unknown".parse::<Crs>();
        assert!(result.is_err());
    }

    fn assert_coords_in_bounds(coords: &[f64], bounds: Bounds) {
        assert!(coords.len() == 2);
        let (lon, lat) = (coords[0], coords[1]);
        assert!(
            bounds.min_lon <= lon && lon <= bounds.max_lon,
            "Longitude {} out of bounds {:?}",
            lon,
            bounds
        );
        assert!(
            bounds.min_lat <= lat && lat <= bounds.max_lat,
            "Latitude {} out of bounds {:?}",
            lat,
            bounds
        );
    }

    #[test]
    fn test_random_point_within_bounds() {
        let crs = Crs::WGS84;
        let bounds = crs.bounds();
        if let RandomGeometry::Point(coords) = RandomGeometry::random_point(&crs) {
            assert_coords_in_bounds(&coords, bounds);
        } else {
            panic!("Expected Point geometry");
        }
    }

    #[test]
    fn test_random_linestring_within_bounds() {
        let crs = Crs::WebMercator;
        let bounds = crs.bounds();
        if let RandomGeometry::LineString(coords) = RandomGeometry::random_linestring(&crs) {
            assert!(coords.len() >= 2);
            for coord in coords {
                assert_coords_in_bounds(&coord, bounds);
            }
        } else {
            panic!("Expected LineString geometry");
        }
    }

    #[test]
    fn test_random_polygon_within_bounds_and_closed() {
        let crs = Crs::WGS84;
        let bounds = crs.bounds();
        if let RandomGeometry::Polygon(rings) = RandomGeometry::random_polygon(&crs) {
            assert_eq!(rings.len(), 1);
            let ring = &rings[0];
            assert!(ring.len() >= 4); // at least 3 + closing point
            for coord in ring {
                assert_coords_in_bounds(coord, bounds);
            }
            assert_eq!(ring.first(), ring.last(), "Polygon ring is not closed");
        } else {
            panic!("Expected Polygon geometry");
        }
    }
}

