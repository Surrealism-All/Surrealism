use surrealism::{SurrealismRes, SurrealID, SurrealValue, Array};
use surrealism::functions::Function;

// [tests\src\main.rs:88] func_add = "array::add(['Java'], 'Rust')"
// [tests\src\main.rs:89] func_all = "array::all([5, true])"
// [tests\src\main.rs:90] func_any = "array::any([1, true])"
// [tests\src\main.rs:91] func_append = "array::append([1, 2], 5)"
// [tests\src\main.rs:92] func_combine = "array::combine(['Rust'], ['Surrealism'])"
// [tests\src\main.rs:93] func_complement = "array::complement([1, 2], [2, 3])"
// [tests\src\main.rs:94] func_union = "array::union([1, 2, 1, 6], [1, 3, 4, 5, 6])"
// [tests\src\main.rs:95] func_max = "array::max([1, 2, 5, 8, 9])"
#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let func_add = Function::array()
        .add(vec![SurrealValue::from("Java")], "Rust");
    let func_all = Function::array()
        .all(vec![SurrealValue::from(5), SurrealValue::from(true)]);
    let func_any = Function::array()
        .any(vec![SurrealValue::from(1), SurrealValue::from(true)]);
    let func_append = Function::array()
        .append(vec![SurrealValue::from(1), SurrealValue::from(2)], 5);
    let func_combine = Function::array()
        .combine(vec![SurrealValue::from("Rust")], vec![SurrealValue::from("Surrealism")]);
    let func_complement = Function::array()
        .complement(vec![SurrealValue::from(1), SurrealValue::from(2)], vec![SurrealValue::from(2), SurrealValue::from(3)]);
    //----------------------------------------------------------------------------
    //use from_vec to convert SurrealValue::Array (Vec<SurrealValue>)
    let func_union = Function::array()
        .union(SurrealValue::from_vec(vec![1, 2, 1, 6]), SurrealValue::from_vec(vec![1, 3, 4, 5, 6]));
    let func_max = Function::array().max(SurrealValue::from_vec(vec![1, 2, 5, 8, 9]));

    dbg!(func_add);
    dbg!(func_all);
    dbg!(func_any);
    dbg!(func_append);
    dbg!(func_combine);
    dbg!(func_complement);
    dbg!(func_union);
    dbg!(func_max);
    Ok(())
}