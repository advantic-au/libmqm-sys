use std::{borrow::Cow, collections::HashMap};

use regex_lite::{Captures, Regex};
use syn::{
    parse_quote,
    visit_mut::{visit_item_mut, VisitMut},
    Attribute,
};

// Used to match code comments above structures / functions
const BARE_REGEX: &str = r"(?msx)
    /\*+/
    \s+/\*\s+([A-Za-z_0-9]+)\s+(?:Structure\s+|Function\s+)?--?\s
    (.+?)\s+\*/
    \s*/\*+/
";
const SPLIT_REGEX: &str = r"(?m)\s*(\*/|/\*)\s*";

const MQ_ITEM_REGEX: &str = r"MQ[A-Z\d_]+";

fn doc_link(c: &Captures) -> Cow<'static, str> {
    let matched = &c[0];
    // MQPMR isn't a real struct
    Cow::Owned(if matched == "MQPMR" {
        format!("`{matched}`")
    } else {
        format!("[`{matched}`]")
    })
}

const STRUCT_REGEX: &str = r"(?msx)
    struct\s+([\w\d_]+)\s+\{
    (.+?)
    \};
";

const FIELD_REGEX: &str = r"(?msx)
    ^(?:\s+([\w\d]+)(?:\[\d+\])?)+;
    ((?:\s+/\*.+?\*/)+)
";

const VER_REGEX: &str = r"/\*\s+Ver:(\d+)\s+\*/";

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

const IBM_REFERENCE_CSV: &str = include_str!("ibm_references.csv");
static IBM_REFERENCES: std::sync::OnceLock<HashMap<String, String>> = std::sync::OnceLock::new();

const EQUIV: &[(&str, &str)] = &[
    ("MQBACK", "MQ_BACK_CALL"),
    ("MQBEGIN", "MQ_BEGIN_CALL"),
    ("MQBUFMH", "MQ_BUFMH_CALL"),
    ("MQCB", "MQ_CB_CALL"),
    ("MQCLOSE", "MQ_CLOSE_CALL"),
    ("MQCMIT", "MQ_CMIT_CALL"),
    ("MQCONN", "MQ_CONN_CALL"),
    ("MQCONNX", "MQ_CONNX_CALL"),
    ("MQCRTMH", "MQ_CRTMH_CALL"),
    ("MQCTL", "MQ_CTL_CALL"),
    ("MQDISC", "MQ_DISC_CALL"),
    ("MQDLTMH", "MQ_DLTMH_CALL"),
    ("MQDLTMP", "MQ_DLTMP_CALL"),
    ("MQGET", "MQ_GET_CALL"),
    ("MQINQ", "MQ_INQ_CALL"),
    ("MQINQMP", "MQ_INQMP_CALL"),
    ("MQMHBUF", "MQ_MHBUF_CALL"),
    ("MQOPEN", "MQ_OPEN_CALL"),
    ("MQPUT", "MQ_PUT_CALL"),
    ("MQPUT1", "MQ_PUT1_CALL"),
    ("MQSET", "MQ_SET_CALL"),
    ("MQSETMP", "MQ_SETMP_CALL"),
    ("MQSTAT", "MQ_STAT_CALL"),
    ("MQSUB", "MQ_SUB_CALL"),
    ("MQSUBRQ", "MQ_SUBRQ_CALL"),
    ("MQXCLWLN", "MQ_XCLWLN_CALL"),
    ("MQXCNVC", "MQ_XCNVC_CALL"),
    ("MQXDX", "MQ_XDX_CALL"),
    ("MQXEP", "MQ_XEP_CALL"),
    ("MQZEP", "MQ_ZEP_CALL"),
];

const REPLACE: &[(&str, &str)] = &[
    ("CompCode", "`CompCode`"),
    ("InBuffer", "`InBuffer`"),
    ("OutBuffer", "`OutBuffer`"),
];

fn replace_keywords(comment: &str) -> String {
    REPLACE
        .iter()
        .fold(comment.to_string(), |comment, (from, to)| comment.replace(from, to))
}

#[derive(Debug, Clone)]
pub struct DocCommentType<'a>(pub &'a HashMap<String, String>);

#[derive(Debug, Clone)]
pub struct DocCommentFields<'a>(pub &'a HashMap<String, HashMap<String, (String, Option<usize>)>>);

#[derive(Debug, Clone)]
pub struct DocCommentArgs<'a>(pub &'a HashMap<String, Vec<(String, String)>>);

#[derive(Debug, Clone)]
pub struct DocCommentReference;

pub struct DescriptionRegex {
    search: Regex,
    delim: Regex,
    mq_item: Regex,
}

pub struct StructFieldExtract {
    struct_search: Regex,
    field_search: Regex,
    ver_search: Regex,
    delim: Regex,
    mq_item: Regex,
}

pub struct FnParamExtract {
    fn_search: Regex,
    param_search: Regex,
    delim: Regex,
    mq_item: Regex,
}

impl Default for DescriptionRegex {
    fn default() -> Self {
        Self {
            search: Regex::new(BARE_REGEX).unwrap(),
            delim: Regex::new(SPLIT_REGEX).unwrap(),
            mq_item: Regex::new(MQ_ITEM_REGEX).unwrap(),
        }
    }
}

impl Default for StructFieldExtract {
    fn default() -> Self {
        Self {
            struct_search: Regex::new(STRUCT_REGEX).unwrap(),
            field_search: Regex::new(FIELD_REGEX).unwrap(),
            ver_search: Regex::new(VER_REGEX).unwrap(),
            delim: Regex::new(SPLIT_REGEX).unwrap(),
            mq_item: Regex::new(MQ_ITEM_REGEX).unwrap(),
        }
    }
}

impl Default for FnParamExtract {
    fn default() -> Self {
        Self {
            fn_search: Regex::new(FN_REGEX).unwrap(),
            param_search: Regex::new(PARAM_REGEX).unwrap(),
            delim: Regex::new(SPLIT_REGEX).unwrap(),
            mq_item: Regex::new(MQ_ITEM_REGEX).unwrap(),
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
        let mut extracted = self
            .search
            .captures_iter(file_content)
            .map(|c| {
                let description =
                    self.delim
                        .split(&c[2])
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .fold(String::new(), |mut acc, s| {
                            acc += " ";
                            acc += &s.split_whitespace().collect::<Vec<_>>().join(" ");
                            acc
                        });
                let description = self.mq_item.replace_all(&description, doc_link);

                (c[1].to_string(), replace_keywords(&description))
            })
            .collect::<HashMap<_, _>>();

        for (name, other) in EQUIV {
            if let Some(description) = extracted.get(*name) {
                let description = description.clone();
                extracted.entry((*other).to_string()).or_insert(description);
            }
        }

        extracted.into_iter()
    }
}

impl ExtractFromC for StructFieldExtract {
    type Extracted = HashMap<String, (String, Option<usize>)>; // field: (description, min_version)

    fn extract(&self, file_content: &str) -> impl Iterator<Item = (String, Self::Extracted)> {
        self.struct_search.captures_iter(file_content).map(move |c| {
            // tag convention is an unneeded c relic which bindgen is configured to remove
            let struct_name = c[1].trim_start_matches("tag").to_string();

            // Split the fields into version segments
            let struct_content = &c[2];
            let mut match_pos = 0;
            let mut by_version = vec![];
            for m in self.ver_search.find_iter(struct_content) {
                by_version.push(&struct_content[match_pos..m.end()]);
                match_pos = m.end();
            }
            by_version.push(&struct_content[match_pos..]);

            let extracted = by_version.iter().flat_map(|&section| {
                let version: Option<usize> = self.ver_search.captures(section).and_then(|c| c[1].parse().ok());
                self.field_search.captures_iter(section).map(move |c| {
                    let description = self
                        .delim
                        .split(&c[2])
                        .filter(|s| !s.is_empty() && !s.starts_with("Ver:"))
                        .collect::<Vec<_>>()
                        .join(" ");
                    let description = self.mq_item.replace_all(&description, doc_link);

                    (
                        c[1].to_string(), // Field name
                        (replace_keywords(&description), version),
                    )
                })
            });

            (struct_name, extracted.collect())
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
                        let description = self
                            .delim
                            .split(&c[2])
                            .filter(|s| !s.is_empty())
                            .take_while(|s| s.chars().any(|c| c != '*'))
                            .collect::<Vec<_>>()
                            .join(" ");
                        let description = self.mq_item.replace_all(&description, doc_link);
                        (
                            c[1].to_string(), // Param name
                            replace_keywords(&description),
                        )
                    })
                    .collect(),
            )
        })
    }
}

impl VisitMut for DocCommentType<'_> {
    fn visit_item_type_mut(&mut self, item: &mut syn::ItemType) {
        if let Some(description) = self.0.get(&item.ident.to_string()) {
            let doc_lit = syn::LitStr::new(description, proc_macro2::Span::call_site());
            item.attrs.insert(0, syn::parse_quote!(#[doc = #doc_lit ]));
        }
    }

    fn visit_item_struct_mut(&mut self, item: &mut syn::ItemStruct) {
        let name = item.ident.to_string();
        let mut names = vec![&*name, name.trim_start_matches("tag")];
        names.dedup();

        if let Some(description) = names.iter().find_map(|name| self.0.get(*name)) {
            let doc_lit = syn::LitStr::new(description, proc_macro2::Span::call_site());
            item.attrs.insert(0, syn::parse_quote!(#[doc = #doc_lit ]));
        }
    }

    fn visit_item_fn_mut(&mut self, item: &mut syn::ItemFn) {
        if let Some(description) = self.0.get(&item.sig.ident.to_string()) {
            let doc_lit = syn::LitStr::new(description, proc_macro2::Span::call_site());
            item.attrs.insert(0, syn::parse_quote!(#[doc = #doc_lit ]));
        }
    }

    fn visit_foreign_item_fn_mut(&mut self, item: &mut syn::ForeignItemFn) {
        if let Some(description) = self.0.get(&item.sig.ident.to_string()) {
            let doc_lit = syn::LitStr::new(description, proc_macro2::Span::call_site());
            item.attrs.insert(0, syn::parse_quote!(#[doc = #doc_lit ]));
        }
    }
}

impl VisitMut for DocCommentFields<'_> {
    fn visit_item_struct_mut(&mut self, item: &mut syn::ItemStruct) {
        let struct_name = item.ident.to_string();
        if let Some(fields) = self.0.get(&struct_name) {
            for field in &mut item.fields {
                if let Some((description, version)) = fields.get(&field.ident.as_ref().unwrap().to_string()) {
                    let version = version.iter().filter(|ver| **ver > 1).flat_map(|ver| {
                        [
                            syn::LitStr::new("", proc_macro2::Span::call_site()),
                            syn::LitStr::new(
                                &format!(" [`{struct_name}::Version`] >= {ver}"),
                                proc_macro2::Span::call_site(),
                            ),
                        ]
                    });

                    let desc_lit = std::iter::once(syn::LitStr::new(&format!(" {description}"), proc_macro2::Span::call_site()));
                    let doc_lit = desc_lit
                        .chain(version)
                        .map(|lit| syn::parse_quote!(#[doc = #lit ]))
                        .chain(field.attrs.iter().cloned());

                    field.attrs = doc_lit.collect();
                }
            }
        }
    }
}

fn doc_comment_args(args: &[(String, String)]) -> Vec<Attribute> {
    let mut arg_attrs: Vec<Attribute> = vec![syn::parse_quote!(#[doc = ""]), syn::parse_quote!(#[doc = " # Arguments"])];
    arg_attrs.extend(args.iter().map(|(name, description)| {
        let dir_desc = match description.split_once(':') {
            Some(("O" | "OC" | "OR" | "OB" | "OL", desc)) => Cow::Owned(format!(" (Output):{desc}")),
            Some(("IO" | "IOB" | "IOL", desc)) => Cow::Owned(format!(" (Input/Output):{desc}")),
            Some((_, desc)) => Cow::Owned(format!(":{desc}")),
            _ => Cow::Borrowed(&**description),
        };
        let doc_lit = syn::LitStr::new(
            &format!(" * `{}`{dir_desc}", name.trim_start_matches('p')),
            proc_macro2::Span::call_site(),
        );
        syn::parse_quote!(#[doc = #doc_lit])
    }));
    arg_attrs
}

impl VisitMut for DocCommentArgs<'_> {
    fn visit_foreign_item_fn_mut(&mut self, item: &mut syn::ForeignItemFn) {
        if let Some(args) = self.0.get(&item.sig.ident.to_string()) {
            let mut arg_attrs = doc_comment_args(args);
            std::mem::swap(&mut item.attrs, &mut arg_attrs);
            item.attrs.extend(arg_attrs);
        }
    }

    fn visit_item_type_mut(&mut self, item: &mut syn::ItemType) {
        if let Some(args) = self.0.get(&item.ident.to_string()) {
            let mut arg_attrs = doc_comment_args(args);
            std::mem::swap(&mut item.attrs, &mut arg_attrs);
            item.attrs.extend(arg_attrs);
        }
    }
}

impl VisitMut for DocCommentReference {
    fn visit_item_mut(&mut self, item: &mut syn::Item) {
        match item {
            syn::Item::Const(item_const) => add_ibm_reference(&mut item_const.attrs, &item_const.ident),
            syn::Item::Struct(item_struct) => add_ibm_reference(&mut item_struct.attrs, &item_struct.ident),
            syn::Item::Type(item_type) => add_ibm_reference(&mut item_type.attrs, &item_type.ident),
            _ => (),
        }

        visit_item_mut(self, item);
    }

    fn visit_foreign_item_fn_mut(&mut self, item: &mut syn::ForeignItemFn) {
        add_ibm_reference(&mut item.attrs, &item.sig.ident);
    }
}

fn add_ibm_reference(attrs: &mut Vec<syn::Attribute>, ident: &syn::Ident) {
    let references = IBM_REFERENCES.get_or_init(|| {
        let mut ibm_csv = csv::Reader::from_reader(IBM_REFERENCE_CSV.as_bytes());

        let headers = ibm_csv.headers().unwrap().clone();
        let symbol_re = regex_lite::Regex::new(r"(?i)\bmq\w+\b").unwrap();

        ibm_csv
            .records()
            .map(|record| {
                let record = record.unwrap();
                let row: Vec<_> = headers.iter().zip(record.iter()).collect();
                (
                    row.iter()
                        .find_map(|(key, value)| (*key == "name").then(|| symbol_re.find(value)))
                        .flatten()
                        .unwrap()
                        .as_str()
                        .to_string(),
                    row.iter()
                        .find_map(|(key, value)| (*key == "url").then_some(*value))
                        .unwrap()
                        .to_string(),
                )
            })
            .collect()
    });

    let ibm_url = references.get(&ident.to_string());
    if let Some(url) = ibm_url {
        let documentation_link = format!(" [IBM `{ident}` Documentation]({url})");

        // Insert the references after the last doc comment
        let mut pos = attrs.len();
        while pos != 0 && !attrs[pos - 1].meta.path().is_ident("doc") {
            pos -= 1;
        }
        if pos == 0 {
            let doc_lit = syn::LitStr::new(&documentation_link, proc_macro2::Span::call_site());
            attrs.insert(
                0,
                parse_quote!(
                    #[doc = #doc_lit]
                ),
            );
        } else {
            let doc_lit = syn::LitStr::new(&format!(" *{documentation_link}"), proc_macro2::Span::call_site());
            let comments: Vec<syn::Attribute> = parse_quote!(
                ///
                /// # References
                #[doc = #doc_lit]
            );
            attrs.splice(pos..pos, comments);
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
