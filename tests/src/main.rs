use surrealism::{DefaultInitService, InitService, SurrealismConnector, SurrealismRes, SurrealValue, UseNSDB, Point, Geometries, SurrealismCommit, ReturnType};
use surrealism::builder::{BaseWrapperImpl, ReturnImpl, SQLBuilderFactory, TableImpl};
use surrealism::builder::create::CreateWrapperImpl;

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let point = Geometries::Point(Point::from((-0.118092, 51.509865)));
    // let line = Geometries::Line(vec![Point::from((10.0, 11.2)), Point::from((10.5, 11.9))]);
    // let polygon = Geometries::Polygon(vec![
    //     Point::from((-0.38314819, 51.37692386)),
    //     Point::from((0.1785278, 51.37692386)),
    //     Point::from((0.1785278, 51.61460570)),
    //     Point::from((-0.38314819, 51.37692386)),
    // ]);
    // let multi_point = Geometries::MultiPoint(vec![Point::from((10.0, 11.2)), Point::from((10.5, 11.9))]);
    // let multi_line = Geometries::MultiLine(vec![
    //     vec![Point::from((10.0, 11.2)), Point::from((10.5, 11.9))],
    //     vec![Point::from((11.0, 12.2)), Point::from((11.5, 12.9)), Point::from((12.0, 13.0))],
    // ]);
    // let multi_polygon = Geometries::MultiPolygon(vec![
    //     vec![Point::from((9.0, 11.2)), Point::from((10.5, 11.9)), Point::from((10.3, 13.0)), Point::from((9.0, 11.2))],
    //     vec![Point::from((10.0, 11.2)), Point::from((10.5, 11.9)), Point::from((10.8, 12.0)), Point::from((10.0, 11.2))],
    // ]);
    // let collections = Geometries::Collection(vec![multi_point, polygon, multi_polygon]);
    // dbg!(point.to_string());
    // dbg!(line.to_string());
    // dbg!(polygon.to_string());
    // dbg!(multi_point.to_string());
    // dbg!(multi_line.to_string());
    // dbg!(multi_polygon.to_string());
    // dbg!(collections.to_string());

    // [tests\src\main.rs:40] commit1 = Response(
    //     {
    //         0: Ok(
    //             [
    //                 Object(
    //                     Object(
    //         {
    //             "center": Geometry(
    //                 Point(
    //                     Point(
    //                         Coord {
    //             x: -0.118092,
    //             y: 51.509865,
    //         },
    //             ),
    //             ),
    //             ),
    //             "id": Thing(
    //                 Thing {
    //             tb: "city",
    //             id: String(
    //             "fhao6bgiwahmo6x5tiq1",
    //             ),
    //         },
    //             ),
    //         },
    //         ),
    //         ),
    //         ],
    //         ),
    //     },
    // )

    let mut service = DefaultInitService::new().init();
    let _ = service.use_commit("test", "test").await?;
    let create = SQLBuilderFactory::create().table("city").set()
        .add_from_value("center", SurrealValue::Geometries(point))
        .return_type(ReturnType::After).build();
    let commit1 = service.commit_sql(&create).await?;
    dbg!(commit1);
    Ok(())
}


// use surrealism::{DefaultInitService, InitService, SurrealID, SurrealismCommit, SurrealismConnector, SurrealismRes, Table, UseNSDB, parse_response};
// use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
// use surrealism::builder::create::{CreateWrapper, CreateWrapperImpl};
// use serde::{Serialize, Deserialize};
// use surrealism::builder::select::SelectWrapperImpl;
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct User {
//     username: String,
//     pwd:String,
//     male: bool,
//     age: u8,
// }
//
// /// create a new user table
// /// table_name:user
// /// table_id:surrealism
// pub fn crate_user_table() -> CreateWrapper {
//     // create a user data
//     let user = User {
//         username: "Tobie".to_string(),
//         pwd: "Tobie001".to_string(),
//         male: true,
//         age: 23,
//     };
//     // create table with content
//     let user_table = SQLBuilderFactory::create()
//         .table("user")
//         .id(SurrealID::from("surrealism"))
//         .content(&user)
//         .deref_mut();
//     user_table
// }
//
// #[tokio::main]
// async fn main() -> SurrealismRes<()> {
//     // init service
//     let mut service = DefaultInitService::new().init();
//     // use ns:test and db:test
//     let _ = service.use_commit("test", "test").await?;
//     // get info from surrealdb
//     // let info = SQLBuilderFactory::info().db().build();
//     // let info_res = service.commit_sql(&info).await?;
//     // dbg!(info_res);
//     // create a table
//     // let create_stmt = crate_user_table().build();
//     // let create_res = service.commit_sql(&create_stmt).await?;
//     // dbg!(create_res);
//     // select user::surrealism table
//     let select = SQLBuilderFactory::select().table("user").id(SurrealID::from("surrealism")).column("*").build();
//     let select_res = service.commit_sql(&select).await?;
//     //parse response to any type you want
//     let res: User = parse_response(select_res);
//     // [tests\src\main.rs:55] res = User {
//     //     username: "Tobie",
//     //     pwd: "Tobie001",
//     //     male: true,
//     //     age: 23,
//     // }
//     dbg!(&res);
//     Ok(())
// }
//
