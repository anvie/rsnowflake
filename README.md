
Snowflake based id generator library for Rust.

[![Latest Version](https://img.shields.io/crates/v/rsnowflake.svg)](https://crates.io/crates/rsnowflake)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/rsnowflake)

Install
==========

```
rsnowflake = "0.1"
```



Usage
======

```rust
let mut idgen = SnowflakeIdGenerator::new(1);
let uniqueId = idgen.generate();
```
 
 
 
[] Robin
