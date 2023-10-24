# Surrealism V0.3+

## Surreal

## Features

```toml
default = ["builder"]
row = []
builder = []
surreal = ["builder"]
full = ["row", "builder", "surreal"]
```

### Configuration配置文件

<form>
    <input type="checkbox" checked disabled />  <strong>Surrealism.json支持</strong> <br />
    <input type="checkbox" checked disabled />  <strong>Surrealism.toml支持</strong> <br />
    <input type="checkbox" />  <strong>自定义构建支持(SurrealismConfig)</strong> <br />
</form>

### Init 初始化服务

<form>
    <input type="checkbox" checked disabled />  <strong>DefaultInitService 默认初始化服务的支持</strong> <br />
    <input type="checkbox" />  <strong>自定义初始化服务的支持</strong> <br />
</form>

### ID 表ID

<form>
    <input type="checkbox" checked disabled />  <strong>SurrealID::Default的支持</strong> <br />
    <input type="checkbox" checked disabled/>  <strong>SurrealID::Int的支持</strong> <br />
    <input type="checkbox" checked disabled/>  <strong>SurrealID::Float的支持</strong> <br />
    <input type="checkbox" checked disabled/>  <strong>SurrealID::String的支持</strong> <br />
    <input type="checkbox" checked disabled/>  <strong>SurrealID::Array的支持</strong> <br />
    <input type="checkbox" checked disabled/>  <strong>SurrealID::UUID的支持</strong> <br />
    <input type="checkbox" checked disabled/>  <strong>SurrealID::ULID的支持</strong> <br />
    <input type="checkbox" checked disabled/>  <strong>SurrealID::RAND的支持</strong> <br />
    <input type="checkbox" checked disabled/>  <strong>SurrealID::Range的支持</strong> <br />
</form>

### Value 数据类型

<form>
    <input type="checkbox" checked disabled />  <strong>SurrealValue::None的支持</strong> <br />
    <input type="checkbox" checked disabled />  <strong>SurrealValue::Null的支持</strong> <br />
    <input type="checkbox" checked disabled />  <strong>SurrealValue::Int的支持</strong> <br />
    <input type="checkbox" checked disabled />  <strong>SurrealValue::Float的支持</strong> <br />
    <input type="checkbox" checked disabled />  <strong>SurrealValue::Decimal的支持</strong> <br />
    <input type="checkbox" checked disabled />  <strong>SurrealValue::String的支持</strong> <br />
    <input type="checkbox" checked disabled />  <strong>SurrealValue::Object的支持</strong> <br />
    <input type="checkbox" checked disabled />  <strong>SurrealValue::Datetime的支持(DatetimeAdapter)</strong> <br />
    <input type="checkbox" checked disabled />  <strong>SurrealValue::Duration的支持(DurationAdapter)</strong> <br />
    <input type="checkbox" checked disabled />  <strong>SurrealValue::Array的支持</strong> <br />
    <input type="checkbox" />  <strong>SurrealValue::Set的支持</strong> <br />
    <input type="checkbox" />  <strong>SurrealValue::Option的支持</strong> <br />
    <input type="checkbox" checked disabled />  <strong>SurrealValue::Geo的支持</strong> <br />
    <input type="checkbox" />  <strong>SurrealValue::Record的支持</strong> <br />
    <input type="checkbox" />  <strong>SurrealValue::Future的支持</strong> <br />
    <input type="checkbox" checked disabled />  <strong>数学常数构建</strong> <br />
    <input type="checkbox" checked disabled />  <strong>数学常数支持</strong> <br />
</form>

## Builder

<form>
    <input type="checkbox" checked disabled />  <strong>USE STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>CREATE STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>SELECT STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>LIVE SELECT STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>RELATE STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>UPDATE STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>INSERT STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>DELETE STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>INFO STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>TRANSACTION STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>DEFINE STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>REMOVE STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>SLEEP STMT</strong> <br />
    <input type="checkbox" disabled />  <strong>LET STMT</strong> <br />
    <input type="checkbox" disabled />  <strong>BEGIN STMT</strong> <br />
    <input type="checkbox" disabled />  <strong>CANCEL STMT</strong> <br />
    <input type="checkbox" disabled />  <strong>COMMIT STMT</strong> <br />
    <input type="checkbox" disabled />  <strong>IF ELSE STMT</strong> <br />
    <input type="checkbox" disabled />  <strong>FOR STMT</strong> <br />
    <input type="checkbox" disabled />  <strong>BREAK STMT</strong> <br />
    <input type="checkbox" disabled />  <strong>CONTINUE STMT</strong> <br />
    <input type="checkbox" disabled />  <strong>KILL STMT</strong> <br />
    <input type="checkbox" disabled />  <strong>THROW STMT</strong> <br />
</form>


### Use

<form>
    <input type="checkbox" checked disabled />  <strong>USE NS STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>USE DB STMT</strong> <br />
</form>

### Create

<form>
    <input type="checkbox" checked disabled />  <strong>CREATE CONTENT STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>CREATE SET STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>RETURN STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>TIMEOUT STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>PARALLEL STMT</strong> <br />
</form>

### Insert

<form>
    <input type="checkbox" checked disabled />  <strong>INSERT INTO STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>ON DUPLICATE KEY UPDATE STMT</strong> <br />
</form>
### Select

<form>
    <input type="checkbox" checked disabled />  <strong>FIELD</strong> <br />
    <input type="checkbox" checked disabled />  <strong>OMIT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>WITH INDEX|NOINDEX</strong> <br />
    <input type="checkbox" checked disabled />  <strong>FROM</strong> <br />
    <input type="checkbox" checked disabled />  <strong>WHERE</strong> <br />
    <input type="checkbox" checked disabled />  <strong>SPLIT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>GROUP</strong> <br />
    <input type="checkbox" checked disabled />  <strong>ORDER</strong> <br />
    <input type="checkbox" checked disabled />  <strong>LIMIT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>START</strong> <br />
    <input type="checkbox" checked disabled />  <strong>FETCH</strong> <br />
    <input type="checkbox" checked disabled />  <strong>TIMEOUT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>PARALLEL</strong> <br />
    <input type="checkbox" checked disabled />  <strong>EXPLAIN [FULL]</strong> <br />
</form>

### Live Select

<form>
    <input type="checkbox" checked disabled />  <strong>FIELD</strong> <br />
    <input type="checkbox" checked disabled />  <strong>FROM</strong> <br />
    <input type="checkbox" checked disabled />  <strong>WHERE</strong> <br />
    <input type="checkbox" checked disabled />  <strong>FETCH</strong> <br />
</form>

### Delete

<form>
    <input type="checkbox" checked disabled />  <strong>DELETE WHERE</strong> <br />
    <input type="checkbox" checked disabled />  <strong>RETURN</strong> <br />
    <input type="checkbox" checked disabled />  <strong>TIMEOUT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>PARALLEL</strong> <br />
    <input type="checkbox" disabled />  <strong>DELETE WITH RELETE</strong> <br />
</form>


### Remove

<form>
    <input type="checkbox" checked disabled />  <strong>NAMESPACE</strong> <br />
    <input type="checkbox" checked disabled />  <strong>DATABASE</strong> <br />
    <input type="checkbox" checked disabled />  <strong>USER</strong> <br />
    <input type="checkbox" checked disabled />  <strong>LOGIN</strong> <br />
    <input type="checkbox" checked disabled />  <strong>TOKEN</strong> <br />
    <input type="checkbox" checked disabled />  <strong>SCOPE</strong> <br />
    <input type="checkbox" checked disabled />  <strong>TABLE</strong> <br />
    <input type="checkbox" checked disabled />  <strong>EVENT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>FUNCTION</strong> <br />
    <input type="checkbox" checked disabled />  <strong>FIELD</strong> <br />
    <input type="checkbox" checked disabled />  <strong>INDEX</strong> <br />
    <input type="checkbox" checked disabled />  <strong>PARAM</strong> <br />
</form>
### Update

<form>
    <input type="checkbox" checked disabled />  <strong>CONTENT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>MERGE</strong> <br />
    <input type="checkbox" checked disabled />  <strong>PATCH</strong> <br />
    <input type="checkbox" checked disabled />  <strong>SET</strong> <br />
    <input type="checkbox" checked disabled />  <strong>WHERE</strong> <br />
    <input type="checkbox" checked disabled />  <strong>RETURN</strong> <br />
    <input type="checkbox" checked disabled />  <strong>TIMEOUT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>PARALLEL</strong> <br />
</form>

### Define

<form>
    <input type="checkbox" checked disabled />  <strong>NAMESPACE</strong> <br />
    <input type="checkbox" checked disabled />  <strong>DATABASE</strong> <br />
    <input type="checkbox" checked disabled />  <strong>USER</strong> <br />
    <input type="checkbox" checked disabled />  <strong>LOGIN</strong> <br />
    <input type="checkbox" checked disabled />  <strong>TOKEN</strong> <br />
    <input type="checkbox" checked disabled />  <strong>SCOPE</strong> <br />
    <input type="checkbox" checked disabled />  <strong>TABLE</strong> <br />
    <input type="checkbox" checked disabled />  <strong>EVENT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>FUNCTION</strong> <br />
    <input type="checkbox" checked disabled />  <strong>FIELD</strong> <br />
    <input type="checkbox" checked disabled />  <strong>INDEX</strong> <br />
    <input type="checkbox" checked disabled />  <strong>PARAM</strong> <br />
    <input type="checkbox" disabled />  <strong>ANALYZER</strong> <br />
</form>

### Info

<form>
    <input type="checkbox" checked disabled />  <strong>KV</strong> <br />
    <input type="checkbox" checked disabled />  <strong>NS</strong> <br />
    <input type="checkbox" checked disabled />  <strong>DB</strong> <br />
    <input type="checkbox" checked disabled />  <strong>SCOPE</strong> <br />
    <input type="checkbox" checked disabled />  <strong>TABLE</strong> <br />
</form>

### Show

<form>
    <input type="checkbox" checked disabled />  <strong>SINCE</strong> <br />
    <input type="checkbox" checked disabled />  <strong>LIMIT</strong> <br />
</form>

### Sleep

<form>
    <input type="checkbox" checked disabled />  <strong>Duration</strong> <br />
</form>

## Assert

<form>
    <input type="checkbox"  disabled />  <strong>ASSERT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>WHERE</strong> <br />
    <input type="checkbox" checked disabled />  <strong>Condition</strong> <br />
    <input type="checkbox" checked disabled />  <strong>Criteria</strong> <br />
</form>

## Functions

<form>
    <input type="checkbox" checked disabled />  <strong>Array</strong> <br />
    <input type="checkbox" checked disabled />  <strong>Count</strong> <br />
    <input type="checkbox" checked disabled />  <strong>Crypto</strong> <br />
    <input type="checkbox" disabled />  <strong>Duration</strong> <br />
    <input type="checkbox" disabled />  <strong>Geo</strong> <br />
    <input type="checkbox" disabled />  <strong>HTTP</strong> <br />
    <input type="checkbox" disabled />  <strong>Math</strong> <br />
    <input type="checkbox" disabled />  <strong>Meta</strong> <br />
    <input type="checkbox" disabled />  <strong>Parse</strong> <br />
    <input type="checkbox" disabled />  <strong>Rand</strong> <br />
    <input type="checkbox" disabled />  <strong>Search</strong> <br />
    <input type="checkbox" disabled />  <strong>Session</strong> <br />
    <input type="checkbox" disabled />  <strong>Sleep</strong> <br />
    <input type="checkbox" disabled />  <strong>String</strong> <br />
    <input type="checkbox" disabled />  <strong>Time</strong> <br />
    <input type="checkbox" disabled />  <strong>Type</strong> <br />
    <input type="checkbox" disabled />  <strong>Scripting</strong> <br />
    <input type="checkbox" disabled />  <strong>Vector</strong> <br />
</form>

## Row

<form>
    <input type="checkbox" checked disabled />  <strong>RowSql的支持</strong> <br />
    <input type="checkbox" checked disabled/>  <strong>row_sql!宏</strong> <br />
</form>
## Operators

| Operator     | Description                                                  | Finish |
| ------------ | ------------------------------------------------------------ | ------ |
| && or AND    | Checks whether both of two values are truthy                 | ✅      |
| \|\| or OR   | Checks whether either of two values is truthy                | ✅      |
| ??           | Check whether either of two values are truthy and not `NULL` | ⛔      |
| ?:           | Check whether either of two values are truthy                | ⛔      |
| = or IS      | Check whether two values are equal                           | ✅      |
| != or IS NOT | Check whether two values are not equal                       | ✅      |
| ==           | Check whether two values are exactly equal                   | ✅      |
| ?=           | Check whether any value in a set is equal to a value         | ⛔      |
| *=           | Check whether all values in a set are equal to a value       | ⛔      |
| ~            | Compare two values for equality using fuzzy matching         | ⛔      |
| !~           | Compare two values for inequality using fuzzy matching       | ⛔      |
| ?~           | Check whether any value in a set is equal to a value using fuzzy matching | ⛔      |
| *~           | Check whether all values in a set are equal to a value using fuzzy matching | ⛔      |
| <            | Check whether a value is less than another value             | ✅      |
| <=           | Check whether a value is less than or equal to another value | ✅      |
| >            | Check whether a value is greater than another value          | ✅      |
| >=           | Check whether a value is greater than or equal to another value | ✅      |
| +            | Add two values together                                      | ✅      |
| -            | Subtract a value from another value                          | ✅      |
| * or ×       | Multiply two values together                                 | ⛔      |
| / or ÷       | Divide a value by another value                              | ⛔      |
| **           | Raises a base value by another value                         | ⛔      |
| IN           | Checks whether a value is contained within another value     | ⛔      |
| NOT IN       | Checks whether a value is not contained within another value | ⛔      |
| CONTAINS     | Checks whether a value contains another value                | ✅      |
| CONTAINSNOT  | Checks whether a value does not contain another value        | ⛔      |
| CONTAINSALL  | Checks whether a value contains all other values             | ⛔      |
| CONTAINSANY  | Checks whether a value contains any other value              | ⛔      |
| CONTAINSNONE | Checks whether a value contains none of the following values | ⛔      |
| INSIDE       | Checks whether a value is contained within another value     | ⛔      |
| NOTINSIDE    | Checks whether a value is not contained within another value | ⛔      |
| ALLINSIDE    | Checks whether all values are contained within other values  | ⛔      |
| ANYINSIDE    | Checks whether any value is contained within other values    | ⛔      |
| NONEINSIDE   | Checks whether no value is contained within other values     | ⛔      |
| OUTSIDE      | Checks whether a geometry type is outside of another geometry type | ⛔      |
| INTERSECTS   | Checks whether a geometry type intersects another geometry type | ⛔      |
| @@           | Checks whether the terms are found in a full-text indexed field | ⛔      |

