// use crate::creator::entities::select_wrapper::TableRegion;
//
// ///=================================================<br>
// /// @params:
// /// <ol>
// ///     <li>keyword:关键字</li>
// ///     <li>stmt:最终语句</li>
// ///     <li>table_region:表区域（生成FROM @TableName）</li>
// ///     <li>field_region:字段区域（生成SELECT @Fields...）</li>
// ///     <li>where_region:条件区域，where子句（生成WHERE @Condition）</li>
// ///     <li>handle_region:处理区域（生成ORDER BY，GROUP BY等子句）</li>
// /// </ol>
// /// @date:2023/5/27<br>
// /// @description:Select语句包装器,生成select语句，实现查询数据操作<br>
// /// @example:<br>
// ///=================================================
// #[derive(Debug, Clone)]
// pub struct SelectWrapper {
//     ///关键字
//     keyword: String,
//     available: Vec<AvailData>,
//     stmt: String,
//     ///表区域（生成INTO @TableName）
//     table_region:TableRegion ,
//     ///字段区域（生成SELECT @Fields...）
//     field_region: FieldRegion,
//     ///条件区域，where子句（生成WHERE @Condition）
//     where_region: WhereRegion,
//     ///处理区域（生成ORDER BY，GROUP BY等子句）
//     handle_region: HandleRegion,
//
// }