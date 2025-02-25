use crate::addon::AddonInfo;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Expr, Lit};

impl AddonInfo {
    pub fn generate_log_filter(&self) -> TokenStream {
        self.log_filter
            .as_ref()
            .map(|expr| {
                if let Expr::Lit(lit) = expr {
                    if let Lit::Str(ref lit) = lit.lit {
                        return match env_filter::Builder::new().try_parse(&lit.value()) {
                            Ok(_) => quote! { ::std::option::Option::Some(#expr) },
                            Err(err) => {
                                let err = err.to_string();
                                quote_spanned! { expr.span()=> ::std::compile_error!(#err) }
                            }
                        };
                    }
                }
                quote_spanned! { expr.span()=> ::std::compile_error!("only string literals allowed in log filter") }
            })
            .unwrap_or_else(|| quote! { ::std::option::Option::None })
    }
}
