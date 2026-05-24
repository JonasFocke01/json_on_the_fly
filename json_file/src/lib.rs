use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(JsonStore)]
pub fn derive_your_trait(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let gen = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(_) => {
                quote! {
                    impl JsonStore for #name {
                        fn db_file_path() -> std::path::PathBuf {
                            let mut path = std::env::current_dir().unwrap();
                            path.push(stringify!(#name));
                            path.set_extension("json");
                            path
                        }
                    }
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                let idx: Vec<syn::Index> = (0..fields_unnamed.unnamed.len())
                    .map(syn::Index::from)
                    .collect();
                quote! {
                    impl YourTrait for #name {
                        fn describe(&self) -> String {
                            let mut parts = Vec::new();
                            #(
                                parts.push(format!("{:?}", &self.#idx));
                            )*
                            format!("{}({})", stringify!(#name), parts.join(", "))
                        }
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    impl YourTrait for #name {
                    }
                }
            }
        },
        _ => {
            return syn::Error::new_spanned(name, "YourTrait can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    gen.into()
}
