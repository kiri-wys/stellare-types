use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::{
    Data, DeriveInput, Expr, Field, Fields, Index, Lit, TypeParamBound, WherePredicate, parse_quote,
};

#[proc_macro_derive(BcBitops, attributes(op_override))]
pub fn bc_bitops(input: TokenStream) -> TokenStream {
    bc_ops(input, &["BitAnd", "BitOr", "BitXor"])
}
#[proc_macro_derive(BcArithmetic, attributes(op_override))]
pub fn bc_arithmetic(input: TokenStream) -> TokenStream {
    bc_ops(input, &["Add", "Sub", "Mul", "Div"])
}

#[proc_macro_derive(CwBitops, attributes(op_override))]
pub fn cw_bitops(input: TokenStream) -> TokenStream {
    cw_ops(input, &["BitAnd", "BitOr", "BitXor"])
}
#[proc_macro_derive(CwArithmetic, attributes(op_override))]
pub fn cw_arithmetic(input: TokenStream) -> TokenStream {
    cw_ops(input, &["Add", "Sub", "Mul", "Div"])
}

fn bc_ops(input: TokenStream, ops: &[&str]) -> TokenStream {
    let ast = syn::parse::<syn::DeriveInput>(input).unwrap();
    let name = &ast.ident;
    let impl_generics = generics(&ast);

    let mut impls = vec![];
    for op in ops {
        let trait_ident = format_ident!("{}", op);
        let func_ident = format_ident!("{}", op.to_lowercase());
        let mut op = vec![];
        let mut where_clause = ast.generics.clone().where_clause;
        let mut scalar_type = None;
        for w in &mut where_clause.as_mut().unwrap().predicates {
            if let WherePredicate::Type(ty) = w {
                let has_scalar = ty.bounds.iter().find_map(|b| {
                    if let TypeParamBound::Trait(t) = b {
                        t.path.segments.last().map(|e| e.ident == "Scalar")
                    } else {
                        None
                    }
                });
                if has_scalar.is_some_and(|e| e) {
                    let path = ty.bounded_ty.clone();
                    ty.bounds
                        .push(parse_quote!(std::ops::#trait_ident<Output = #path>));
                    scalar_type = Some(path);
                }
            }
        }

        let ass_trait_ident = format_ident!("{}Assign", trait_ident);
        let ass_func_ident = format_ident!("{}_assign", func_ident);
        let mut ass_op = vec![];
        let mut ass_where_clause = ast.generics.clone().where_clause;
        for w in &mut ass_where_clause.as_mut().unwrap().predicates {
            if let WherePredicate::Type(ty) = w {
                let has_scalar = ty.bounds.iter().find_map(|b| {
                    if let TypeParamBound::Trait(t) = b {
                        t.path.segments.last().map(|e| e.ident == "Scalar")
                    } else {
                        None
                    }
                });
                if has_scalar.is_some_and(|e| e) {
                    ty.bounds.push(parse_quote!(std::ops::#ass_trait_ident));
                }
            }
        }

        for component in create_components_iter(&ast) {
            match component {
                ComponentCase::Operate { ident } => {
                    op.push(
                        quote! { #ident: std::ops::#trait_ident::#func_ident(self.#ident, other) },
                    );
                    ass_op.push(quote! { std::ops::#ass_trait_ident::#ass_func_ident(&mut self.#ident, other) });
                }
                ComponentCase::Override { ident, replacement } => {
                    op.push(quote! { #ident: #replacement });
                }
            }
        }
        impls.push(quote! {
            impl #impl_generics std::ops::#trait_ident<#scalar_type> for #name #impl_generics #where_clause {
                type Output = Self;
                fn #func_ident(self, other: #scalar_type)->Self {
                    Self {
                        #(#op,)*
                    }
                }
            }
        });
        impls.push(quote! {
            impl #impl_generics std::ops::#ass_trait_ident<#scalar_type> for #name #impl_generics #ass_where_clause {
                fn #ass_func_ident(&mut self, other: #scalar_type){
                        #(#ass_op;)*
                }

            }
        });
    }
    quote! {#(#impls)*}.into()
}

fn cw_ops(input: TokenStream, ops: &[&str]) -> TokenStream {
    let ast = syn::parse::<syn::DeriveInput>(input).unwrap();
    let name = &ast.ident;
    let impl_generics = generics(&ast);

    let mut impls = vec![];
    for op in ops {
        let trait_ident = format_ident!("{}", op);
        let func_ident = format_ident!("{}", op.to_lowercase());
        let mut op = vec![];
        let mut where_clause = ast.generics.clone().where_clause;
        for w in &mut where_clause.as_mut().unwrap().predicates {
            if let WherePredicate::Type(ty) = w {
                let has_scalar = ty.bounds.iter().find_map(|b| {
                    if let TypeParamBound::Trait(t) = b {
                        t.path.segments.last().map(|e| e.ident == "Scalar")
                    } else {
                        None
                    }
                });
                if has_scalar.is_some_and(|e| e) {
                    let path = ty.bounded_ty.clone();
                    ty.bounds
                        .push(parse_quote!(std::ops::#trait_ident<Output = #path>));
                }
            }
        }

        let ass_trait_ident = format_ident!("{}Assign", trait_ident);
        let ass_func_ident = format_ident!("{}_assign", func_ident);
        let mut ass_op = vec![];
        let mut ass_where_clause = ast.generics.clone().where_clause;
        for w in &mut ass_where_clause.as_mut().unwrap().predicates {
            if let WherePredicate::Type(ty) = w {
                let has_scalar = ty.bounds.iter().find_map(|b| {
                    if let TypeParamBound::Trait(t) = b {
                        t.path.segments.last().map(|e| e.ident == "Scalar")
                    } else {
                        None
                    }
                });
                if has_scalar.is_some_and(|e| e) {
                    ty.bounds.push(parse_quote!(std::ops::#ass_trait_ident));
                }
            }
        }

        for component in create_components_iter(&ast) {
            match component {
                ComponentCase::Operate { ident } => {
                    op.push(quote! { #ident: std::ops::#trait_ident::#func_ident(self.#ident, other.#ident) });
                    ass_op.push(quote! { std::ops::#ass_trait_ident::#ass_func_ident(&mut self.#ident, other.#ident) });
                }
                ComponentCase::Override { ident, replacement } => {
                    op.push(quote! { #ident: #replacement });
                }
            }
        }
        impls.push(quote! {
            impl #impl_generics std::ops::#trait_ident for #name #impl_generics #where_clause {
                type Output = Self;
                fn #func_ident(self, other: Self)->Self {
                    Self {
                        #(#op,)*
                    }
                }
            }
        });
        impls.push(quote! {
            impl #impl_generics std::ops::#ass_trait_ident for #name #impl_generics #ass_where_clause {
                fn #ass_func_ident(&mut self, other: Self){
                        #(#ass_op;)*
                }

            }
        });
    }
    quote! {#(#impls)*}.into()
}

fn generics(ast: &DeriveInput) -> TokenStream2 {
    let params_no_defaults = ast.generics.params.iter().map(|param| match param {
        syn::GenericParam::Type(ty) => {
            let ident = &ty.ident;
            let bounds = &ty.bounds;
            quote! { #ident #bounds }
        }
        syn::GenericParam::Lifetime(lt) => quote! { #lt },
        syn::GenericParam::Const(c) => quote! { #c },
    });
    quote! { <#(#params_no_defaults),*> }
}
fn create_components_iter(
    ast: &DeriveInput,
) -> ComponentsIter<'_, syn::punctuated::Iter<'_, syn::Field>> {
    if let Data::Struct(data) = &ast.data {
        let e = &data.fields;
        let a = match e {
            Fields::Named(fields_named) => fields_named.named.iter(),
            Fields::Unnamed(fields_unnamed) => fields_unnamed.unnamed.iter(),
            Fields::Unit => todo!(),
        };
        ComponentsIter::new(a)
    } else {
        panic!("Can only be derived for structs");
    }
}

struct ComponentsIter<'a, I>
where
    I: Iterator<Item = &'a Field>,
{
    fields: I,
    field_idx: usize,
}

impl<'a, I> ComponentsIter<'a, I>
where
    I: Iterator<Item = &'a Field>,
{
    fn new(fields: I) -> Self {
        Self {
            fields,
            field_idx: 0,
        }
    }
}

impl<'a, I> Iterator for ComponentsIter<'a, I>
where
    I: Iterator<Item = &'a Field>,
{
    type Item = ComponentCase;

    fn next(&mut self) -> Option<Self::Item> {
        let field = self.fields.next()?;
        let ident = match &field.ident {
            Some(i) => i.into_token_stream(),
            None => {
                let ident = Index::from(self.field_idx);
                self.field_idx += 1;
                ident.into_token_stream()
            }
        };
        let replacement = field
            .attrs
            .iter()
            .find(|a| a.path().is_ident("op_override"))
            .map(|a| match a.parse_args::<Expr>() {
                Ok(expr) => match expr {
                    Expr::Lit(lit) => match lit.lit {
                        Lit::Str(litstr) => litstr.value().parse().unwrap_or_else(|e| {
                            syn::Error::new_spanned(litstr, e).into_compile_error()
                        }),
                        _ => syn::Error::new_spanned(lit, "Expected string literal")
                            .into_compile_error(),
                    },
                    _ => syn::Error::new_spanned(expr, "Expected string literal")
                        .into_compile_error(),
                },
                Err(e) => e.into_compile_error(),
            });
        match replacement {
            Some(replacement) => Some(ComponentCase::Override { ident, replacement }),
            None => Some(ComponentCase::Operate { ident }),
        }
    }
}

#[derive(Debug)]
enum ComponentCase {
    Operate {
        ident: TokenStream2,
    },
    Override {
        ident: TokenStream2,
        replacement: TokenStream2,
    },
}
