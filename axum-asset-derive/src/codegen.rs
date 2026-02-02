use std::path::PathBuf;

use proc_macro2::TokenStream;
use quote::quote;

use crate::{ast::AssetAst, file::FileInfo};

pub fn impl_derive_asset(ast: AssetAst) -> Result<TokenStream, syn::Error> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let base_dir = PathBuf::from(&manifest_dir).join(ast.dir.value());

    let files = crate::file::collect_files(ast.dir.span(), &base_dir)?;

    let ident = ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let len = files.len();

    let get_expr = get_expr(&files);
    let iter_expr = iter_expr(&files);

    Ok(quote! {
        impl #impl_generics ::axum_asset::Asset for #ident #ty_generics #where_clause {
            fn get(path: &str) -> ::core::option::Option<::axum_asset::EmbeddedFile> {
                #get_expr
            }

            fn iter() -> impl ::core::iter::Iterator<Item = &'static str> {
                #iter_expr
            }

            fn len() -> usize {
                #len
            }
        }
    })
}

fn get_expr(files: &[FileInfo]) -> TokenStream {
    if files.is_empty() {
        return quote! {
            ::core::option::Option::None
        };
    }

    let file_exprs: Vec<_> = files.iter().map(get_file_expr).collect();

    quote! {
        match path {
            #(#file_exprs),*
            _ => ::core::option::Option::None,
        }
    }
}

fn iter_expr(files: &[FileInfo]) -> TokenStream {
    if files.is_empty() {
        return quote! {
            ::core::iter::IntoIterator::into_iter([])
        };
    }

    let relative_paths: Vec<_> = files.iter().map(|f| f.relative_path.as_str()).collect();

    quote! {
        ::core::iter::IntoIterator::into_iter([
            #(#relative_paths),*
        ])
    }
}

fn get_file_expr(file: &FileInfo) -> TokenStream {
    let content_hash = &file.content_hash;
    let last_modified = file.last_modified;
    let mime_type = &file.mime_type;

    let path = &file.relative_path;
    let route = format!("/{}", path);
    let contents = &file.contents;

    quote! {
        #path => ::core::option::Option::Some(::axum_asset::EmbeddedFile {
            route: #route,
            path: #path,
            contents: &[#(#contents),*],
            metadata: ::axum_asset::EmbeddedFileMetadata {
                content_hash: #content_hash,
                last_modified: #last_modified,
                mime_type: #mime_type,
            },
        }),
    }
}
