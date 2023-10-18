//! # Array Functions
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/27
//! @version:0.0.1
//! @description:
//! ```


use serde::Serialize;
use crate::db::{SurrealValue};

/// These functions can be used when working with, and manipulating arrays of data.
/// ## example
/// ```rust
/// use surrealism::db::{ SurrealID, SurrealValue, Array};
/// use surrealism::db::functions::Function;
/// use surrealism::surreal::SurrealismRes;
///
/// // [tests\src\main.rs:88] func_add = "array::add(['Java'], 'Rust')"
/// // [tests\src\main.rs:89] func_all = "array::all([5, true])"
/// // [tests\src\main.rs:90] func_any = "array::any([1, true])"
/// // [tests\src\main.rs:91] func_append = "array::append([1, 2], 5)"
/// // [tests\src\main.rs:92] func_combine = "array::combine(['Rust'], ['Surrealism'])"
/// // [tests\src\main.rs:93] func_complement = "array::complement([1, 2], [2, 3])"
/// // [tests\src\main.rs:94] func_union = "array::union([1, 2, 1, 6], [1, 3, 4, 5, 6])"
/// // [tests\src\main.rs:95] func_max = "array::max([1, 2, 5, 8, 9])"
/// #[tokio::main]
/// async fn main() -> SurrealismRes<()> {
///     let func_add = Function::array()
///         .add(vec![SurrealValue::from("Java")], "Rust");
///     let func_all = Function::array()
///         .all(vec![SurrealValue::from(5), SurrealValue::from(true)]);
///     let func_any = Function::array()
///         .any(vec![SurrealValue::from(1), SurrealValue::from(true)]);
///     let func_append = Function::array()
///         .append(vec![SurrealValue::from(1), SurrealValue::from(2)], 5);
///     let func_combine = Function::array()
///         .combine(vec![SurrealValue::from("Rust")], vec![SurrealValue::from("Surrealism")]);
///     let func_complement = Function::array()
///         .complement(vec![SurrealValue::from(1), SurrealValue::from(2)], vec![SurrealValue::from(2), SurrealValue::from(3)]);
///     //----------------------------------------------------------------------------
///     //use from_vec to convert SurrealValue::Array (Vec<SurrealValue>)
///     let func_union = Function::array()
///         .union(SurrealValue::from_vec(vec![1, 2, 1, 6]), SurrealValue::from_vec(vec![1, 3, 4, 5, 6]));
///     let func_max = Function::array().max(SurrealValue::from_vec(vec![1, 2, 5, 8, 9]));
///     Ok(())
/// }
/// ```
pub enum ArrayFunc {
    Default,
    /// Adds an item to an array if it doesn't exist
    Add,
    ///Checks whether all array values are truthy,
    All,
    ///Checks whether any array value is truthy
    Any,
    /// Appends an item to the end of an array
    Append,
    /// Combines all values from two arrays together
    Combine,
    /// Returns the complement of two arrays
    Complement,
    /// Returns the merged values from two arrays
    Concat,
    /// Returns the difference between two arrays
    Difference,
    /// Returns the unique items in an array
    Distinct,
    /// Flattens multiple arrays into a single array
    Flatten,
    /// Flattens and returns the unique items in an array
    Group,
    /// Inserts an item at the end of an array, or in a specific position
    Insert,
    /// Returns the values which intersect two arrays
    Intersect,
    /// Returns the length of an array
    Len,
    /// Returns the maximum item in an array
    Max,
    /// Returns the minimum item in an array
    Min,
    /// Returns the last item from an array
    Pop,
    /// Prepends an item to the beginning of an array
    Prepend,
    /// Appends an item to the end of an array
    Push,
    /// Removes an item at a specific position from an array
    Remove,
    /// Reverses the sorting order of an array
    Reverse,
    /// Sorts the values in an array in ascending or descending order
    Sort,
    /// Sorts the values in an array in ascending order
    SortASC,
    /// Sorts the values in an array in descending order
    SortDESC,
    /// Returns the unique merged values from two arrays
    Union,
}

impl Default for ArrayFunc {
    fn default() -> Self {
        ArrayFunc::Default
    }
}

impl ArrayFunc {
    /// # use add function
    /// Adds an item to an array if it doesn't exist
    /// this is a kind of easy way!
    pub fn add<T>(&self, core_value: Vec<SurrealValue>, value: T) -> String
        where
            T: Serialize,
    {
        self.single_core_value(core_value, value, "add")
    }
    /// # use all function
    /// Checks whether all array values are truthy
    pub fn all(&self, core_value: Vec<SurrealValue>) -> String {
        self.single_core(core_value, "all")
    }
    /// # use any function
    /// Checks whether any array value is truthy
    pub fn any(&self, core_value: Vec<SurrealValue>) -> String {
        self.single_core(core_value, "any")
    }
    /// # use append function
    /// appends a value to the end of an array
    pub fn append<T>(&self, core_value: Vec<SurrealValue>, value: T) -> String
        where
            T: Serialize,
    {
        self.single_core_value(core_value, value, "append")
    }
    /// # use combine function
    /// Combines all values from two arrays together
    pub fn combine(&self, core_value: Vec<SurrealValue>, value: Vec<SurrealValue>) -> String {
        self.double_core(core_value, value, "combine")
    }
    /// # use complement function
    /// Returns the complement of two arrays
    pub fn complement(&self, core_value: Vec<SurrealValue>, value: Vec<SurrealValue>) -> String {
        self.double_core(core_value, value, "complement")
    }
    /// # use concat function
    /// The array::concat function merges two arrays together, returning an array which may contain duplicate values. If you want to remove duplicate values from the resulting array, then use the array::union() function
    pub fn concat(&self, core_value: Vec<SurrealValue>, value: Vec<SurrealValue>) -> String {
        self.double_core(core_value, value, "concat")
    }
    /// # use difference function
    /// The array::difference determines the difference between two arrays, returning a single array containing items which are not in both arrays.
    pub fn difference(&self, core_value: Vec<SurrealValue>, value: Vec<SurrealValue>) -> String {
        self.double_core(core_value, value, "difference")
    }
    /// array::distinct
    /// The array::distinct function calculates the unique values in an array, returning a single array.
    pub fn distinct(&self, core_value: Vec<SurrealValue>) -> String {
        self.single_core(core_value, "distinct")
    }
    /// array::flatten
    /// The array::flatten flattens an array of arrays, returning a new array with all sub-array elements concatenated into it.
    pub fn flatten(&self, core_value: Vec<SurrealValue>, index: usize) -> String {
        self.core_index(core_value, index, "flatten")
    }
    /// array::group
    /// The array::group function flattens and returns the unique items in an array.
    pub fn group(&self, core_value: Vec<SurrealValue>) -> String {
        self.single_core(core_value, "group")
    }
    /// array::insert
    /// The array::insert function inserts a value into an array at a specific position.
    pub fn insert<T>(&self, core_value: Vec<SurrealValue>, value: T, index: usize) -> String
        where
            T: Serialize,
    {
        self.core_value_index(core_value, value, index, "insert")
    }
    /// array::intersect
    /// The array::intersect function calculates the values which intersect two arrays, returning a single array containing the values which are in both arrays.
    pub fn intersect(&self, core_value: Vec<SurrealValue>, value: Vec<SurrealValue>) -> String {
        self.double_core(core_value, value, "intersect")
    }
    /// array::len
    /// The array::len function calculates the length of an array, returning a number. This function includes all items when counting the number of items in the array. If you want to only count truthy values, then use the count() function.
    pub fn len(&self, core_value: Vec<SurrealValue>) -> String {
        self.single_core(core_value, "len")
    }
    pub fn max(&self, core_value: Vec<SurrealValue>) -> String {
        self.single_core(core_value, "max")
    }
    pub fn min(&self, core_value: Vec<SurrealValue>) -> String {
        self.single_core(core_value, "min")
    }
    pub fn pop(&self, core_value: Vec<SurrealValue>) -> String {
        self.single_core(core_value, "pop")
    }
    /// array::prepend
    /// The array::prepend function prepends a value to the end of an array.
    pub fn prepend<T>(&self, core_value: Vec<SurrealValue>, value: T) -> String
        where
            T: Serialize,
    {
        self.single_core_value(core_value, value, "prepend")
    }
    pub fn push<T>(&self, core_value: Vec<SurrealValue>, value: T) -> String
        where
            T: Serialize,
    {
        self.single_core_value(core_value, value, "push")
    }
    /// array::remove
    /// The array::remove function removes an item from a specific position in an array.
    pub fn remove(&self, core_value: Vec<SurrealValue>, index: usize) -> String {
        self.core_num(core_value, index, "remove")
    }
    pub fn reverse(&self, core_value: Vec<SurrealValue>) -> String {
        self.single_core(core_value, "reverse")
    }
    pub fn sort(&self, core_value: Vec<SurrealValue>) -> String {
        self.single_core(core_value, "sort")
    }
    pub fn sort_asc(&self, core_value: Vec<SurrealValue>) -> String {
        let core_value = SurrealValue::from(core_value);
        format!("array::sort({}, true)", core_value.to_str())
    }
    pub fn sort_desc(&self, core_value: Vec<SurrealValue>) -> String {
        let core_value = SurrealValue::from(core_value);
        format!("array::sort({}, false)", core_value.to_str())
    }
    /// array::union
    /// The array::union function combines two arrays together, removing duplicate values, and returning a single array.
    pub fn union(&self, core_value: Vec<SurrealValue>, value: Vec<SurrealValue>) -> String {
        self.double_core(core_value, value, "union")
    }

    fn core_value_index<T>(&self, core_value: Vec<SurrealValue>, value: T, index: usize, f_name: &str) -> String
        where
            T: Serialize,
    {
        let core_value = SurrealValue::from(core_value);
        let value = SurrealValue::from_each(value);
        format!("array::{}({}, {}, {})", f_name, core_value.to_str(), value.to_str(), index)
    }
    fn core_num(&self, core_value: Vec<SurrealValue>, index: usize, f_name: &str) -> String {
        let core_value = SurrealValue::from(core_value);
        format!("array::{}({}, {})", f_name, core_value.to_str(), index)
    }
    fn core_index(&self, core_value: Vec<SurrealValue>, index: usize, f_name: &str) -> String {
        let index = core_value[index].clone();
        let core_value = SurrealValue::from(core_value);
        format!("array::{}({}, {})", f_name, core_value.to_str(), index.to_str())
    }
    fn double_core(&self, core_value: Vec<SurrealValue>, value: Vec<SurrealValue>, f_name: &str) -> String {
        let core_value = SurrealValue::from(core_value);
        let value = SurrealValue::from(value);
        format!("array::{}({}, {})", f_name, core_value.to_str(), value.to_str())
    }
    fn single_core_value<T>(&self, core_value: Vec<SurrealValue>, value: T, f_name: &str) -> String
        where
            T: Serialize,
    {
        let core_value = SurrealValue::from(core_value);
        let value = SurrealValue::from_each(value);
        format!("array::{}({}, {})", f_name, core_value.to_str(), value.to_str())
    }
    /// core_value function
    fn single_core(&self, core_value: Vec<SurrealValue>, f_name: &str) -> String {
        let core_value = SurrealValue::from(core_value);
        format!("array::{}({})", f_name, core_value.to_str())
    }
    fn check_array<F>(&self, core_value: &SurrealValue, f: F) -> String where F: Fn() -> String, {
        return if core_value.is_array() {
            f()
        } else {
            panic!("core_value need SurrealValue::Array")
        };
    }
}
