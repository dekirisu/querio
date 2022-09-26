#![allow(dead_code)]
use querio_redisgraph::*;
use querio::*;

#[derive(QuerioRGInput, Intuple)]
struct In {
    num: u32,
    str: u32
}

#[derive(QuerioRGInput, Intuple)]
struct InR {
    num: u32,
    #[querio(ignore,cascade)] str: In,
    #[querio(ignore)] num2: u32,
}

#[derive(QuerioRGOutputJson)]
struct OutJson {
    num: u32,
    str: u32
}

#[derive(QuerioRGOutput)]
struct Out {
    num: u32,
    str: u32
}

#[derive(Strung, Intuple)]
struct HIn {
    num: u32,
    str: u32
}

#[derive(Querio)]
#[querio(I(In),O(Out),V(HIn),
    S("Test1","Test2"),"
    <Input>
    <0> <1>
    RETURN <Output>
")] struct QQ {}

#[derive(Querio)]
#[querio(I(InR),O(Out),V(HIn),S("Test1","Test2"),"
    <Input>
    <0> <1>
    RETURN <Output>
")] struct QQ2 {}

fn main(){
    println!("{}",QQ::qrio((50,12),(),(501,34)));
    println!("{}",QQ2::qrio((50,In{num: 12,str:21},12),(),(501,34)));
}