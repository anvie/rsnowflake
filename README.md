
Snowflake based id generator library for Rust.

[![Build Status](https://travis-ci.org/fatkhur1960/rsnowflake.svg?branch=master)](https://travis-ci.org/fatkhur1960/rsnowflake)
[![Build status](https://ci.appveyor.com/api/projects/status/9ns139hy8i6e8m2a?svg=true)](https://ci.appveyor.com/project/anvie/rsnowflake)
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
