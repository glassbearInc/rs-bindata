extern crate proc_macro;
extern crate proc_macro2;
#[macro_use] extern crate quote;
extern crate syn;
extern crate walkdir;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
#[allow(unused)]
use std::path::{Path, PathBuf};

use proc_macro::TokenStream;
use proc_macro2::{TokenTree, TokenNode, Delimiter, Span};
use quote::{Tokens, ToTokens};


struct Bytes(Vec<u8>);
impl ToTokens for Bytes {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let bytes = &self.0;
        let byte_tokens = quote!{ #(#bytes),* }.into();

        tokens.append(TokenTree {
            span: Span::def_site(),
            kind: TokenNode::Group(
                Delimiter::Bracket,
                byte_tokens,
            ),
        });
    }
}


#[allow(unused)]
fn read_bytes(filename: &Path) -> (String, Bytes) {
    match File::open(filename) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut contents = Vec::new();
            let _ = reader.read_to_end(&mut contents);

            (filename.to_str().unwrap().to_owned(), Bytes(contents))
        },
        Err(error) => {
            panic!(format!("could not open {:?}: {}", filename, error));
        }
    }
}

fn abs_path(path: String) -> PathBuf {
    let current = env::current_dir().unwrap();
    let mut path: PathBuf = path.into();
    if path.is_relative() {
        path = current.join(path)
    }

    path
}

fn relativize_path(path: PathBuf) -> PathBuf {
    path.strip_prefix(&env::current_dir().unwrap()).unwrap().to_path_buf()
}


fn walk(path: PathBuf) -> Vec<(String, Bytes)> {
    let mut result = Vec::new();
    let entries = walkdir::WalkDir::new(path.clone()).into_iter();

    for entry in entries {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    let (abspath, contents) = read_bytes(entry.path());
                    let relpath = relativize_path(abspath.into());
                    let relpath = relpath.to_str().unwrap();
                    result.push((relpath.to_owned(), contents));
                }
            },
            Err(error) => panic!(format!("error walking {:?}: {}", path, error)),
        }
    }

    result 
}

#[proc_macro_derive(BinDataImpl, attributes(BinDataImplContent))]
pub fn bindata_impl(input: TokenStream) -> TokenStream {
    let mut values = Vec::new();
    let input: syn::DeriveInput = syn::parse(input).unwrap();
    let ident = input.ident;

    for attr in input.attrs {
        if let Some(meta) = attr.interpret_meta() {
            if let syn::Meta::List(attrlist) = meta {
                if attrlist.ident == "BinDataImplContent" {
                    for nested in attrlist.nested {
                        if let syn::NestedMeta::Meta(meta) = nested {
                            if let syn::Meta::NameValue(nv) = meta {
                                // TODO: Should we allow multiple terms?
                                if let syn::Lit::Str(path) = nv.lit {
                                    for (name, value) in walk(abs_path(path.value())) {
                                        values.push(quote!{ #name => Some(vec!#value), });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let expanded = quote! {
        impl #ident {
            fn get(&self, key: &str) -> Option<Vec<u8>> {
                match key {
                    #(#values)*
                    _ => None,
                }
            }
        }
    };

    expanded.into()
}
