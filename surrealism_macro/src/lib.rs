//--------------------you need-----------------
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data};

///该宏帮助struct实现所有getter以及实现parse_sql方法
///调用者需要将他标注到需要转化到SQL语句的Struct上
///This macro helps struct implement all getters and implement parse_sql method
///The caller needs to annotate it on the Struct that needs to be converted to an SQL statement
/// 例如(example):
/// use surrealism::{ParseSQL,SQLParser};
/// #[derive(Debug, Serialize, Deserialize, ParseSQL)]
/// struct User {
///     pub userId: String,
///     pub name: String,
///     pub email: String,
/// }
///
/// let user = User {
///             userId: "123".to_string(),
///             name: "zhangsan".to_string(),
///             email: "syf20020816".to_string(),
///         }
/// user.parse_sql();
///简单来说最后会转化为：{userId:'123',name:'zhangsan','email':'syf20020816'}
#[proc_macro_derive(ParseSQL)]
pub fn parse_sql(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let fields = match &input.data {
        Data::Struct(ref fields) => &fields.fields,
        _ => panic!("Only struct with named fields are supported")
    };

    //构造属性列表
    let mut field_names = Vec::new();
    let mut getters = Vec::new();
    let mut counter = 0;
    let f_len = fields.len();
    for field in fields {
        counter += 1;
        let name = &field.ident;
        let n_str = name.clone().unwrap().to_string();
        let ty = &field.ty;
        if counter == f_len {
            field_names.push(
                quote! {
                     s.push_str(&format!("{}:'{}'", #n_str, &self.#name()));
                }
            );
        } else {
            field_names.push(
                quote! {
                        s.push_str(&format!("{}:'{}',", #n_str, &self.#name()));
                 }
            );
        }
        getters.push(quote! {
            pub fn #name(&self) -> &#ty {
                &self.#name
            }
        })
    }


    let expander = quote!(
         impl #name {
            #(#getters)*
        }

        impl SQLParser for #name {
            fn parse_sql(&self)->String{
                let mut s = String::new();
                s.push('{');
                #(#field_names)*
                s.push('}');
                s
            }
        }
    );
    TokenStream::from(expander)
}

