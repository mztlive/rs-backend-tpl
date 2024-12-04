use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    quote! {
        impl entity_core::HasId for #name {
            fn get_id(&self) -> &str {
                &self.base.id
            }
        }

        impl entity_core::HasVersion for #name {
            fn get_version(&self) -> u64 {
                self.base.version
            }
        }
    }
    .into()
}
