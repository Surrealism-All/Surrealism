use surrealism::{SurrealismRes, Point, Geometries};

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let point = Geometries::Point(Point::from((-0.118092, 51.509865)));
    let line = Geometries::Line(vec![Point::from((10.0, 11.2)), Point::from((10.5, 11.9))]);
    let polygon = Geometries::Polygon(vec![
        Point::from((-0.38314819, 51.37692386)),
        Point::from((0.1785278, 51.37692386)),
        Point::from((0.1785278, 51.61460570)),
        Point::from((-0.38314819, 51.37692386)),
    ]);
    let multi_point = Geometries::MultiPoint(vec![Point::from((10.0, 11.2)), Point::from((10.5, 11.9))]);
    let multi_line = Geometries::MultiLine(vec![
        vec![Point::from((10.0, 11.2)), Point::from((10.5, 11.9))],
        vec![Point::from((11.0, 12.2)), Point::from((11.5, 12.9)), Point::from((12.0, 13.0))],
    ]);
    let multi_polygon = Geometries::MultiPolygon(vec![
        vec![Point::from((9.0, 11.2)), Point::from((10.5, 11.9)), Point::from((10.3, 13.0)), Point::from((9.0, 11.2))],
        vec![Point::from((10.0, 11.2)), Point::from((10.5, 11.9)), Point::from((10.8, 12.0)), Point::from((10.0, 11.2))],
    ]);
    let collections = Geometries::Collection(vec![multi_point, polygon, multi_polygon]);
    dbg!(point.to_string());
    dbg!(line.to_string());
    dbg!(polygon.to_string());
    dbg!(multi_point.to_string());
    dbg!(multi_line.to_string());
    dbg!(multi_polygon.to_string());
    dbg!(collections.to_string());

}