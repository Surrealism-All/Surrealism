# Surrealism V0.3+

## Surreal

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
    <input type="checkbox" checked disabled />  <strong>RELATE STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>UPDATE STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>INSERT STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>DELETE STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>INFO STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>TRANSACTION STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>DEFINE STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>REMOVE STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>SLEEP STMT</strong> <br />
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

### Delete

<form>
    <input type="checkbox" checked disabled />  <strong>DELETE WHERE STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>RETURN STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>TIMEOUT STMT</strong> <br />
    <input type="checkbox" checked disabled />  <strong>PARALLEL STMT</strong> <br />
    <input type="checkbox" disabled />  <strong>DELETE WITH RELETE STMT</strong> <br />
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

## Row

<form>
    <input type="checkbox" checked disabled />  <strong>RowSql的支持</strong> <br />
    <input type="checkbox" checked disabled/>  <strong>row_sql!宏</strong> <br />
</form>
