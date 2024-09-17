use proc_macro2::{Span, TokenStream};
use syn::{parse::Parse, punctuated::Punctuated, Error, Expr, FieldValue, Member, Token};

pub struct AddonInfo {
    pub signature: Expr,
    pub name: Option<Expr>,
    pub load: Option<Expr>,
    pub unload: Option<Expr>,
    pub flags: Option<Expr>,
    pub provider: Option<Expr>,
    pub update_link: Option<Expr>,
    pub log_filter: Option<Expr>,
}

impl AddonInfo {
    fn populate_from_fields(
        mut self,
        span: Span,
        fields: impl IntoIterator<Item = FieldValue>,
    ) -> syn::Result<Self> {
        let mut found_signature = false;

        for field in fields {
            if let Member::Named(ident) = &field.member {
                match ident.to_string().as_str() {
                    "signature" => {
                        found_signature = true;
                        self.signature = field.expr;
                    }
                    "name" => self.name = Some(field.expr),
                    "load" => self.load = Some(field.expr),
                    "unload" => self.unload = Some(field.expr),
                    "flags" => self.flags = Some(field.expr),
                    "provider" => self.provider = Some(field.expr),
                    "update_link" => self.update_link = Some(field.expr),
                    "log_filter" => self.log_filter = Some(field.expr),
                    _ => return Err(Error::new_spanned(ident, "unknown field {ident}")),
                }
            } else {
                return Err(Error::new_spanned(&field.member, "field must have a name"));
            }
        }

        if !found_signature {
            return Err(Error::new(span, "missing signature field"));
        }

        Ok(self)
    }
}

impl Default for AddonInfo {
    fn default() -> Self {
        Self {
            signature: Expr::Verbatim(TokenStream::new()),
            name: None,
            load: None,
            unload: None,
            flags: None,
            provider: None,
            update_link: None,
            log_filter: None,
        }
    }
}

impl Parse for AddonInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fields = Punctuated::<FieldValue, Token![,]>::parse_terminated(input)?;
        Self::default().populate_from_fields(input.span(), fields)
    }
}
