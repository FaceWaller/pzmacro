mod macros;
use proc_macro::TokenStream;

/* 
#[derive(Builder)]
#[derive(Debug)]
pub struct Command{
    pub executable: Option<String>,
    pub args: Vec<i32>,
    pub current_dir: String,
}

#[test]
fn test_builder() {
    println!("start-------------");
    let com = Command::builder().executable(Some("123".to_string())).args(vec![1,2,3]).current_dir("id".to_string()).build();
    println!("com {:?}", com);
    println!("end  -------------");
}
*/

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    macros::builder::derive_builder(input)
}

/*
#[log_func_info]
fn my_function() {
    println!("Hello, world!");
}

#[test]
fn test_func() {
    my_function();
}
*/

#[proc_macro_attribute]
pub fn log_func_info(attr: TokenStream, input: TokenStream) -> TokenStream {
    macros::fun_log::log_func_info(attr, input)
}

