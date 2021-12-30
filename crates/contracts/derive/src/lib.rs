//! contracts derive
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::fs;
use syn::{parse_macro_input, DeriveInput, LitStr};

/// Derive contract metadata
#[proc_macro_attribute]
pub fn meta(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let contract = &ast.ident;
    let mut contract_name = contract.to_string();
    for (n, _) in contract_name
        .clone()
        .match_indices(|s: char| s.is_uppercase())
    {
        if n != 0 && n != contract_name.len() {
            contract_name.insert(n, '_');
        }
    }
    contract_name = contract_name.to_uppercase();

    // read abi
    let mut path = env!("CARGO_MANIFEST_DIR").to_string();
    path.push_str(&format!(
        "/../resources/{}.abi.json",
        contract_name.replace("_", "-").to_lowercase()
    ));
    let abi = LitStr::new(
        &fs::read_to_string(&path).expect(&format!("read {} failed", path)),
        Span::call_site(),
    );

    // build address
    let address_hex = attr.to_string();
    if address_hex.len() != 42 && address_hex.len() != 40 {
        panic!("invalid contract address: {}", contract_name.replace("_", "-").to_lowercase());
    }
    let address = LitStr::new(&address_hex, Span::call_site());

    quote! {
        #ast

        impl crate::Metadata for #contract {
            fn abi() -> Vec<u8> {
                #abi.as_bytes().to_vec()
            }

            fn address() -> [u8; 20] {
                let mut r: [u8; 20] = [0; 20];
                if let Ok(addr) = hex::decode(&#address[2..]) {
                    r.copy_from_slice(&addr);
                }
                r
            }
        }
    }
    .into()
}
