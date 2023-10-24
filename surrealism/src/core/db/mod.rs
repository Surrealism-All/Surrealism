mod sql;
pub mod constants;
mod id;
mod value;
mod content;
mod strategy;
mod condition;
mod patch;
mod set;
mod decimal;
mod field;
mod order;
mod geo;
pub mod functions;
mod duration_adapter;
mod datetime_adapter;
mod with;

pub use sql::*;
pub use value::*;
pub use id::*;
pub use condition::*;
pub use content::ContentSet;
pub use patch::Patch;
pub use set::Set;
pub use strategy::*;
pub use order::Order;
pub use field::Field;
pub use geo::*;
pub use decimal::Decimal;
pub use duration_adapter::DurationAdapter;
pub use datetime_adapter::DatetimeAdapter;
pub use with::With;


/// Trans Adapter to SurrealValue
pub trait AdapterToValue{
    fn to_value(self)->SurrealValue;
}


/// # param combine trait
/// 对参数进行组合
pub trait ParamCombine {
    fn combine(&self) -> String;
}

// #[macro_export]
// macro_rules! is_enum_match {
//     (
//         $(($Name:tt,$E:expr))*
//     ) => (
//         $(
//             pub fn $Name(&self) -> bool {
//                 match self {
//                     $E => true,
//                     _ =>false
//                 }
//             }
//         )*
//     )
// }