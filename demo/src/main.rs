use pzmacro::*;
// ========================================================================

#[macro_export]  // 导出供其他包使用
macro_rules! hashmap {        // 进行宏定义，该宏的名字是hashmap
    () => { std::collections::HashMap::new() };
	//  $key: expr => $val: expr表示输入参数格式为 aa => bb,
    // $( xxxx ),+ 则表示输入参数匹配至少一次, ','是输入参数的分隔符且最后一次不能有逗号
    // $(,)? 表示匹配0个或1个逗号
    ($ ($key: expr => $val: expr),+ $(,)?  ) => {          
                                                    
        {             
            let mut map = std::collections::HashMap::new();             
	          //  $( )+ 括号中的部分将会执行，执行次数为输入参数匹配次数
            $(  map.insert($key, $val);  )+                        
             map         
        }                            
    }; 
} 

use std::collections::HashMap;
/// 测试代码
#[test]
fn test_hashmap() {
    let map1: HashMap<i32, &str> = hashmap!();
    let map2 = hashmap!(1 => "one", 2=> "two", 3=> "three" );
    let map3 = hashmap!(1 => "one", 2=> "two", 3=> "three", );
}

// ========================================================================

#[derive(Builder)]
#[derive(Debug)]
pub struct Command{
    pub executable: Option<String>,
    pub args: Vec<i32>,
    pub current_dir: String,
}


#[test]
fn test_builder() {
    let com = Command::builder()
            .executable(Some("123".to_string()))
            .args(vec![1,2,3])
            .current_dir("id".to_string())
            .build();
    println!("com {:?}", com);
}

// ========================================================================

#[log_func_info]
fn my_function() {
    println!("Hello, world!");
}

#[test]
fn test_func() {
    my_function();
}

// ========================================================================

fn main() { }
