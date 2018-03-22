// Copyright 2018 Glassbear, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


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
use syn::{Lit, LitStr, Meta, MetaNameValue, NestedMeta, MetaList };
use quote::{Tokens, ToTokens};


struct Asset(String, Bytes);
impl ToTokens for Asset {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let &Asset(ref path, ref contents) = self;
        let asset_tokens = quote!{ #path => Some(vec!#contents), }.into();

        tokens.append(TokenTree {
            span: Span::def_site(),
            kind: TokenNode::Group(
                Delimiter::None,
                asset_tokens,
            ),
        })
    }
}


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
fn read_bytes(filename: &Path) -> Asset {
    match File::open(filename) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut contents = Vec::new();
            let _ = reader.read_to_end(&mut contents);

            Asset(filename.to_str().unwrap().to_owned(), Bytes(contents))
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

fn relativize_path(path: PathBuf) -> String {
    let current = env::current_dir().unwrap();
    let path = match path.strip_prefix(&current) {
        Ok(path) => path,
        Err(_)   => &path,
    };

    path.to_str().unwrap().to_string()
}


fn walk(path: PathBuf) -> Vec<Asset> {
    let entries = walkdir::WalkDir::new(path.clone()).into_iter();

    entries.filter_map(|e| e.ok() ) // TODO:  Silently ignore errors?
           .filter(|e| e.file_type().is_file())
           .map(|f| read_bytes(f.path()))
           .map(|Asset(path, contents)| Asset(relativize_path(path.into()), contents))
           .collect()
}

#[proc_macro_derive(BinDataImpl, attributes(BinDataImplContent))]
pub fn bindata_impl(input: TokenStream) -> TokenStream {
    fn parse_meta_list(m: Meta) -> Option<MetaList> {
        match m {
            Meta::List(meta) => Some(meta),
            _ => None,
        }
    }

    fn parse_meta_namevalue(m: Meta) -> Option<MetaNameValue> {
        match m {
            Meta::NameValue(namevalue) => Some(namevalue),
            _ => None,
        }
    }

    fn parse_nestedmeta_meta(m: NestedMeta) -> Option<Meta> {
        match m {
            NestedMeta::Meta(meta) => Some(meta),
            _ => None
        }
    }

    fn parse_string_literal(l: Lit) -> Option<LitStr> {
        match l {
            Lit::Str(string) => Some(string),
            _ => None,
        }
    }

    let input: syn::DeriveInput = syn::parse(input).unwrap();
    let ident = input.ident;
    let values: Vec<Asset> = input.attrs.iter()
                                        .filter_map(|a| a.interpret_meta())
                                        .filter(|m| m.name() == "BinDataImplContent") 
                                        .filter_map(parse_meta_list)
                                        .flat_map(|bindata|
                                             bindata.nested.into_iter()
                                                           .filter_map(parse_nestedmeta_meta)
                                                           .filter_map(parse_meta_namevalue)
                                                           .map(|nv| nv.lit)
                                                           .filter_map(parse_string_literal))
                                                           .map(|l| l.value())
                                        .flat_map(|path| walk(abs_path(path)))
                                        .collect();
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
