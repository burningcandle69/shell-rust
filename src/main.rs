mod autocomplete;
mod builtins;
mod command;
mod shell;
mod trie;

use crate::autocomplete::ShellAutocomplete;
use crate::shell::Shell;
use inquire::ui::{RenderConfig, Styled};
use inquire::Text;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut shell = Shell::new();
    let autocomplete = ShellAutocomplete::new(&shell.path);

    let mut render_config = RenderConfig::empty();
    let stl = Styled::new("$");
    render_config.prompt_prefix = stl;
    render_config.answered_prompt_prefix = stl;

    let prompt = Text::new("")
        .with_autocomplete(autocomplete)
        .with_render_config(render_config)
        .with_formatter(&|s| s.to_string());

    // let txt = prompt.prompt();

    loop {
        let prompt = prompt.clone();

        let input = prompt.prompt().unwrap();

        shell.execute(input).err().and_then(|e| {
            println!("{e}");
            Option::<String>::None
        });
    }
}
