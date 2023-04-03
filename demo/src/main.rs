use pzmacro::*;

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



fn main() {

}