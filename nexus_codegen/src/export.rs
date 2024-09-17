use crate::addon::AddonInfo;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::env;
use syn::Expr;

fn env_var(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{key} not set"))
}

fn expr_or(expr: &Option<Expr>, default: impl FnOnce() -> TokenStream) -> TokenStream {
    expr.as_ref()
        .map(ToTokens::to_token_stream)
        .unwrap_or_else(default)
}

fn as_char_ptr(value: impl ToTokens) -> TokenStream {
    quote! { ::std::primitive::str::as_ptr(concat!(#value, "\0")).cast()}
}

impl AddonInfo {
    pub fn generate_name(&self) -> TokenStream {
        self.name
            .as_ref()
            .map(|name| name.to_token_stream())
            .unwrap_or_else(|| env_var("CARGO_PKG_NAME").to_token_stream())
    }

    pub fn generate_version(&self) -> TokenStream {
        let major = env_var("CARGO_PKG_VERSION_MAJOR")
            .parse::<i16>()
            .expect("crate version not number");
        let minor = env_var("CARGO_PKG_VERSION_MINOR")
            .parse::<i16>()
            .expect("crate version not number");
        let build = env_var("CARGO_PKG_VERSION_PATCH")
            .parse::<i16>()
            .expect("crate version not number");

        quote! {
            ::nexus::addon::AddonVersion {
                major: #major,
                minor: #minor,
                build: #build,
                revision: 0,
            }
        }
    }

    pub fn generate_load(&self) -> TokenStream {
        self.load
            .as_ref()
            .map(|load| {
                quote! {
                    const __LOAD: ::nexus::addon::AddonLoad = #load;
                    __LOAD();
                }
            })
            .unwrap_or_default()
    }

    pub fn generate_unload(&self) -> TokenStream {
        self.unload
            .as_ref()
            .map(|unload| {
                quote! {
                    const __UNLOAD: ::nexus::addon::AddonUnload = #unload;
                    __UNLOAD();
                }
            })
            .unwrap_or_default()
    }

    pub fn generate_update_link(&self) -> TokenStream {
        self.update_link
            .as_ref()
            .map(as_char_ptr)
            .unwrap_or_else(|| quote! { ::std::ptr::null() })
    }

    pub fn generate_log_filter(&self) -> TokenStream {
        self.log_filter
            .as_ref()
            .map(|e| {
                quote! { ::std::option::Option::Some(#e) }
            })
            .unwrap_or_else(|| quote! { ::std::option::Option::None })
    }

    pub fn generate_export(&self) -> TokenStream {
        let signature = &self.signature;
        let name = self.generate_name();
        let name_ptr = as_char_ptr(&name);
        let author = as_char_ptr(env_var("CARGO_PKG_AUTHORS"));
        let description = as_char_ptr(env_var("CARGO_PKG_DESCRIPTION"));
        let version = self.generate_version();
        let log_filter = self.generate_log_filter();

        let load = self.generate_load();
        let unload = self.generate_unload();

        let flags = expr_or(&self.flags, || quote! { ::nexus::addon::AddonFlags::None });
        let provider = expr_or(
            &self.provider,
            || quote! { ::nexus::addon::UpdateProvider::None },
        );
        let update_link = self.generate_update_link();

        quote! {
            mod __nexus_addon_export {
                use super::*;

                const __ADDON_NAME: &'static ::std::primitive::str = #name;

                static __ADDON_DEF: ::nexus::addon::AddonDefinition = ::nexus::addon::AddonDefinition {
                    signature: #signature,
                    api_version: ::nexus::AddonApi::VERSION,
                    name: #name_ptr,
                    version: #version,
                    author: #author,
                    description: #description,
                    load: self::__load_wrapper,
                    unload: ::std::option::Option::Some(self::__unload_wrapper),
                    flags: #flags,
                    provider: #provider,
                    update_link: #update_link,
                };

                #[no_mangle]
                unsafe extern "system-unwind" fn GetAddonDef() -> *const ::nexus::addon::AddonDefinition {
                    &self::__ADDON_DEF
                }

                unsafe extern "C-unwind" fn __load_wrapper(api: *const ::nexus::AddonApi) {
                    ::nexus::__macro::init(api, self::__ADDON_NAME, #log_filter);
                    #load
                }

                unsafe extern "C-unwind" fn __unload_wrapper() {
                    #unload
                    ::nexus::__macro::deinit();
                }
            }
        }
    }
}
