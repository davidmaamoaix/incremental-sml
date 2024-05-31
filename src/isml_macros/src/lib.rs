use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{ Span, TokenStream };
use quote::quote;
use syn::parse::{ Parse, ParseStream, Result };
use syn::{ Ident, punctuated::Punctuated, LitStr, Token };

enum KeywordDef {
    // Identifier is the uppercased version of the literal.
    // Stored string should be lowercase, which is then converted to upper.
    IdentSameAsLiteral(String),
    // (<identifier (probably uppercase)>, <token literal>)
    IdentAndLiteral(String, String)
}

impl KeywordDef {
    fn ident(&self) -> String {
        match self {
            KeywordDef::IdentSameAsLiteral(s) => s.to_uppercase(),
            KeywordDef::IdentAndLiteral(ident, _) => ident.clone()
        }
    }

    fn literal(&self) -> String {
        match self {
            KeywordDef::IdentSameAsLiteral(s) => s.clone(),
            KeywordDef::IdentAndLiteral(_, lit) => lit.clone()
        }
    }
}

impl Parse for KeywordDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit: LitStr = input.parse()?;
        if let Ok(_) = input.parse::<Token![:]>() {
            let iden: Ident = input.parse()?;
            let kw = KeywordDef::IdentAndLiteral(
                iden.to_string(), lit.value()
            );
            return Ok(kw);
        }

        return Ok(KeywordDef::IdentSameAsLiteral(lit.value()));
    }
}

struct Input(Punctuated<KeywordDef, Token![,]>);

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Input(Punctuated::parse_terminated(input)?))
    }
}

#[proc_macro]
pub fn gen_keyword_defs(input: TokenStream1) -> TokenStream1 {
    internal_gen_keyword_defs(input.into()).into()
}

fn internal_gen_keyword_defs(tokens: TokenStream) -> TokenStream {
    let input = match syn::parse2::<Input>(tokens) {
        Ok(val) => val,
        Err(e) => return e.to_compile_error()
    };

    let mut enum_stream = quote!{};
    let mut map_stream = quote!{};
    let mut display_stream = quote!{};

    for kw in input.0.iter() {
        let ident = Ident::new(&kw.ident(), Span::call_site());
        let lit = LitStr::new(&kw.literal(), Span::call_site());

        enum_stream.extend(quote!{
            #ident,
        });

        map_stream.extend(quote!{
            #lit => Kw::#ident,
        });

        display_stream.extend(quote!{
            #ident => #lit,
        });
    }

    return quote!{
        use phf::{ phf_map, Map };

        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        pub enum Kw {
            #enum_stream
        }

        impl std::fmt::Display for Kw {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let repr = match self {
                    #display_stream
                };

                write!(f, "{}", repr)
            }
        }

        pub static LIT_TO_KW: Map<&'static str, Kw> = phf_map! {
            #map_stream
        };
    };
}
