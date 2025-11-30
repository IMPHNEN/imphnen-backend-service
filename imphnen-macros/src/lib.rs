use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type, PathArguments, GenericArgument};

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let builder_name = format_ident!("{}Builder", name);

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Builder derive only supports structs with named fields"),
        },
        _ => panic!("Builder derive only supports structs"),
    };

    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if is_option(ty) {
            let inner_ty = extract_option_inner(ty);
            quote! {
                #name: Option<#inner_ty>
            }
        } else {
            quote! {
                #name: Option<#ty>
            }
        }
    });

    let builder_methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if is_option(ty) {
            let inner_ty = extract_option_inner(ty);
            quote! {
                #[must_use]
                pub fn #name(mut self, #name: #inner_ty) -> Self {
                    self.#name = Some(#name);
                    self
                }
            }
        } else {
            quote! {
                #[must_use]
                pub fn #name(mut self, #name: #ty) -> Self {
                    self.#name = Some(#name);
                    self
                }
            }
        }
    });

    let expanded = quote! {
        #[derive(Default, serde::Serialize, serde::Deserialize)]
        pub struct #builder_name {
            #(#builder_fields,)*
        }

        impl #builder_name {
            #[must_use]
            pub fn new() -> Self {
                Self::default()
            }

            #(#builder_methods)*
        }
    };

    TokenStream::from(expanded)
}

fn is_option(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

fn extract_option_inner(ty: &Type) -> &Type {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                    return inner_ty;
                }
            }
        }
    }
    panic!("Expected Option<T>");
}