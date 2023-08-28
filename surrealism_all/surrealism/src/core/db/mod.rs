mod sql;
pub mod constants;
mod id;
mod value;
mod content;
mod strategy;
mod condition;
mod patch;
mod set;
mod field;
mod order;
pub mod functions;

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