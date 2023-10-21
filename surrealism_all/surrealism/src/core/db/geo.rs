use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use crate::core::db::SurrealValue;
use super::AdapterToValue;

/// # Geometries
/// - Point 	A geolocation point with latitude and longitude
/// - Line 	A GeoJSON LineString value for storing a geometric path
/// - Polygon 	A GeoJSON Polygon value for storing a geometric area
/// - MultiPoint 	A value which contains multiple geometry points
/// - MultiLine 	A value which contains multiple geometry lines
/// - MultiPolygon 	A value which contains multiple geometry polygons
/// - Collection 	A value which contains multiple different geometry types
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Geometries {
    Default,
    Point(Point),
    Line(Vec<Point>),
    Polygon(Vec<Point>),
    MultiPoint(Vec<Point>),
    MultiLine(Vec<Vec<Point>>),
    MultiPolygon(Vec<Vec<Point>>),
    Collection(Vec<Geometries>),
}

impl Geometries {
    pub fn point(&self, x: f32, y: f32) -> Self {
        Point::new(x, y).to_geo()
    }
    pub fn line(&self, points: Vec<Point>) -> Result<Geometries, &'static str> {
        //check pointer
        if check_points(&points) {
            return Err("first point == end point is not a line!");
        }
        Ok(Geometries::Line(points))
    }
    pub fn line_unchecked(&self, points: Vec<Point>) -> Self {
        Geometries::Line(points)
    }
    pub fn polygon(&self, points: Vec<Point>) -> Result<Geometries, &'static str> {
        if check_points(&points) {
            return Ok(Geometries::Polygon(points));
        }
        Err("first point != end point is not a polygon!")
    }
    pub fn polygon_unchecked(&self, points: Vec<Point>) -> Self {
        Geometries::Polygon(points)
    }
    pub fn multi_point(&self, points: Vec<Point>) -> Self {
        Geometries::MultiPoint(points)
    }
    pub fn multi_line(&self, points_list: Vec<Vec<Point>>) -> Result<Geometries, &'static str> {
        for points in &points_list {
            if check_points(points) {
                return Err(
                    "first point == end point is not a line!"
                );
            }
        }
        Ok(Geometries::MultiLine(points_list))
    }
    pub fn multi_line_unchecked(&self, points_list: Vec<Vec<Point>>) -> Self {
        Geometries::MultiLine(points_list)
    }
    pub fn multi_polygon(&self, points_list: Vec<Vec<Point>>) -> Result<Geometries, &'static str> {
        for points in &points_list {
            if check_points(points) == false {
                return Err(
                    "first point != end point is not a polygon!"
                );
            }
        }
        Ok(Geometries::MultiPolygon(points_list))
    }
    pub fn multi_polygon_unchecked(&self, points_list: Vec<Vec<Point>>) -> Self {
        Geometries::MultiPolygon(points_list)
    }
    pub fn collections(&self, collections: Vec<Geometries>) -> Self {
        Geometries::Collection(collections)
    }
}

impl AdapterToValue for Geometries {
    fn to_value(self) -> SurrealValue {
        SurrealValue::Geometries(self)
    }
}


impl Display for Geometries {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut json_type = "";
        let mut json = String::new();
        let mut json_data = "coordinates";
        match self {
            Geometries::Default => {
                panic!("Geometries::Default is not supported!");
            }
            Geometries::Point(point) => {
                json_type = "\"Point\"";
                json = point.parse();
            }
            Geometries::Line(line) => {
                json_type = "\"LineString\"";
                json = format!("[{}]", line.iter().map(|p| p.parse()).collect::<Vec<String>>().join(" , "));
            }
            Geometries::Polygon(pol) => {
                json_type = "\"Polygon\"";
                json = format!("[[{}]]", pol.iter().map(|p| p.parse()).collect::<Vec<String>>().join(" , "));
            }
            Geometries::MultiPoint(m_point) => {
                json_type = "\"MultiPoint\"";
                json = format!("[{}]", m_point.iter().map(|p| p.parse()).collect::<Vec<String>>().join(" , "));
            }
            Geometries::MultiLine(m_line) => {
                json_type = "\"MultiLineString\"";
                json = format!("[{}]", m_line.iter()
                    .map(|line| format!("[{}]", line.iter().map(|p| p.parse()).collect::<Vec<String>>().join(" , ")))
                    .collect::<Vec<String>>()
                    .join(" , "));
            }
            Geometries::MultiPolygon(m_pol) => {
                json_type = "\"MultiPolygon\"";
                json = format!("[{}]", m_pol.iter().map(|pol| format!("[[{}]]", pol.iter().map(|p| p.parse()).collect::<Vec<String>>().join(" , ")))
                    .collect::<Vec<String>>()
                    .join(" , "));
            }
            Geometries::Collection(collection) => {
                json_type = "\"GeometryCollection\"";
                json_data = "geometries";
                json = format!("[{}]", collection.iter().map(|geo| geo.to_string()).collect::<Vec<String>>().join(", "));
            }
        }
        write!(f, "{} type:{}, {}:{} {}", "{", json_type, json_data, json, "}")
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
}

/// # point
/// The simplest form of GeoJSON that SurrealDB supports is a geolocation point.
/// These can be written using two different formats.
/// The first format is a simple 2-element tuple (longitude, latitude).
impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point {
            x,
            y,
        }
    }
    /// convert Point to String
    pub fn parse(&self) -> String {
        format!("[{}, {}]", self.x, self.y)
    }
    pub fn to_geo(self) -> Geometries {
        Geometries::Point(self)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl From<(f32, f32)> for Point {
    fn from(value: (f32, f32)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<[f32; 2]> for Point {
    fn from(value: [f32; 2]) -> Self {
        Point {
            x: value[0],
            y: value[1],
        }
    }
}

/// Check whether points have same point
fn check_points(points: &Vec<Point>) -> bool {
    return if &points[0] == &points[points.len() - 1] {
        true
    } else {
        false
    };
}