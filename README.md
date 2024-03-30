<div align="center" style="background:#6d0023;padding:12px 0px;margin-bottom:16px;border-radius:24px;">
    <img src="https://github.com/dekirisu/strung/assets/78398528/59006ba5-f19c-4617-9cf3-691e841b0aba">
</div>
<p align="center">
    <a href="https://github.com/dekirisu/querio" style="position:relative">
        <img src="https://img.shields.io/badge/github-dekirisu/querio-ee6677">
    </a>
    <a href="https://crates.io/crates/querio" style="position:relative">
        <img src="https://img.shields.io/crates/v/querio">
    </a>
</p>

<h3 align="center">🚧 WIP, but wanted a basic version up 🚧</h3>

## Struct based string builder
🐠 add **querio** to the dependencies in the `Cargo.toml`:
```toml
[dependencies]
querio = "0.0.1"
```
🦀 use/import everything into rust:
```rust 
use querio::*;
```
🦊 build the string:
```rust 
// runtime variables
#[derive(Strung,Intuple)]
struct VarStruct {
    numb: u32,
    strg: &'static str
}

// create builder
#[derive(Querio)]
#[querio(
    // attach runtime variables, address with #{field_name}
    variables(VarStruct), 

    // add compile time variables, address with <index>
    sections("Test1","Test2"), 

    // the string those variables will be merged in
    "#numb #strg <0> <1> <0>"
)] struct ABuilder {}

fn main(){
    // use the builder with attached variables struct
    let text = ABuilder::querio(&VarStruct{numb:10,strg:"RT"}); 
    // => "10 RT Test1 Test2 Test1"
        
    // OR use the variable structs tuple representation and ::qrio(..)
    let text = ABuilder::qrio((10,"RT"));
    // => "10 RT Test1 Test2 Test1"
}
```

---
### License
<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>