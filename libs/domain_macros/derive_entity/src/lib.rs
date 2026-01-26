use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    DataStruct, DeriveInput, Field, Fields, FieldsNamed, GenericParam, Generics, Lifetime,
    parse_macro_input,
};

/// 既存のジェネリクス定義と衝突しない一意なライフタイムを生成する
fn generate_unique_lifetime(generics: &Generics, base_name: &str) -> Lifetime {
    // 1. 既存のライフタイム名をすべて取得してSetにする
    let existing_lifetimes: std::collections::HashSet<String> = generics
        .params
        .iter()
        .filter_map(|param| {
            if let GenericParam::Lifetime(l) = param {
                Some(l.lifetime.to_string())
            } else {
                None
            }
        })
        .collect();

    // 2. 衝突しなくなるまで _ を追加する
    let mut candidate_name = format!("'{}", base_name);
    while existing_lifetimes.contains(&candidate_name) {
        candidate_name.push('_');
    }

    // 3. syn::Lifetime を生成して返す
    Lifetime::new(&candidate_name, Span::call_site())
}

/// `#[derive(Entity)]` 用の派生マクロです。
///
/// このマクロは「エンティティ」を表す構造体に対して、ドメイン層で利用するための
/// 各種実装を自動生成します。エンティティは 1 つ以上の「識別子フィールド」を持つ
/// 前提で設計されており、そのフィールドに `#[entity_id]` 属性を付与します。
///
/// # 使い方
///
/// 単一の識別子を持つエンティティ:
///
/// ```rust
/// use derive_entity::Entity;
///
/// #[derive(PartialEq, Eq)]
/// struct UserId(u64);
///
/// #[derive(Entity)]
/// struct User {
///     /// エンティティを一意に識別する ID
///     #[entity_id]
///     id: UserId,
///     name: String,
/// }
/// ```
///
/// 複合主キー（複数フィールドによる識別子）を持つエンティティ:
///
/// ```rust
/// use derive_entity::Entity;
///
/// #[derive(PartialEq, Eq)]
/// struct StudentId(u64);
/// #[derive(PartialEq, Eq)]
/// struct CourseId(u64);
///
/// #[derive(Entity)]
/// struct Enrollment {
///     /// 複合 ID の一部として扱われるフィールド
///     #[entity_id]
///     student_id: StudentId,
///
///     /// 複合 ID の一部として扱われるフィールド
///     #[entity_id]
///     course_id: CourseId,
///
///     enrolled_at: std::time::SystemTime,
/// }
/// ```
///
/// 上記のように、`#[entity_id]` が付与された 1 つ以上のフィールドが「エンティティ ID」
/// として扱われます。1 つだけ付与した場合は単一 ID、複数に付与した場合はそれらの
/// 組み合わせがエンティティの識別子（複合 ID）として解釈されます。
///
/// このマクロは **名前付きフィールドを持つ構造体** に対してのみ有効です。
/// タプル構造体や列挙型に対して使用した場合はコンパイルエラーになります。
///
/// また、`#[entity_id]` が 1 つも指定されていない場合もコンパイルエラーになります。
#[proc_macro_derive(Entity, attributes(entity_id))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let unique_lifetime = generate_unique_lifetime(&generics, "identity_scope");

    let fields = match data {
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

    let ident_ids = fields.iter().filter(|field| {
        let Field { attrs, .. } = field;

        attrs.iter().any(|attr| attr.path().is_ident("entity_id"))
    });

    if ident_ids.clone().next().is_none() {
        return quote! {
            compile_error!("#[entity_id] attribute is required");
        }
        .into();
    }

    let ident_id_names = ident_ids.clone().map(|one_of_ids| {
        let ident = one_of_ids
            .ident
            .as_ref()
            .expect("internal error in derive(Entity): expected named field to have an identifier; this is a bug in the macro, please report it");
        quote! {
            self.#ident
        }
    });

    let ident_id_types = ident_ids.clone().map(|one_of_ids| {
        let ty = &one_of_ids.ty;
        quote! {
            #ty
        }
    });

    let generics_bounds = generics.params.iter().map(|param| match param {
        syn::GenericParam::Type(t) => {
            let ident = &t.ident;
            quote! { #ident: #unique_lifetime }
        }
        syn::GenericParam::Lifetime(l) => {
            let lifetime = &l.lifetime;
            quote! { #lifetime: #unique_lifetime }
        }
        syn::GenericParam::Const(_) => {
            // 定数ジェネリックパラメータにはライフタイム境界を指定できないため、ここでは意図的に境界を生成しない
            quote! {}
        }
    });

    let expanded = quote! {
        impl #impl_generics ::domain_objects::EntityTrait for #ident #ty_generics
        #where_clause
        {
            type Identity<#unique_lifetime>
                = (#(&#unique_lifetime #ident_id_types),*)
            where
                #(#generics_bounds),*;

            fn identity(&self) -> Self::Identity<'_> {
                (#(&#ident_id_names),*)
            }
        }

        impl #impl_generics std::cmp::PartialEq for #ident #ty_generics
        #where_clause
        {
            fn eq(&self, other: &Self) -> bool {
                <#ident #ty_generics as ::domain_objects::EntityTrait>::eq(self, other)
            }
        }

        impl #impl_generics std::cmp::Eq for #ident #ty_generics
        #where_clause
        {}
    };

    TokenStream::from(expanded)
}
