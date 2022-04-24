use proc_macro::TokenStream as MainTokenStream;

use proc_macro2::TokenStream;
use quote::quote;

#[proc_macro_derive(Sanitize, attributes(sanitize))]
pub fn sanitize_derive(input: MainTokenStream) -> MainTokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_sanitize(&ast).into()
}

fn impl_sanitize(ast: &syn::DeriveInput) -> TokenStream {
    // Ensure the macro is on a struct with named fields
    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            if fields.iter().any(|field| field.ident.is_none()) {
                panic!("invalid field, struct with named fields");
            }
            fields.iter().cloned().collect::<Vec<_>>()
        }
        _ => panic!("invalid field, struct with named fields"),
    };

    let mut sanitize_fields: Vec<TokenStream> = vec![];

    for field in fields {
        let field_ident = field.ident.clone().unwrap();

        let mut skip = false;

        for attr in field.attrs {
            if attr.tokens.to_string() == "(skip_sanitizing)" {
                skip = true;
                break;
            }
        }

        if skip {
            sanitize_fields.push(quote! {
                #field_ident: self.#field_ident,
            });
        } else {
            sanitize_fields.push(quote! {
                #field_ident: self.#field_ident.sanitize(),
            });
        }
    }

    let name = &ast.ident;

    let gen = quote! {
        impl Sanitize for #name {
            fn sanitize(mut self) -> Self {
               #name {
                #(#sanitize_fields)*
               }
            }
        }
    };

    gen.into()
}
