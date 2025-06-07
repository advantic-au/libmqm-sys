use std::{collections::HashMap, mem::swap};

use regex_lite::Regex;
use syn::{visit_mut::VisitMut, Attribute};

const BARE_REGEX: &str = r"(?msx)
    /\*+/
    \s+/\*\s+([A-Za-z_0-9]+)\s+(?:Structure\s+|Function\s+)?--?\s
    (.+?)\s+\*/
    \s*/\*+/
";
const SPLIT_REGEX: &str = r"(?m)\s*(\*/|/\*)\s*";

const STRUCT_REGEX: &str = r"(?msx)
    struct\s+([\w\d_]+)\s+\{
    (.+?)
    \};
";

const FIELD_REGEX: &str = r"(?msx)
    ^(?:\s+([\w\d]+)(?:\[\d+\])?)+;
    ((?:\s+/\*.+?\*/)+)
";

const FN_REGEX: &str = r"(?msx)
    (?msx)
    void\s+MQENTRY\s+([\w\d_]+)\s\(
    (.*?\);
    (?:\s+/\*.+?\*/)+)
";

const PARAM_REGEX: &str = r"(?msx)
    ^(?:\s+([\w\d]+))+(?:,|\);)
    ((?:\s+/\*.+?\*/)+)
";

#[derive(Debug, Clone)]
pub struct DocCommentType<'a>(pub &'a HashMap<String, String>);

#[derive(Debug, Clone)]
pub struct DocCommentFields<'a>(pub &'a HashMap<String, HashMap<String, String>>);

#[derive(Debug, Clone)]
pub struct DocCommentArgs<'a>(pub &'a HashMap<String, Vec<(String, String)>>);

pub struct DescriptionRegex {
    search: Regex,
    delim: Regex,
}

pub struct StructFieldExtract {
    struct_search: Regex,
    field_search: Regex,
    delim: Regex,
}

pub struct FnParamExtract {
    fn_search: Regex,
    param_search: Regex,
    delim: Regex,
}

impl Default for DescriptionRegex {
    fn default() -> Self {
        Self {
            search: Regex::new(BARE_REGEX).unwrap(),
            delim: Regex::new(SPLIT_REGEX).unwrap(),
        }
    }
}

impl Default for StructFieldExtract {
    fn default() -> Self {
        Self {
            struct_search: Regex::new(STRUCT_REGEX).unwrap(),
            field_search: Regex::new(FIELD_REGEX).unwrap(),
            delim: Regex::new(SPLIT_REGEX).unwrap(),
        }
    }
}

impl Default for FnParamExtract {
    fn default() -> Self {
        Self {
            fn_search: Regex::new(FN_REGEX).unwrap(),
            param_search: Regex::new(PARAM_REGEX).unwrap(),
            delim: Regex::new(SPLIT_REGEX).unwrap(),
        }
    }
}

/// Extracts items from source code represented as a String key and an associated data
pub trait ExtractFromC: Sized {
    type Extracted;
    fn extract(&self, file_content: &str) -> impl Iterator<Item = (String, Self::Extracted)>;
}

impl ExtractFromC for DescriptionRegex {
    type Extracted = String;

    fn extract(&self, file_content: &str) -> impl Iterator<Item = (String, Self::Extracted)> {
        self.search.captures_iter(file_content).map(|c| {
            (
                c[1].to_string(),
                self.delim
                    .split(&c[2])
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .fold(String::new(), |mut acc, s| {
                        acc += " ";
                        acc += &s.split_whitespace().collect::<Vec<_>>().join(" ");
                        acc
                    }),
            )
        })
    }
}

impl ExtractFromC for StructFieldExtract {
    type Extracted = HashMap<String, String>;

    fn extract(&self, file_content: &str) -> impl Iterator<Item = (String, Self::Extracted)> {
        self.struct_search.captures_iter(file_content).map(|c| {
            (
                c[1].to_string(), // Struct name
                self.field_search
                    .captures_iter(&c[2])
                    .map(|c| {
                        (
                            c[1].to_string(), // Field name
                            self.delim
                                .split(&c[2])
                                .filter(|s| !s.is_empty() && !s.starts_with("Ver:"))
                                .collect::<Vec<_>>()
                                .join(" "),
                        )
                    })
                    .collect(),
            )
        })
    }
}

impl ExtractFromC for FnParamExtract {
    type Extracted = Vec<(String, String)>;

    fn extract(&self, file_content: &str) -> impl Iterator<Item = (String, Self::Extracted)> {
        self.fn_search.captures_iter(file_content).map(|c| {
            (
                c[1].to_string(), // Function name
                self.param_search
                    .captures_iter(&c[2])
                    .map(|c| {
                        (
                            c[1].to_string(), // Param name
                            self.delim
                                .split(&c[2])
                                .filter(|s| !s.is_empty())
                                .take_while(|s| s.chars().any(|c| c != '*'))
                                .collect::<Vec<_>>()
                                .join(" "),
                        )
                    })
                    .collect(),
            )
        })
    }
}

impl VisitMut for DocCommentType<'_> {
    fn visit_item_type_mut(&mut self, item: &mut syn::ItemType) {
        if let Some(description) = self.0.get(&format!("{}", item.ident)) {
            let doc_lit = syn::LitStr::new(description, proc_macro2::Span::call_site());
            item.attrs.insert(0, syn::parse_quote!(#[doc = #doc_lit ]));
        }
    }

    fn visit_item_fn_mut(&mut self, item: &mut syn::ItemFn) {
        if let Some(description) = self.0.get(&format!("{}", item.sig.ident)) {
            let doc_lit = syn::LitStr::new(description, proc_macro2::Span::call_site());
            item.attrs.insert(0, syn::parse_quote!(#[doc = #doc_lit ]));
        }
    }

    fn visit_foreign_item_fn_mut(&mut self, item: &mut syn::ForeignItemFn) {
        if let Some(description) = self.0.get(&format!("{}", item.sig.ident)) {
            let doc_lit = syn::LitStr::new(description, proc_macro2::Span::call_site());
            item.attrs.insert(0, syn::parse_quote!(#[doc = #doc_lit ]));
        }
    }
}

impl VisitMut for DocCommentFields<'_> {
    fn visit_item_struct_mut(&mut self, item: &mut syn::ItemStruct) {
        if let Some(fields) = self.0.get(&format!("{}", item.ident)) {
            for field in &mut item.fields {
                if let Some(description) = fields.get(&format!("{}", field.ident.as_ref().unwrap())) {
                    let doc_lit = syn::LitStr::new(&format!(" {description}"), proc_macro2::Span::call_site());
                    field.attrs.insert(0, syn::parse_quote!(#[doc = #doc_lit ]));
                }
            }
        }
    }
}

fn doc_comment_args(args: &[(String, String)]) -> Vec<Attribute> {
    let mut arg_attrs: Vec<Attribute> = vec![syn::parse_quote!(#[doc = " # Arguments"])];
    arg_attrs.extend(args.iter().map(|(name, description)| {
        let doc_lit = syn::LitStr::new(&format!(" * `{name}`: {description}"), proc_macro2::Span::call_site());
        syn::parse_quote!(#[doc = #doc_lit])
    }));
    arg_attrs
}

impl VisitMut for DocCommentArgs<'_> {
    fn visit_item_fn_mut(&mut self, item: &mut syn::ItemFn) {
        if let Some(args) = self.0.get(&format!("{}", item.sig.ident)) {
            let mut arg_attrs = doc_comment_args(args);
            swap(&mut item.attrs, &mut arg_attrs);
            item.attrs.extend(arg_attrs);
        }
    }

    fn visit_foreign_item_fn_mut(&mut self, item: &mut syn::ForeignItemFn) {
        if let Some(args) = self.0.get(&format!("{}", item.sig.ident)) {
            let mut arg_attrs = doc_comment_args(args);
            swap(&mut item.attrs, &mut arg_attrs);
            item.attrs.extend(arg_attrs);
        }
    }
}

/// Iterates over the headers and applies and extractor on each header content
pub fn extract_from_headers<T: ExtractFromC>(
    inc_path: &std::path::Path,
    extractor: &T,
) -> std::io::Result<HashMap<String, T::Extracted>> {
    let mut result = HashMap::new();
    for item in std::fs::read_dir(inc_path)? {
        let item = item?;
        let file_path = item.path();
        if file_path.extension().is_some_and(|e| e == "h") && item.file_type()?.is_file() {
            let file_content = std::fs::read_to_string(file_path)?;
            result.extend(extractor.extract(&file_content));
        }
    }

    Ok(result)
}
