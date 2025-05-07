#[doc = include_str!("../README.md")]
use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use titlecase::titlecase;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Info {
    documentation: String,
    api_scope: String,
    valid_rust_name: String,
    original_name: String,
}

#[proc_macro]
/// A procedural macro for generating group scope enums
pub fn generate_grouped_scopes_enums(_: TokenStream) -> TokenStream {
    let trait_for_scopes = quote! {
        use tlns_google_oauth2_traits::{*};
    };

    let content = include_str!("../info.txt");

    let mut stuffs: HashMap<String, Vec<Info>> = HashMap::new();
    let mut current_header = String::new();
    let mut original_header_name = String::new();
    let check = ["openid", "profile", "email", "http"];

    for line in content.lines() {
        if line == "Scopes" {
            continue;
        }

        let mut found_scope = false;
        for need in &check {
            if line.contains(need) {
                found_scope = true;
                // Scope
                let valided_rust: String;
                let doc: String;
                let api_scope_thing: String;
                let mut splitted = line.split("\t");
                api_scope_thing = splitted.next().unwrap().to_string();
                doc = splitted.next().unwrap().to_string();
                if !line.contains("http") {
                    // Not scope with HTTP
                    valided_rust = titlecase(&api_scope_thing);
                } else {
                    // Scope with HTTP
                    let parsed = Url::parse(&api_scope_thing).unwrap();
                    let paths: Vec<&str> = parsed.path_segments().unwrap().collect();
                    if paths.len() >= 2 {
                        let paths_slice = &paths[paths.len() - 2..];
                        let joined = paths_slice.join(" ");
                        let joined = joined
                            .replace("-", " ")
                            .replace(".", " ")
                            .replace("read", "Read")
                            .replace("only", "Only")
                            .replace("(", "")
                            .replace(")", "");
                        let titlecased = titlecase(&joined);
                        valided_rust = titlecased.split_whitespace().collect::<String>();
                    } else {
                        if api_scope_thing.contains("mail") {
                            valided_rust = "GMail".to_string();
                        } else {
                            panic!("Weird scope: {api_scope_thing}")
                        }
                    }
                }
                stuffs
                    .entry(current_header.to_string())
                    .or_insert_with(Vec::new)
                    .push(Info {
                        documentation: doc,
                        api_scope: api_scope_thing,
                        valid_rust_name: valided_rust,
                        original_name: original_header_name.clone(),
                    });
                break;
            }
        }
        if !found_scope && line.chars().nth(0).unwrap().is_ascii_uppercase() {
            current_header = line
                .replace(",", "")
                .replace("-", "")
                .replace(".", "Point")
                .replace("(", "")
                .replace(")", "")
                .replace("/", "")
                .replace("&", "")
                .split(" ")
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join("");
            original_header_name = line.to_string();
        }
    }
    let mut impl_scope_to: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut impl_scope_from: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut enums: Vec<proc_macro2::TokenStream> = Vec::new();
    for (k, v) in stuffs.iter_mut() {
        let name = syn::Ident::new(k, proc_macro2::Span::call_site());
        let variants = v
            .iter()
            .map(|i| syn::Ident::new(&i.valid_rust_name, proc_macro2::Span::call_site()))
            .collect::<Vec<proc_macro2::Ident>>();
        let scope_variants = v.iter().map(|i| &i.api_scope).collect::<Vec<&String>>();
        let doc_variants = v
            .iter()
            .map(|i| {
                let fd = format!("Documentation: {}, Scope: {}", i.documentation, i.api_scope);
                quote! {
                    #[doc = #fd]
                }
            })
            .collect::<Vec<proc_macro2::TokenStream>>();
        let orig = v.first().unwrap().original_name.clone();
        let enum_doc = quote! {
            #[doc = #orig]
        };
        enums.push(quote! {
                #enum_doc
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                pub enum #name {
                    #(#doc_variants
                    #variants),*
                }
        });
        impl_scope_to.push(quote! {
            impl ToGoogleScope for #name {
                fn to_google_scope(&self) -> &'static str {
                    match self {
                        #(Self::#variants => #scope_variants),*
                    }
                }
            }
        });
        impl_scope_from.push(quote! {
            impl FromGoogleScope<#name> for #name {
                fn from_google_scope(google_scope: &str) -> Result<#name, ()> {
                    match google_scope {
                        #(scope_variants => Ok(#name::#variants)),*,
                        _ => Err(())
                    }
                }
            }
        });
    }

    let output = quote! {
        #trait_for_scopes
        #(#enums)*
        #(#impl_scope_from)*
        #(#impl_scope_to)*
    };
    output.into()
}

#[proc_macro]
/// A procedural macro for generating scope enums
pub fn generate_scopes_enums(_: TokenStream) -> TokenStream {
    let trait_for_scopes = quote! {
        use tlns_google_oauth2_traits::{*};
        use std::str::FromStr;
    };

    let content = include_str!("../info.txt");

    let mut scopes_map: HashMap<String, (String, String, String)> = HashMap::new();
    let check = vec!["openid", "profile", "email", "http"];

    for line in content.lines() {
        if line == "Scopes" {
            continue;
        }

        for need in &check {
            if line.contains(need) {
                let valided_rust: String;
                let doc: String;
                let api_scope_thing: String;
                let mut splitted = line.split("\t");
                api_scope_thing = splitted.next().unwrap().to_string();
                doc = splitted.next().unwrap().to_string();
                if !line.contains("http") {
                    valided_rust = titlecase(&api_scope_thing);
                } else {
                    let parsed = Url::parse(&api_scope_thing).unwrap();
                    let paths: Vec<&str> = parsed.path_segments().unwrap().collect();
                    if paths.len() >= 2 {
                        let paths_slice = &paths[paths.len() - 2..];
                        let joined = paths_slice.join(" ");
                        let joined = joined
                            .replace("-", " ")
                            .replace(".", " ")
                            .replace("read", "Read")
                            .replace("only", "Only")
                            .replace("(", "")
                            .replace(")", "");
                        let titlecased = titlecase(&joined);
                        valided_rust = titlecased.split_whitespace().collect::<String>();
                    } else {
                        if api_scope_thing.contains("mail") {
                            valided_rust = "GMail".to_string();
                        } else {
                            panic!("Weird scope: {}", api_scope_thing)
                        }
                    }
                }
                scopes_map.entry(api_scope_thing.clone()).or_insert((
                    valided_rust,
                    doc,
                    api_scope_thing,
                ));
                break;
            }
        }
    }

    let enum_variants = scopes_map
        .iter()
        .map(|(_, (valided_rust, doc, api_scope_thing))| {
            let fd = format!("Documentation: {doc}, Scope: {api_scope_thing}");
            let variant = syn::Ident::new(valided_rust, proc_macro2::Span::call_site());
            quote! {
                #[doc = #fd]
                #variant,
            }
        });

    let to_google_scope_impl = scopes_map
        .iter()
        .map(|(_, (valided_rust, _, api_scope_thing))| {
            let variant = syn::Ident::new(valided_rust, proc_macro2::Span::call_site());
            quote! {
                Scopes::#variant => #api_scope_thing,
            }
        });

    let from_google_scope_impl =
        scopes_map
            .iter()
            .map(|(_, (valided_rust, _, api_scope_thing))| {
                let variant = syn::Ident::new(valided_rust, proc_macro2::Span::call_site());
                quote! {
                    #api_scope_thing => Ok(Scopes::#variant),
                }
            });

    let cloned_from_google_scope_impl = from_google_scope_impl.clone();

    let expanded = quote! {
        #trait_for_scopes

        /// A bunch of scopes merged into one enum (use [`crate::grouped_scopes`] for scopes grouped by their header enums)
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Scopes {
            #(#enum_variants)*
        }

        impl ToGoogleScope for Scopes {
            fn to_google_scope(&self) -> &'static str {
                match self {
                    #(#to_google_scope_impl)*
                }
            }
        }

        impl FromGoogleScope<Scopes> for Scopes {
            fn from_google_scope(google_scope: &str) -> Result<Scopes, ()> {
                match google_scope {
                    #(#from_google_scope_impl)*
                    _ => Err(()),
                }
            }
        }

        impl FromStr for Scopes {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#cloned_from_google_scope_impl)*
                    _ => Err(()),
                }
            }
        }
    };

    expanded.into()
}
