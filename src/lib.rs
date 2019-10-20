extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate chrono;

use proc_macro::TokenStream;
use chrono::prelude::*;

#[proc_macro]
pub fn todo(input: TokenStream) -> TokenStream {
    // TODO: error checking
    let deadline_str = syn::parse::<syn::LitStr>(input).unwrap().value();
    let parsed_deadline = chrono::NaiveDate::parse_from_str(&deadline_str, "%Y-%m-%d");

    match parsed_deadline {
        Ok(deadline) => {
            if Local::today().naive_local() > deadline {
                panic!("Tsk tsk. You missed your deadline");
            }
            },
        Err(e) => panic!("The todo! macro failed to parse the input string to a date: {}", e)
    }

    TokenStream::new()
}
