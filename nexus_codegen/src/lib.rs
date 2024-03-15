mod addon;
mod export;

use self::addon::AddonInfo;
use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Creates addon exports for Raidcore Nexus.
#[proc_macro]
pub fn export(input: TokenStream) -> TokenStream {
    let addon = parse_macro_input!(input as AddonInfo);
    addon.generate_export().into()
}
