use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro_derive(Entity, attributes(entity_id))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let variants = match data {
        syn::Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => {
            return quote! {
                compile_error!("Entity derive only works on structs with named fields");
            }
            .into();
        }
    };

    let ident_ids = variants.iter().filter(|field| {
        let Field { attrs, .. } = field;

        attrs.iter().any(|attr| attr.path().is_ident("entity_id"))
    });

    if ident_ids.clone().count() == 0 {
        return quote! {
            compile_error!("#[entity_id] attribute is required");
        }
        .into();
    }

    let entity_id_comps = ident_ids.map(|one_of_ids| {
        let ident = &one_of_ids.ident.clone().unwrap();
        quote! {
            self.#ident == other.#ident
        }
    });

    let expanded = quote! {
        impl #generics std::cmp::PartialEq for #ident #generics {
            fn eq(&self, other: &Self) -> bool {
                true #(&& #entity_id_comps)*
            }
        }

        impl #generics std::cmp::Eq for #ident #generics {}
    };

    TokenStream::from(expanded)
}
