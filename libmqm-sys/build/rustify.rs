use std::collections::HashMap;

use proc_macro2::Span;
use syn::{visit_mut::VisitMut, Expr, Ident, Lit, Type, TypePath};

pub struct MqLongConstWrap;
pub struct FnArgType<'a>(HashMap<(&'a str, Option<&'a str>), Type>);

impl FnArgType<'_> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl<'a> FnArgType<'a> {
    pub fn add(&mut self, arg_name: &'a str, fn_name: Option<&'a str>, ty: Type) {
        self.0.insert((arg_name, fn_name), ty);
    }
}

impl VisitMut for FnArgType<'_> {
    fn visit_foreign_item_fn_mut(&mut self, item: &mut syn::ForeignItemFn) {
        let fn_name = item.sig.ident.to_string();
        for arg in &mut item.sig.inputs {
            if let syn::FnArg::Typed(pat) = arg {
                if let syn::Pat::Ident(ident) = &*pat.pat {
                    let arg_name = ident.ident.to_string();
                    if let Some(ty) = self
                        .0
                        .get(&(&arg_name, None)) // Match on just arg name
                        .or_else(|| self.0.get(&(&arg_name, Some(&fn_name)))) // Match on arg name and fn name
                    {
                        pat.ty = Box::new(ty.clone()); // Set the new type
                    }
                }
            }
        }
    }
}

impl VisitMut for MqLongConstWrap {
    fn visit_item_const_mut(&mut self, item: &mut syn::ItemConst) {
        // Replace MQLONGs that are too large with wrapped equivalent MQLONG's.

        let mqlong = Ident::new("MQLONG", Span::call_site());
        match (&*item.ty, &*item.expr) {
            (
                // type must be MQLONG
                Type::Path(TypePath { path, .. }),
                // expression must be int literal
                Expr::Lit(syn::ExprLit {
                    lit: Lit::Int(lit_int), ..
                }),
            ) if path.is_ident(&mqlong) /* MQLONG */=> {
                // Parse as i32
                if lit_int.base10_parse::<i32>().is_err() {
                    // Parse as u32
                    if let Ok(unsigned) = lit_int.base10_parse::<u32>() {
                        // Wrap the literal integer into an i32
                        #[allow(clippy::cast_possible_wrap)]
                        let lit = syn::LitInt::new(&(unsigned as i32).to_string(), Span::call_site());
                        item.expr = Box::new(syn::parse_quote!(#lit));
                    }
                }
            }
            _ => (),
        }
    }
}
