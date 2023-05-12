///当使用#[dervice(ParseSQL)]时必须引入
///When using # [dervice (ParseSQL)], it must be introduced
pub trait SQLParser{
    fn parse_sql(&self)->String;
}
