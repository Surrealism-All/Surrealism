use surrealism::DefaultRes;
use surrealism::db::{Geometries, AdapterToValue, Point, SurrealValue};

// [tests\src\main.rs:32] geo_point.to_string() = "{ type:\"Point\", coordinates:[12, 16.6] }"
// [tests\src\main.rs:33] geo_line.to_string() = "{ type:\"LineString\", coordinates:[[10, 11.2] , [11, 11.2]] }"
// [tests\src\main.rs:34] geo_polygon.to_string() = "{ type:\"Polygon\", coordinates:[[[10, 11.2] , [11, 11.2] , [10, 11.2]]] }"
// [tests\src\main.rs:35] geo_multi_points.to_string() = "{ type:\"MultiPoint\", coordinates:[[10, 11.2] , [11, 11.2]] }"
// [tests\src\main.rs:36] geo_multi_line.to_value().to_string() = "{ type:\"MultiLineString\", coordinates:[[[10, 11.2] , [10.5, 11.9]] , [[11, 12.2] , [11.5, 12.9] , [12, 13]]]}"
// [tests\src\main.rs:37] geo_multi_polygon.to_string() = "{ type:\"MultiPolygon\", coordinates:[[[[9, 11.2] , [10.5, 11.9] , [10.3, 13] , [9, 11.2]]] , [[[10, 11.2] , [10.5, 11.
// 9] , [10.8, 12] , [10, 11.2]]]] }"
// [tests\src\main.rs:38] collection.to_string() = "{ type:\"GeometryCollection\", geometries:[{ type:\"LineString\", coordinates:[[10, 11.2] , [11, 11.2]] }, { type:\"MultiLineS
// tring\", coordinates:[[[10, 11.2] , [10.5, 11.9]] , [[11, 12.2] , [11.5, 12.9] , [12, 13]]] }] }"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let geo_point = SurrealValue::geo().point(12.0, 16.6).to_value();
    let geo_line = SurrealValue::geo()
        .line(vec![Point::new(10.0, 11.2), Point::from((11.0, 11.2))])
        .unwrap();
    let geo_polygon = SurrealValue::geo()
        .polygon(vec![Point::new(10.0, 11.2), Point::from((11.0, 11.2)), Point::new(10.0, 11.2)])
        .unwrap()
        .to_value();
    let geo_multi_points = SurrealValue::geo()
        .multi_point(vec![Point::new(10.0, 11.2), Point::from((11.0, 11.2))])
        .to_value();
    let geo_multi_line = SurrealValue::geo().multi_line(
        vec![
            vec![Point::from((10.0, 11.2)), Point::from((10.5, 11.9))],
            vec![Point::from((11.0, 12.2)), Point::from((11.5, 12.9)), Point::from((12.0, 13.0))],
        ]
    ).unwrap();
    let geo_multi_polygon = SurrealValue::geo().multi_polygon(
        vec![
            vec![Point::from((9.0, 11.2)), Point::from((10.5, 11.9)), Point::from((10.3, 13.0)), Point::from((9.0, 11.2))],
            vec![Point::from((10.0, 11.2)), Point::from((10.5, 11.9)), Point::from((10.8, 12.0)), Point::from((10.0, 11.2))],
        ]
    ).unwrap().to_value();
    let collection = SurrealValue::geo().collections(
        vec![geo_line.clone(),geo_multi_line.clone()]
    ).to_value();
    dbg!(geo_point.to_string());
    dbg!(geo_line.to_string());
    dbg!(geo_polygon.to_string());
    dbg!(geo_multi_points.to_string());
    dbg!(geo_multi_line.to_value().to_string());
    dbg!(geo_multi_polygon.to_string());
    dbg!(collection.to_string());
    Ok(())
}