use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn log_func_info(_: TokenStream, input: TokenStream) -> TokenStream {
    // 解析输入的函数
    let mut func = parse_macro_input!(input as ItemFn);

    // 获取函数的名称
    let func_name = &func.sig.ident;

    // 构造函数体的代码
    let func_block = &func.block;
    let output = quote! {
        {
            println!("fun {} starts", stringify!(#func_name));
            let start_time = std::time::Instant::now();
            let __log_result = { #func_block };
            let end_time = std::time::Instant::now();
            let duration = end_time - start_time;
            println!("fun {} ends", stringify!(#func_name));
            println!("fun {} execution time: {:?}", stringify!(#func_name), duration);
            __log_result
        }
    };

    // 将函数体替换为新的代码
    func.block = syn::parse2(output).unwrap();

    // 将新的函数定义转换回 TokenStream
    quote! { #func }.into()
}
