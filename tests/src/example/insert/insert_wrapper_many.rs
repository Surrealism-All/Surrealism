use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, InsertWrapper, UseWrapper};
use serde::{Serialize, Deserialize};

///构建结构体,需要使用surrealism提供的宏:ParseSQL
/// build struct,Need to use the macro provided by surrealism: ParseSQL
/// 请注意:ParseSQl宏和SQLParser trait要同时导入
/// Please note that the ParseSQl macro and SQLParser trait need to be imported simultaneously
#[derive(Debug, Serialize, Deserialize)]
struct User {
    pub userId: String,
    pub name: String,
    pub age: usize,
}


#[tokio::main]
async fn main() -> SurrealRes<()> {
    ///初始化连接
    ///init connection
    let db = DefaultInitServiceImpl::new().init().unwrap();
    ///创建UseWrapper
    /// new UseWrapper
    let mut use_wrapper = UseWrapper::new();
    /// 设置命名空间和数据库
    /// Set namespace and database
    use_wrapper.use_ns("test").use_db("test");
    /// 提交语句
    /// commit statement
    let res_use = db.use_commit(use_wrapper).await;
    dbg!(res_use);
    ///准备数据
    let data1 = User {
        userId: "noob001".to_string(),
        name: "Noob".to_string(),
        age: 16,
    };
    let data2 = User {
        userId: "noob002".to_string(),
        name: "Noob".to_string(),
        age: 26,
    };
    let mut data_list = vec![&data1, &data2];
    ///构建InsertWrapper
    /// 通过键值对形式构建传统语句
    ///INSERT INTO user  [ {userId:'noob001',name:'Noob',age:16} , {userId:'noob002',name:'Noob',age:26} ];
    let mut insert_wrapper = InsertWrapper::new();
    insert_wrapper
        .insert_into("user")
        .insert_many(&data_list);
    /// 提交语句
    /// commit statement
    let create_res = db.commit(insert_wrapper).await;
    dbg!(create_res.unwrap());
    Ok(())
}