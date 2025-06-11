# random-geojson

**random-geojson** is a command-line tool written in Rust for generating random [GeoJSON](https://geojson.org/) geometries. It supports multiple geometry types (`Point`, `LineString`, `Polygon`, and `All`) and coordinate systems (`WGS84`, `WebMercator`, `4326`, `3857`). The tool can generate a collection of random features with customizable properties and output them to a file in either compact or pretty-printed JSON.

## Features

- Generates random GeoJSON `FeatureCollection`
- Supports `Point`, `LineString`, `Polygon`, and `All` (all geometry types)
- Output geometries in either `WGS84` (EPSG:4326) or `Web Mercator` (EPSG:3857)
- Randomized properties with string, number, and boolean values
- Configurable number of features and properties
- Output to a file in compact or pretty-printed format

## Usage

```
random-geojson [OPTIONS]
```

### Options

- `--num-columns <NUM_COLUMNS>`  
  Number of properties (columns) to generate for each feature (default: 0)

- `--length <LENGTH>`  
  Number of features to generate (default: 100)

- `--geometry-type <GEOMETRY_TYPE>`  
  Type of geometry to generate. Possible values: `Point`, `LineString`, `Polygon`, `All` (default: `All`)

- `--coordinate-system <COORDINATE_SYSTEM>`  
  Coordinate system to use. Possible values: `WGS84`, `WebMercator`, `4326`, `3857` (default: `WGS84`)

- `--pretty`  
  Output GeoJSON in pretty-printed format (default: false)

- `--output-file <OUTPUT_FILE>`  
  File name to save the generated GeoJSON (default: `random.geojson`)

### Example

Generate 10 random points in WGS84 with 3 properties per feature and pretty-printed output:

```
random-geojson --length 10 --geometry-type Point --num-columns 3 --pretty -o mydata.geojson
```

## Output

The tool generates a valid GeoJSON `FeatureCollection` with the specified number of features and properties. Each feature has a unique UUID as its `id`, random geometry, and random property values.

## License

Licensed under [MIT](LICENSE)

---

*This tool is intended for generating synthetic spatial data for testing, development, and demonstration purposes.*