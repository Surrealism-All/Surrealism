mod sql;
pub mod constants;
mod id;
mod value;
mod content;

pub use sql::*;
pub use value::*;
pub use id::*;
pub use content::ContentSet;


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