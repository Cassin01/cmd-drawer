use confy;
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect};
use serde_derive::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct Conf {
    zsh: Vec<Cmd>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Cmd {
    cmd: String,
    desc: String,
}

impl Cmd {
    fn new(cmd: &str, desc: &str) -> Self {
        Self {
            cmd: cmd.to_string(),
            desc: desc.to_string(),
        }
    }
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            zsh: vec![Cmd::new(r"(cd ./target/debug/ && pwd)", "test")],
        }
    }
}

type Res<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> Res<()> {
    let cfg: Conf = confy::load("cmd-drawer", None)?;
    let cmds = &cfg.zsh;
    let selections: Vec<String> = cmds
        .iter()
        .map(|x| format!("{}: {}", x.desc.clone(), x.cmd.clone()))
        .collect();
    println!("{:?}", selections);
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("")
        .default(0)
        .items(&selections[..])
        .interact()?;
    println!("{}", cmds[selection].cmd);
    Ok(())
}
