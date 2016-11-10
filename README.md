
Snowflake based id generator library for Rust.

See: https://github.com/twitter/snowflake

Install
==========

```
rsnowflake = {git = "https://github.com/anvie/rsnowflake.git"}
```



Usage
======

```rust
let mut idgen = SnowflakeIdGenerator::new(1);
let uniqueId = idgen.generate();
```
 
 
 
[] Robin
