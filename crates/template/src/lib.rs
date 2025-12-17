use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta};

#[proc_macro_derive(Template, attributes(template))]
pub fn derive_template(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // Extract the template path from the attribute
    let template_path = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("template"))
        .and_then(|attr| {
            if let Meta::List(meta_list) = &attr.meta {
                syn::parse2::<syn::MetaNameValue>(meta_list.tokens.clone())
                    .ok()
                    .and_then(|nv| {
                        if nv.path.is_ident("path") {
                            if let syn::Expr::Lit(expr_lit) = nv.value {
                                if let syn::Lit::Str(lit_str) = expr_lit.lit {
                                    return Some(lit_str.value());
                                }
                            }
                        }
                        None
                    })
            } else {
                None
            }
        })
        .expect("template attribute must have a path parameter");

    let template_const_name = syn::Ident::new(
        &format!("{}_TEMPLATE", name.to_string().to_uppercase()),
        name.span(),
    );

    let expanded = quote! {
        const #template_const_name: &str = include_str!(#template_path);

        impl #name {
            fn render(&self) -> String {
                let mut tt = ::tinytemplate::TinyTemplate::new();
                tt.add_template("template", #template_const_name).unwrap();
                tt.render("template", &self).unwrap()
            }
        }
    };

    TokenStream::from(expanded)
}
