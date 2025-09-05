use crate::addon::AddonInfo;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::env;
use syn::Expr;

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
        let pre = env_var("CARGO_PKG_VERSION_PRE");

        let rev: i16 = Self::get_revision(&pre);

        quote! {
            ::nexus::addon::AddonVersion {
                major: #major,
                minor: #minor,
                build: #build,
                revision: #rev,
            }
        }
    }

    /// Revision for stable versions.
    ///
    /// Negative values hide the revision number in nexus.
    const STABLE_REVISION: i16 = -1;

    fn get_revision(pre: &str) -> i16 {
        fn as_ascii_byte(identifier: &str) -> u8 {
            let first = identifier
                .chars()
                .next()
                .expect("empty pre-release identifier");
            if first.is_ascii() {
                first as u8
            } else {
                panic!("non-ascii pre-release identifier")
            }
        }

        let mut pre = pre.split('.');
        if let Some(first) = pre.next().filter(|part| !part.is_empty()) {
            let first = (as_ascii_byte(first) as i16) << 8;
            let pre_rev = if let Some(second) = pre.next() {
                let second = second
                    .parse::<u8>()
                    .ok()
                    .unwrap_or_else(|| as_ascii_byte(second));
                let second = second as i16;
                first + second
            } else {
                first
            };
            i16::MIN
                + pre_rev
                    .checked_sub(1)
                    .expect("null character in pre-release")
        } else {
            Self::STABLE_REVISION
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

    pub fn generate_export(&self) -> TokenStream {
        let signature = &self.signature;
        let name = self.generate_name();
        let name_ptr = as_char_ptr(&name);
        let author = as_char_ptr(env_var("CARGO_PKG_AUTHORS"));
        let description = as_char_ptr(env_var("CARGO_PKG_DESCRIPTION"));
        let version = self.generate_version();

        #[cfg(feature = "log_filter")]
        let log_filter = self.generate_log_filter();

        #[cfg(not(feature = "log_filter"))]
        let log_filter = quote! { ::std::option::Option::None };

        let initfn = {
            quote! { ::nexus::__macro::init(api, self::__ADDON_NAME, #log_filter); }
        };

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
                    #initfn
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

#[cfg(test)]
mod tests {
    use super::*;

    // sanity check for possible pre-release revision range
    #[test]
    fn semver_revision_range() {
        // non-null ascii
        let first_min = 1;
        let first_max = 0x7F << 8;

        // byte number or non-null ascii
        let second_min = 0;
        let second_max = 0xFF;

        assert!(first_min + second_min > 0);
        assert!(i16::MIN + first_max + second_max - 1 < AddonInfo::STABLE_REVISION);
    }

    #[test]
    fn semver_revisions() {
        let alpha0 = AddonInfo::get_revision("alpha.0");
        let alpha1 = AddonInfo::get_revision("alpha.1");
        let alpha2 = AddonInfo::get_revision("alpha.2");
        let beta0 = AddonInfo::get_revision("beta.0");
        let pre0 = AddonInfo::get_revision("pre.0");
        let rc0 = AddonInfo::get_revision("rc.0");
        let x_a = AddonInfo::get_revision("x.a");
        let x_b = AddonInfo::get_revision("x.b");

        assert!(alpha0 < alpha1);
        assert!(alpha1 < alpha2);
        assert!(alpha2 < beta0);
        assert!(beta0 < pre0);
        assert!(pre0 < rc0);
        assert!(rc0 < x_a);
        assert!(x_a < x_b);
        assert!(x_b < AddonInfo::STABLE_REVISION)
    }
}
