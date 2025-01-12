use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

fn to_snake_case(variant_name: &str) -> String {
    let mut result = String::new();
    let mut prev_is_lowercase = false;

    for ch in variant_name.chars() {
        if ch.is_uppercase() && prev_is_lowercase {
            result.push('_');
        }
        result.push(ch.to_ascii_lowercase());
        prev_is_lowercase = ch.is_lowercase();
    }

    result
}

#[proc_macro_derive(GenerateMailSchemas)]
pub fn generate_mail_schemas(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut schemas = Vec::new();
    let mut template_constants = Vec::new();

    if let Data::Enum(data_enum) = input.data {
        for variant in data_enum.variants {
            let variant_name = variant.ident.clone();
            let field_names = if let Fields::Named(fields) = variant.fields {
                fields
                    .named
                    .iter()
                    .map(|f| f.ident.clone().unwrap())
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };

            let template_constant_name_str = to_snake_case(&variant_name.to_string()).to_uppercase();
            let template_constant_name = format_ident!("{}_TEMPLATE", template_constant_name_str);
            let template_name_string = to_snake_case(&variant_name.to_string());

            template_constants.push(quote! {
                pub const #template_constant_name: &'static str = #template_name_string;
            });

            let schema = quote! {
                mail_template::MailSchema {
                    template_name: Self::#template_constant_name.to_string(),
                    parameters: vec![#(stringify!(#field_names).to_string()),*],
                }
            };
            schemas.push(schema);
        }
    }

    let expanded = quote! {
        impl #name {
            #(#template_constants)*

            pub fn get_mail_schemas() -> Vec<mail_template::MailSchema> {
                vec![#(#schemas),*]
            }
        }
    };

    TokenStream::from(expanded)
}
