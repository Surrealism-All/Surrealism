//! # Array Functions
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/27
//! @version:0.0.1
//! @description:
//! ```


use serde::Serialize;
use crate::SurrealValue;

/// These functions can be used when working with, and manipulating arrays of data.
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
    Len(),
    /// Returns the maximum item in an array
    Max(),
    /// Returns the minimum item in an array
    Min(),
    /// Returns the last item from an array
    Pop(),
    /// Prepends an item to the beginning of an array
    Prepend(),
    /// Appends an item to the end of an array
    Push(),
    /// Removes an item at a specific position from an array
    Remove(),
    /// Reverses the sorting order of an array
    Reverse(),
    /// Sorts the values in an array in ascending or descending order
    Sort(),
    /// Sorts the values in an array in ascending order
    SortASC(),
    /// Sorts the values in an array in descending order
    SortDESC(),
    /// Returns the unique merged values from two arrays
    Union(),
}

impl Default for ArrayFunc {
    fn default() -> Self {
        ArrayFunc::Default
    }
}

impl ArrayFunc {
    /// # use add function (Recommended)
    /// Adds an item to an array if it doesn't exist
    /// this is a kind of easy way!
    /// ## example
    /// ```rust
    /// use surrealism::functions::Function;
    /// let func3 = Function::array().add(vec!["hello", "world"], "Rust");
    /// ```
    pub fn add<T, O>(&self, core_value: T, value: O) -> String where T: Serialize, O: Serialize, {
        self.add_from(SurrealValue::from(serde_json::to_value(core_value).unwrap()), SurrealValue::from(serde_json::to_value(value).unwrap()))
    }
    /// # use add function (Basic)
    /// Adds an item to an array if it doesn't exist
    /// ## example
    /// ```rust
    /// use surrealism::functions::Function;
    ///     let func1 = Function::array().add_from(SurrealValue::from(vec!["hello", "world"]), SurrealValue::from("Rust"));
    ///     let func2 = Function::array().add_from(SurrealValue::from(vec![SurrealValue::from(5), SurrealValue::from(true)]), SurrealValue::from("Rust"));
    /// ```
    pub fn add_from(&self, core_value: SurrealValue, value: SurrealValue) -> String {
        return if core_value.is_array() {
            format!("array::add({}, {})", core_value.to_str(), value.to_str())
        } else {
            panic!("core_value need SurrealValue::Array")
        };
    }
}
