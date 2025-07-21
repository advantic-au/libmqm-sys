use syn::{
    parse_quote,
    token::Colon,
    visit::Visit,
    visit_mut::{visit_pat_type_mut, visit_type_mut, VisitMut},
    BareFnArg, FnArg, ForeignItemFn, Ident, ImplItem, ImplItemFn, Lifetime, PatIdent, PatType, Path, TraitItem, TraitItemFn,
    TypeReference,
};

#[derive(Debug, Default)]
pub struct TraitGenerator {
    foreign_fn: Vec<TraitItem>,
}

#[derive(Debug)]
pub struct ImplFnGenerator<F> {
    foreign_fn: Vec<ImplItem>,
    block_fn: F,
}

pub struct WrapperGenerator<F>(pub F);

impl TraitGenerator {
    #[must_use]
    pub fn generate(&self, ident: &Ident) -> syn::ItemTrait {
        let trait_fns = &self.foreign_fn;
        parse_quote!(pub trait #ident {
            #(#trait_fns)*
        })
    }
}

pub struct DesugarForMockall;

struct DesugarFn<T>(T);
impl<T: FnMut(&mut syn::Type)> VisitMut for DesugarFn<T> {
    fn visit_type_mut(&mut self, item: &mut syn::Type) {
        self.0(item);
        visit_type_mut(self, item);
    }
}
impl VisitMut for DesugarForMockall {
    fn visit_impl_item_fn_mut(&mut self, item: &mut syn::ImplItemFn) {
        let mut generics = item.sig.generics.clone();
        let mut add_lifetime = DesugarFn(|item: &mut syn::Type| {
            if let syn::Type::Reference(tr @ TypeReference { lifetime: None, .. }) = item {
                if let Some(lt_char) = ('a'..='z').find(|c| !generics.lifetimes().any(|lt| lt.lifetime.ident == c.to_string())) {
                    let lt = Lifetime::new(&format!("'{lt_char}"), proc_macro2::Span::call_site());
                    tr.lifetime = Some(lt.clone());
                    generics.params.insert(0, parse_quote!(#lt));
                }
            }
        });
        for mut input in item.sig.inputs.pairs_mut() {
            match input.value_mut() {
                syn::FnArg::Receiver(_) => (),
                syn::FnArg::Typed(pat_type) => visit_pat_type_mut(&mut add_lifetime, pat_type),
            }
        }
        item.sig.generics = generics;
    }
}

impl Visit<'_> for TraitGenerator {
    fn visit_foreign_item_fn(
        &mut self,
        ForeignItemFn {
            attrs, sig, semi_token, ..
        }: &syn::ForeignItemFn,
    ) {
        let mut sig = sig.clone();
        let self_arg = parse_quote!(&self);
        sig.inputs.insert(0, syn::FnArg::Receiver(self_arg));
        sig.unsafety = Some(parse_quote!(unsafe));

        let doc_link_re = regex_lite::Regex::new(r"\[`(.+?)`\]").unwrap();
        let new_attrs = attrs.iter().map(|attr| match attr.meta.require_name_value() {
            Ok(syn::MetaNameValue {
                value: syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(ls), ..
                }),
                ..
            }) if attr.meta.path().is_ident("doc") => {
                let val = ls.value();
                let doc = doc_link_re.replace_all(&val, |c: &regex_lite::Captures<'_>| {
                    std::borrow::Cow::Owned(format!("{}(Self::{})", &c[0], &c[1]))
                });
                let doc_lit = syn::LitStr::new(&doc, proc_macro2::Span::call_site());
                parse_quote!(#[doc = #doc_lit])
            }
            _ => attr.clone(),
        });
        self.foreign_fn.push(TraitItem::Fn(TraitItemFn {
            attrs: new_attrs.collect(),
            sig,
            default: None,
            semi_token: Some(*semi_token),
        }));
    }
}

impl<F: FnMut(syn::Field)> Visit<'_> for WrapperGenerator<F> {
    fn visit_foreign_item_fn(&mut self, item: &'_ syn::ForeignItemFn) {
        let field_name = &item.sig.ident;
        let fn_args = item.sig.inputs.iter().filter_map(|arg| match arg {
            syn::FnArg::Receiver(_) => None?,
            syn::FnArg::Typed(pat_type) => Some(BareFnArg {
                attrs: vec![],
                name: Some(match &*pat_type.pat {
                    syn::Pat::Ident(PatIdent { ident, .. }) => (ident.clone(), Colon(proc_macro2::Span::call_site())),
                    _ => None?,
                }),
                ty: *pat_type.ty.clone(),
            }),
        });
        let f = &mut self.0;
        f(parse_quote!(
            #field_name: unsafe extern "C" fn(#(#fn_args,)*)
        ));
    }
}

impl<F: Fn(&mut ImplItemFn, &TraitItemFn)> Visit<'_> for ImplFnGenerator<F> {
    fn visit_trait_item_fn(&mut self, item: &'_ syn::TraitItemFn) {
        let bf = &self.block_fn;
        let mut impl_item_fn = ImplItemFn {
            attrs: vec![],
            vis: syn::Visibility::Inherited,
            defaultness: None,
            sig: item.sig.clone(),
            block: parse_quote!({}),
        };
        bf(&mut impl_item_fn, item);
        self.foreign_fn.push(ImplItem::Fn(impl_item_fn));
    }
}

impl<F> ImplFnGenerator<F> {
    pub const fn new(block_fn: F) -> Self {
        Self {
            foreign_fn: vec![],
            block_fn,
        }
    }

    pub fn generate(self, tr: &Path, st: &Path) -> syn::ItemImpl {
        let fn_sig = self.foreign_fn;
        parse_quote!(
            impl #tr for #st {
                #(#fn_sig)*
            }
        )
    }
}

pub fn impl_dlopen2_fn(wrapper_name: &Path) -> impl Fn(&mut ImplItemFn, &TraitItemFn) + use<'_> {
    move |impl_item_fn, trait_fn| {
        let fn_name = &trait_fn.sig.ident;
        let inputs = trait_fn.sig.inputs.iter().map(|f: &FnArg| -> syn::Expr {
            match f {
                syn::FnArg::Receiver(_) => parse_quote!(self),
                syn::FnArg::Typed(PatType { pat, .. }) => parse_quote!(#pat),
            }
        });
        impl_item_fn.block = parse_quote!({
            unsafe {
                #wrapper_name::#fn_name(#(#inputs), *);
            }
        });
    }
}

pub fn impl_iep_fn(impl_item_fn: &mut ImplItemFn, trait_fn: &TraitItemFn) {
    let fn_name = &trait_fn.sig.ident;
    let fn_name_call = Ident::new(&format!("{fn_name}_Call"), fn_name.span());
    let inputs = trait_fn.sig.inputs.iter().filter_map(|arg: &FnArg| -> Option<syn::Expr> {
        if let syn::FnArg::Typed(PatType { pat, .. }) = arg {
            Some(parse_quote!(#pat))
        } else {
            None
        }
    });
    impl_item_fn.block = parse_quote!({
        unsafe {
            self.#fn_name_call.unwrap()(#(#inputs), *);
        }
    });
}

pub fn impl_link_fn(mod_path: &Path) -> impl Fn(&mut ImplItemFn, &TraitItemFn) + use<'_> {
    move |impl_item_fn, trait_fn| {
        let fn_name = &trait_fn.sig.ident;
        let inputs = trait_fn.sig.inputs.iter().filter_map(|arg: &FnArg| -> Option<syn::Expr> {
            if let syn::FnArg::Typed(PatType { pat, .. }) = arg {
                Some(parse_quote!(#pat))
            } else {
                None
            }
        });
        impl_item_fn.block = parse_quote!({
            unsafe {
               #mod_path::#fn_name(#(#inputs), *);
            }
        });
    }
}
