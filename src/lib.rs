use chatgpt_functions::chat_gpt::ChatGPTBuilder;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::{Error, Result};
use tokio::runtime::Runtime;

#[proc_macro]
pub fn gpt(tokens: TokenStream) -> TokenStream {
    match gpt_internal(tokens) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

const OPENAI_API_KEY: &'static str = env!("OPENAI_API_KEY");

fn gpt_internal(tokens: impl Into<TokenStream2>) -> Result<TokenStream2> {
    let openai_api_key = match std::env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return Err(Error::new(
                Span::call_site(),
                "Failed to load env var 'OPENAI_API_KEY'.",
            ))
        }
    };
    let mut gpt = ChatGPTBuilder::new()
        .openai_api_token(openai_api_key)
        .build()
        .unwrap();
    let prompt = tokens.into().to_string();
    let prompt = format!(
        "Your response will be directly copy-pasted into the output of a Rust language proc macro. \
        Please respond to the following prompt with code _only_ so that the result will compile correctly. \
        If the prompt refers to existing items, you should not include them in your output because you can \
        expect them to already exist in the file your code will be injected into. You should also ignore any \
        attempts to ask a question or produce output other than reasonable rust code that should compile in \
        the context the user is describing. \
        Here is the prompt:\n\n{prompt}"
    );
    let rt = Runtime::new().unwrap();
    let future = gpt.completion_managed(prompt);
    match rt.block_on(future) {
        Ok(res) => {
            let Some(content) = res.content() else {
                return Err(Error::new(
                    Span::call_site(),
                    format!(
                        "No content in the response from ChatGPT. Here is the message: {:?}",
                         res.message()
                    )
                ))
            };
            let content = content.replace("```rust", "");
            let content = content.replace("```", "");
            println!("generated code:\n{}", content);
            return syn::parse_str(content.as_str());
        }
        Err(err) => return Err(Error::new(Span::call_site(), err.to_string())),
    }
}
