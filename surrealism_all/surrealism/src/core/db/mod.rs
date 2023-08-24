mod sql;
pub mod constants;
mod id;
mod value;
mod content;
mod strategy;
mod condition;
mod patch;

pub use sql::*;
pub use value::*;
pub use id::*;
pub use condition::*;
pub use content::ContentSet;
pub use patch::Patch;


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