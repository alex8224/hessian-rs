extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use syn::ext::IdentExt;

#[proc_macro_derive(PrintFields)]
pub fn derive_print_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let field_names = field_names(input.data);
    let mut map = std::collections::HashMap::<&str, &str>::new();
    for field in field_names {
        let parts = field.split(" ").collect::<Vec<_>>();
        map.insert(parts[0], parts[1]);
    }

    let ident_name = format!("{}", ident);
    let output = quote! {
        impl #ident {
            // fn print_field_names(&self) {
            //     println!("====print field meta==");
            //     #(println!(#field_names);)*
            // }
            fn meta(&self) -> std::collections::HashMap<&str, &str> {
                let mut map = std::collections::HashMap::<&str, &str>::new();
                map
            }
        }
    };
    output.into()
}

fn field_names(data: Data) -> impl Iterator<Item=String> {
    match data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) =>
                fields.named.into_iter()
                    .map(|field| {
                        let mut buffstr = String::new();
                        let mut cnt = 0;
                        match field.ty {
                            syn::Type::Path(v) => {
                                let seg = v.path.segments;
                                let seg_len = &seg.len();
                                for i in seg {
                                    buffstr.push_str(i.ident.to_string().as_str());
                                    if cnt < seg_len  - 1{
                                        buffstr.push_str("::")
                                    }
                                    cnt+=1;
                                }
                            }
                            _ => {}
                        }
                        // (field.ident.unwrap().to_string(), buffstr)
                        format!("{} {}", &field.ident.unwrap().to_string(), buffstr)
                    }),
            _ => unimplemented!(),
        }
        _ => unimplemented!(),
    }
}