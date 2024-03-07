use proc_macro::{TokenStream};
use quote::quote;

// https://github.com/dtolnay/proc-macro-workshop

// #[proc_macro_derive(Printable)]
// pub fn print_info_derive(input: TokenStream) -> TokenStream {
//     let struct_name = input.ident.to_string().as_str();
//
//     let fields = match input.data.clone() {
//         syn::Data::Struct(data) => data.fields,
//         _ => panic!("Only structs are supported"),
//     };
//     let fields_name: Vec<Ident> = fields.iter().map(|field| {
//         field.ident.as_ref().unwrap().clone()
//     }).collect();
//     let print_code = gen_print(fields_name);
//     let output_token = quote! {
//         impl Printable for #struct_name {
//             pub fn print_me(&self) {
//                 #output_token
//             }
//         }
//     };
//     output_token.into()
// }

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _ = input;
    unimplemented!()
}