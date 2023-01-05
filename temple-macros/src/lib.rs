use proc_macro::TokenStream;

fn fail_compilation(msg: &str) -> TokenStream {
    let quoted_msg = format!("{:?}", msg);
    TokenStream::from(quote::quote! {
        compile_error!(#quoted_msg)
    })
}

struct TemplateArgs {
    path: String,
}

impl syn::parse::Parse for TemplateArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args_input: syn::parse::ParseBuffer;
        let _ = syn::parenthesized!(args_input in input);
        let results = args_input
            .parse_terminated::<syn::NestedMeta, syn::Token![,]>(syn::NestedMeta::parse)?;

        if results.len() == 1 {
            if let syn::NestedMeta::Lit(syn::Lit::Str(s)) = &results[0] {
                return Ok(Self { path: s.value() });
            }
        }

        Err(syn::Error::new(
            input.span(),
            "invalid 'template' attribute args",
        ))
    }
}

fn parse_macro_file(path: &str) -> Result<Vec<temple_common::parse::Node>, TokenStream> {
    let data = match std::fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) => return Err(fail_compilation(e.to_string().as_str())),
    };

    let mut parser = temple_common::parse::Parser::new(&data);
    parser
        .parse_nodes()
        .map_err(|e| fail_compilation(e.to_string().as_str()))
}

fn parse_template(
    path: &str,
    render_ident: &str,
) -> Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
    let mut raw = String::new();
    for node in parse_macro_file(path)? {
        match node {
            temple_common::parse::Node::Render(s) => {
                raw.push_str(&format!("{}.render(&mut {})?; ", s, render_ident))
            }
            temple_common::parse::Node::Content(s) => raw.push_str(&format!(
                "if let Err(_) = {}.write_str({:?}) {{ return Err(()); }}",
                render_ident, s
            )),
            temple_common::parse::Node::Control(s) => raw.push_str(&s),
        }
    }

    syn::parse_str::<proc_macro2::TokenStream>(&raw).map_err(|e| e.into_compile_error())
}

#[proc_macro_derive(Template, attributes(template))]
pub fn derive_template(input: TokenStream) -> TokenStream {
    let info = syn::parse_macro_input!(input as syn::DeriveInput);
    let args = match info.attrs.iter().find(|a| a.path.is_ident("template")) {
        Some(a) => match syn::parse2::<TemplateArgs>(a.tokens.clone()) {
            Ok(args) => args,
            Err(e) => return e.to_compile_error().into(),
        },
        None => return fail_compilation("missing 'template' attribute"),
    };

    let type_name = info.ident;
    let self_ident = quote::format_ident!("{}", "self");
    let renderer_ident = quote::format_ident!("{}", "renderer");
    let (i_generics, t_generics, where_clause) = info.generics.split_for_impl();

    let abs_template_path = match std::fs::canonicalize(&args.path) {
        Ok(p) => p.to_string_lossy().to_string(),
        Err(e) => return fail_compilation(e.to_string().as_str()),
    };
    let template = match parse_template(&abs_template_path, &renderer_ident.to_string()) {
        Ok(t) => t,
        Err(e) => return TokenStream::from(e),
    };

    TokenStream::from(quote::quote! {
        impl #i_generics ::temple::Template for #type_name #t_generics #where_clause {
            const TEMPLATE_DATA: &'static str = include_str!(#abs_template_path);
        }

        impl #i_generics ::temple::Renderable for #type_name #t_generics #where_clause {
            fn render<R: ::temple::Renderer>(&#self_ident, mut #renderer_ident: R) -> ::temple::Result {
                use ::temple::{Renderable, Renderer};
                #template
                Ok(())
            }
        }
    })
}
