mod error;
mod geometry;

use clap::Parser;
use error::{RandomGeojsonError, RandomGeojsonResult};
use geojson::Value::{LineString, Point, Polygon};
use geojson::feature::Id;
use geojson::{Feature, FeatureCollection, Geometry, JsonObject};
use geometry::{Crs, RandomGeometry};
use rand::Rng;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(
    name = "Random Geojson",
    about = "Random Geojson is a tool to generate random geojson data.",
    version
)]
pub struct Cli {
    /// Number of properties (optional, defaults to 0)
    #[arg(long, default_value_t = 0, value_parser = validate_zero_or_more)]
    pub num_properties: usize,

    /// Length of data (optional, defaults to 100)
    #[arg(long, default_value_t = 100, value_parser = validate_zero_or_more)]
    pub length: usize,

    /// Type of Geometry to generate (optional, defaults to "Point")
    /// Possible values: "Point", "LineString", "Polygon", "All"
    #[arg(long, default_value = "All", value_parser = validate_geometry_type)]
    pub geometry_type: String,

    /// Coordinate system to use (optional, defaults to "WGS84")
    /// Possible values: "WGS84", "WebMercator", "4326", "3857"
    #[arg(long, default_value = "WGS84", value_parser = validate_coordinate_system)]
    pub coordinate_system: String,

    /// Output GeoJSON format in pretty print (optional, defaults to false)
    #[arg(long, default_value_t = false)]
    pub pretty: bool,

    // File name to save the generated GeoJSON (optional, defaults to "random.geojson")
    #[arg(short, long, default_value = "random.geojson")]
    pub output_file: String,
}

fn main() -> RandomGeojsonResult<()> {
    let cli = Cli::parse();

    let mut fc = FeatureCollection::default();

    for _ in 0..cli.length {
        let mut feature = Feature {
            id: Some(Id::String(Uuid::new_v4().to_string())),
            ..Default::default()
        };

        let crs: Crs = cli.coordinate_system.parse()?;

        // Generate a random WGS84 coordinate
        let geometry = match cli.geometry_type.to_lowercase().as_str() {
            "point" => match RandomGeometry::random_point(&crs) {
                RandomGeometry::Point(coords) => Geometry {
                    bbox: None,
                    value: Point(coords),
                    foreign_members: None,
                },
                _ => unreachable!(),
            },
            "linestring" => match RandomGeometry::random_linestring(&crs) {
                RandomGeometry::LineString(coords) => Geometry {
                    bbox: None,
                    value: LineString(coords),
                    foreign_members: None,
                },
                _ => unreachable!(),
            },
            "polygon" => match RandomGeometry::random_polygon(&crs) {
                RandomGeometry::Polygon(coords) => Geometry {
                    bbox: None,
                    value: Polygon(coords),
                    foreign_members: None,
                },
                _ => unreachable!(),
            },
            "all" => {
                let mut rng = rand::rng();
                match rng.random_range(0..3) {
                    0 => match RandomGeometry::random_point(&crs) {
                        RandomGeometry::Point(coords) => Geometry {
                            bbox: None,
                            value: Point(coords),
                            foreign_members: None,
                        },
                        _ => unreachable!(),
                    },
                    1 => match RandomGeometry::random_linestring(&crs) {
                        RandomGeometry::LineString(coords) => Geometry {
                            bbox: None,
                            value: LineString(coords),
                            foreign_members: None,
                        },
                        _ => unreachable!(),
                    },
                    2 => match RandomGeometry::random_polygon(&crs) {
                        RandomGeometry::Polygon(coords) => Geometry {
                            bbox: None,
                            value: Polygon(coords),
                            foreign_members: None,
                        },
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }
            _ => {
                return Err(RandomGeojsonError::InvalidArgument(
                    "Invalid geometry type".to_string(),
                ));
            }
        };

        feature.geometry = Some(geometry);

        // Generate random properties
        if cli.num_properties > 0 {
            let mut properties = JsonObject::new();

            for i in 1..=cli.num_properties {
                let key = format!("prop{}", i);
                let value = random_property_value();
                properties.insert(key, value);
            }

            feature.properties = Some(properties);
        }

        // Add the feature to the feature collection
        fc.features.push(feature);
    }

    // Save the generated GeoJSON to a file
    if cli.pretty {
        save_geojson_to_file(&fc, &cli.output_file, true)?;
    } else {
        save_geojson_to_file(&fc, &cli.output_file, false)?;
    }

    Ok(())
}

// Validates that the value is zero or more.
fn validate_zero_or_more(value: &str) -> RandomGeojsonResult<usize> {
    value
        .parse::<usize>()
        .map_err(|_| RandomGeojsonError::InvalidArgument("Value must be zero or more".to_string()))
}

// Validates the geometry type.
fn validate_geometry_type(value: &str) -> RandomGeojsonResult<String> {
    match value.to_lowercase().as_str() {
        "point" | "linestring" | "polygon" | "all" => Ok(value.to_string()),
        _ => Err(RandomGeojsonError::InvalidArgument(
            "Geometry type must be one of: Point, LineString, Polygon".to_string(),
        )),
    }
}

// Validates the coordinate system.
fn validate_coordinate_system(value: &str) -> RandomGeojsonResult<String> {
    match value.to_lowercase().as_str() {
        "wgs84" | "webmercator" | "4326" | "3857" => Ok(value.to_string()),
        _ => Err(RandomGeojsonError::InvalidArgument(
            "Coordinate system must be one of: WGS84, WebMercator, 4326, 3857".to_string(),
        )),
    }
}

fn random_property_value() -> serde_json::Value {
    let mut rng = rand::rng();
    match rng.random_range(0..3) {
        0 => serde_json::Value::Number(rng.random_range(0..1000).into()),
        1 => serde_json::Value::String(
            (0..rng.random_range(3..10))
                .map(|_| random_word::get(random_word::Lang::En))
                .collect::<Vec<_>>()
                .join(" "),
        ),
        2 => serde_json::Value::Bool(rng.random_bool(0.5)),
        _ => unreachable!(),
    }
}

// Saves the generated GeoJSON feature collection to a file.
fn save_geojson_to_file(
    fc: &FeatureCollection,
    file_path: &str,
    pretty: bool,
) -> RandomGeojsonResult<()> {
    let geojson_string = if pretty {
        serde_json::to_string_pretty(fc).map_err(|e| {
            RandomGeojsonError::InvalidArgument(format!("Failed to serialize GeoJSON: {}", e))
        })?
    } else {
        serde_json::to_string(fc).map_err(|e| {
            RandomGeojsonError::InvalidArgument(format!("Failed to serialize GeoJSON: {}", e))
        })?
    };

    std::fs::write(file_path, geojson_string)
        .map_err(|e| RandomGeojsonError::InvalidArgument(format!("Failed to write file: {}", e)))?;

    Ok(())
}
