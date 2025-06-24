use std::{borrow::Cow, collections::HashMap};

use regex_lite::{Captures, Regex};
use syn::{parse_quote, visit_mut::VisitMut, Attribute};

const BARE_REGEX: &str = r"(?msx)
    /\*+/
    \s+/\*\s+([A-Za-z_0-9]+)\s+(?:Structure\s+|Function\s+)?--?\s
    (.+?)\s+\*/
    \s*/\*+/
";
const SPLIT_REGEX: &str = r"(?m)\s*(\*/|/\*)\s*";

const MQ_ITEM_REGEX: &str = r"MQ[A-Z\d_]+";

fn doc_link(c: &Captures) -> Cow<'static, str> {
    Cow::Owned(format!("[`{}`]", &c[0]))
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

const IBM_REFERENCE: &[(&str, &str)] = &[
    ("mqPad", "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqpad"),
    ("mqTrim", "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqtrim"),
    (
        "MQXEP",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=reference-exit-entry-point-registration-call-mqxep",
    ),
    (
        "MQXCLWLN",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=structures-mqxclwln-navigate-cluster-workload-records",
    ),
    (
        "MQZEP",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=information-mqzep-add-component-entry-point",
    ),
    (
        "MQCONNX",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqconnx-connect-queue-manager-extended",
    ),
    (
        "MQCONN",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqconn-connect-queue-manager",
    ),
    (
        "MQDISC",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqdisc-disconnect-queue-manager",
    ),
    (
        "MQOPEN",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqopen-open-object",
    ),
    (
        "MQPUT1",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqput1-put-one-message",
    ),
    (
        "MQCLOSE",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqclose-close-object",
    ),
    (
        "MQCMIT",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqcmit-commit-changes",
    ),
    (
        "MQGET",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqget-get-message",
    ),
    (
        "MQPUT",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqput-put-message",
    ),
    (
        "MQINQ",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinq-inquire-object-attributes",
    ),
    (
        "MQSUB",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsub-register-subscription",
    ),
    (
        "MQSUBRQ",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsubrq-subscription-request",
    ),
    (
        "MQBEGIN",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqbegin-begin-unit-work",
    ),
    (
        "MQBACK",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqback-back-out-changes",
    ),
    (
        "MQCRTMH",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqcrtmh-create-message-handle",
    ),
    (
        "MQDLTMH",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqdltmh-delete-message-handle",
    ),
    (
        "MQMHBUF",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqmhbuf-convert-message-handle-into-buffer",
    ),
    (
        "MQBUFMH",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqbufmh-convert-buffer-into-message-handle",
    ),
    (
        "MQCB",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqcb-manage-callback",
    ),
    (
        "MQCTL",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqctl-control-callbacks",
    ),
    (
        "MQSET",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqset-set-object-attributes",
    ),
    (
        "MQSETMP",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetmp-set-message-property",
    ),
    (
        "MQSTAT",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqstat-retrieve-status-information",
    ),
    (
        "MQINQMP",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinqmp-inquire-message-property",
    ),
    (
        "MQDLTMP",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqdltmp-delete-message-property",
    ),
    (
        "MQXCNVC",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=exit-mqxcnvc-convert-characters",
    ),
    (
        "mqCreateBag",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqcreatebag",
    ),
    (
        "mqClearBag",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqclearbag",
    ),
    (
        "mqDeleteBag",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqdeletebag",
    ),
    ("mqGetBag", "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqgetbag"),
    ("mqPutBag", "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqputbag"),
    (
        "mqTruncateBag",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqtruncatebag",
    ),
    (
        "mqAddInquiry",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddinquiry",
    ),
    (
        "mqDeleteItem",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqdeleteitem",
    ),
    (
        "mqAddInteger",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddinteger",
    ),
    (
        "mqAddIntegerFilter",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddintegerfilter",
    ),
    (
        "mqAddInteger64",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddinteger64",
    ),
    (
        "mqAddString",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddstring",
    ),
    (
        "mqAddStringFilter",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddstringfilter",
    ),
    (
        "mqAddByteString",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddbytestring",
    ),
    (
        "mqAddByteStringFilter",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddbytestringfilter",
    ),
    (
        "mqSetInteger",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetinteger",
    ),
    (
        "mqSetIntegerFilter",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetintegerfilter",
    ),
    (
        "mqSetInteger64",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetinteger64",
    ),
    ("mqAddBag", "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddbag"),
    (
        "mqSetString",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetstring",
    ),
    (
        "mqSetStringFilter",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetstringfilter",
    ),
    (
        "mqSetByteString",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetbytestring",
    ),
    (
        "mqSetByteStringFilter",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetbytestringfilter",
    ),
    (
        "mqInquireInteger",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquireinteger",
    ),
    (
        "mqInquireIntegerFilter",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquireintegerfilter",
    ),
    (
        "mqInquireInteger64",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquireinteger64",
    ),
    (
        "mqInquireByteString",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquirebytestring",
    ),
    (
        "mqInquireString",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquirestring",
    ),
    (
        "mqInquireStringFilter",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquirestringfilter",
    ),
    (
        "mqInquireByteStringFilter",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquirebytestringfilter",
    ),
    (
        "mqInquireBag",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquirebag",
    ),
    (
        "mqCountItems",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqcountitems",
    ),
    ("mqExecute", "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqexecute"),
    (
        "mqBagToBuffer",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqbagtobuffer",
    ),
    (
        "mqBufferToBag",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqbuffertobag",
    ),
    (
        "mqInquireItemInfo",
        "https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquireiteminfo",
    ),
];

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
pub struct DocCommentFields<'a>(pub &'a HashMap<String, HashMap<String, String>>);

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
    type Extracted = HashMap<String, String>;

    fn extract(&self, file_content: &str) -> impl Iterator<Item = (String, Self::Extracted)> {
        self.struct_search.captures_iter(file_content).map(|c| {
            (
                c[1].trim_start_matches("tag").to_string(), // Struct name
                self.field_search
                    .captures_iter(&c[2])
                    .map(|c| {
                        let description = self
                            .delim
                            .split(&c[2])
                            .filter(|s| !s.is_empty() && !s.starts_with("Ver:"))
                            .collect::<Vec<_>>()
                            .join(" ");
                        let description = self.mq_item.replace_all(&description, doc_link);

                        (
                            c[1].to_string(), // Field name
                            replace_keywords(&description),
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
        if let Some(fields) = self.0.get(&item.ident.to_string()) {
            for field in &mut item.fields {
                if let Some(description) = fields.get(&field.ident.as_ref().unwrap().to_string()) {
                    let doc_lit = syn::LitStr::new(&format!(" {description}"), proc_macro2::Span::call_site());
                    field.attrs.insert(0, syn::parse_quote!(#[doc = #doc_lit ]));
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
    fn visit_foreign_item_fn_mut(&mut self, item: &mut syn::ForeignItemFn) {
        let ibm_url = IBM_REFERENCE
            .iter()
            .find_map(|(name, url)| (item.sig.ident == *name).then_some(*url));
        if let Some(url) = ibm_url {
            let doc_lit = syn::LitStr::new(&format!(" * [IBM Documentation]({url})"), proc_macro2::Span::call_site());
            let comments: Vec<syn::Attribute> = parse_quote!(
                ///
                /// # References
                #[doc = #doc_lit]
            );
            item.attrs.extend(comments);
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
