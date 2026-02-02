mod ast;
mod codegen;
mod file;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Asset, attributes(asset))]
pub fn derive_asset(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let asset_ast = crate::ast::AssetAst::try_from(input);

    match asset_ast {
        Ok(asset_ast) => match codegen::impl_derive_asset(asset_ast) {
            Ok(stream) => stream.into(),
            Err(err) => err.to_compile_error().into(),
        },
        Err(err) => err.to_compile_error().into(),
    }
}
