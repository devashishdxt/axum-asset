use syn::{
    Attribute, DeriveInput, Expr, Generics, Ident, Lit, LitStr, Meta, MetaNameValue,
    spanned::Spanned,
};

#[derive(Debug)]
pub struct AssetAst {
    pub dir: LitStr,
    pub ident: Ident,
    pub generics: Generics,
}

impl TryFrom<DeriveInput> for AssetAst {
    type Error = syn::Error;

    fn try_from(value: DeriveInput) -> Result<Self, Self::Error> {
        let asset_attributes = collect_asset_attributes(&value);

        if asset_attributes.is_empty() {
            return Err(syn::Error::new(
                value.ident.span(),
                "Expected `#[asset(dir = \"path/to/dir\")]` attribute",
            ));
        }

        if asset_attributes.len() > 1 {
            return Err(syn::Error::new(
                value.ident.span(),
                "Expected only one `#[asset]` attribute",
            ));
        }

        let asset_attribute = asset_attributes.into_iter().next().unwrap();

        let dir = match asset_attribute.meta {
            Meta::List(meta_list) => {
                let meta_name_value: MetaNameValue = syn::parse2(meta_list.tokens)?;

                if !meta_name_value.path.is_ident("dir") {
                    return Err(syn::Error::new(
                        value.ident.span(),
                        "Expected only one `#[asset]` attribute",
                    ));
                }

                meta_name_value.value
            }
            Meta::NameValue(_) => {
                return Err(syn::Error::new(
                    value.ident.span(),
                    "Expected only one `#[asset]` attribute",
                ));
            }
            Meta::Path(_) => {
                return Err(syn::Error::new(
                    value.ident.span(),
                    "Expected only one `#[asset]` attribute",
                ));
            }
        };

        Ok(Self {
            dir: get_dir_from_expr(dir)?,
            ident: value.ident,
            generics: value.generics,
        })
    }
}

fn collect_asset_attributes(input: &DeriveInput) -> Vec<Attribute> {
    let mut attrs = Vec::new();

    for attr in input.attrs.iter() {
        if attr.path().is_ident("asset") {
            attrs.push(attr.clone());
        }
    }

    attrs
}

fn get_dir_from_expr(expr: Expr) -> Result<LitStr, syn::Error> {
    match expr {
        Expr::Lit(lit) => match lit.lit {
            Lit::Str(lit_str) => Ok(lit_str),
            _ => Err(syn::Error::new(
                lit.span(),
                "Expected a literal string for the `dir` attribute",
            )),
        },
        _ => Err(syn::Error::new(
            expr.span(),
            "Expected a literal string for the `dir` attribute",
        )),
    }
}
