use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{DeriveInput, parse_macro_input, Data, Fields, Type};
use proc_macro2::TokenStream as TokenStream2; // 使用TokenStream的from_iter能力

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput); // 解析input为 DeriveInput类型
    let input_ident = input.ident;  // 获取原始类名
    let ident_builder = format_ident!("{}Builder", input_ident.to_string()); // 拼接builder类名

    if let Data::Struct(r) = input.data {   // 处理
        let fields = r.fields;

        // 结构体属性声明
        let builder_fields = map_fields(&fields, &mut |(ident, ty)| {
            quote!(
                #ident: Option<#ty>,
            ) 
        });

        // 为builder增加set函数
        let builder_set_fields = map_fields(&fields, &mut |(ident, ty)| {
           quote!(
                pub fn #ident(mut self, value: #ty) -> Self {
                    self.#ident = Some(value);
                    self
                }
           ) 
        });

        // 获取builder的属性值
        let builder_lets = map_fields(&fields, &mut |(ident, _)| {
            quote!(
                let #ident = self.#ident.ok_or(format!(
                    "field {:?}  not set yet", stringify!(#ident),
                ))?;
            )
        });

        // 初始化时的默认值
        let builder_fields_values = map_fields(&fields, &mut |(ident, _)| {
            quote!(
                #ident,
            )
        });

        quote!(
            impl #input_ident {
                pub fn builder() -> #ident_builder {
                    #ident_builder::default()
                }
            }

            #[derive(Default)]
            pub struct #ident_builder {
                #builder_fields
            }

            impl #ident_builder {
                #builder_set_fields
                pub fn build(self) -> Result<#input_ident, String> {
                    #builder_lets
                    Ok(#input_ident{ #builder_fields_values })
                }
            }
        ).into()

    } else {
        // 不支持非struct类型
        quote!().into()
    }
}

fn map_fields<F>(fields: &Fields, mapper:&mut F) -> TokenStream2
where
    F: FnMut((&Option<proc_macro2::Ident> ,  &Type)) -> TokenStream2,
{
    let fs = fields.iter().map(|field| mapper((&field.ident ,&field.ty)) );
    let stream2 = TokenStream2::from_iter(fs);
    stream2
}