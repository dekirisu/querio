use querio::*;

#[derive(Strung,Intuple)]
struct HIn {
    num: u32,
    str: u32
}

#[derive(Querio)]
#[querio(
    V(HIn), S("Test1","Test2"), "
    #num #str #not_a_field
    <0> <1>
")] struct QQ {}

fn main(){
    println!("{}",QQ::qrio((501,34)));
}