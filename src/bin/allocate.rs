use std::io::Write;
use std::process::{Command, Stdio};

use proc_macro2::{Ident, Punct, Spacing, Span, TokenTree};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::token::Semi;
use syn::{ItemUse, Macro};

struct SolutionsFile {
    import: ItemUse,
    macro_: Macro,
}

impl SolutionsFile {
    fn insert_next_day(&mut self) -> usize {
        let next_day = self.macro_.tokens.clone().into_iter().count() / 2 + 1;
        self.macro_.tokens.extend([
            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
            TokenTree::Ident(Ident::new(&format!("day{}", next_day), Span::call_site())),
        ]);
        next_day
    }
}

impl Parse for SolutionsFile {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let import = input.parse()?;
        let macro_ = input.parse()?;
        _ = input.parse::<Semi>()?;
        Ok(Self { import, macro_ })
    }
}

impl std::fmt::Display for SolutionsFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "{}\n", self.import.to_token_stream())?;
        writeln!(f, "{};", self.macro_.to_token_stream())?;
        Ok(())
    }
}

fn rustfmt(source: String) -> String {
    let mut child = Command::new("rustfmt").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();

    let mut stdin = child.stdin.take().unwrap();
    std::thread::spawn(move || {
        stdin.write_all(source.as_bytes()).unwrap();
    });

    let output = child.wait_with_output().unwrap();
    std::str::from_utf8(&output.stdout).unwrap().to_string()
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let year = &args[1];
    let solutions_prefix = format!("src/solutions/year_{year}");
    let solutions_file_path = format!("{solutions_prefix}.rs");
    let input = std::fs::read_to_string(&solutions_file_path).unwrap();
    let mut solutions_file: SolutionsFile = syn::parse_str(&input).unwrap();
    let next_day = solutions_file.insert_next_day();
    let contents = solutions_file.to_string();
    let contents = rustfmt(contents);
    std::fs::write(&solutions_file_path, contents).unwrap();
    std::fs::write(format!("{solutions_prefix}/day{next_day}.rs"), include_str!("../../template.rs")).unwrap();
}
