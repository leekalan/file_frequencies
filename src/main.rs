use std::{env, fs::File, path::Path};

use file_explorer::{boot, truncate_path, CanonicalPath};
use roped::{
    command::Command, console::run_console, parsr::parser_matcher::MatcherSingle, EmptyState,
    Matcher, Strand,
};

pub mod file_explorer;
pub mod frequency_extractor;

#[derive(Strand)]
pub enum Container {
    #[strand(name = "quit")]
    Quit(Quit),

    #[strand(name = "cd", prefix = "@")]
    CD(ChangeDirectory),

    #[strand(name = "freq")]
    Freq(ReadFrequencies),
}

#[derive(Strand)]
pub struct Quit;
impl Command for Quit {
    type State = EmptyState;
    type Err = String;
    fn action(self, _: &mut Self::State) -> Result<(), Self::Err> {
        std::process::exit(0);
    }
}

#[derive(Strand)]
pub struct ChangeDirectory {
    #[strand(trail)]
    pub path: String,
}
impl Command for ChangeDirectory {
    type State = EmptyState;

    type Err = String;

    fn action(self, _: &mut Self::State) -> Result<(), Self::Err> {
        let current = env::current_dir().expect("Unable to get current directory");
        let canonical =
            CanonicalPath::new(current.join(self.path)).ok_or("Invalid path provided")?;

        env::set_current_dir(canonical.view()).map_err(|_| "Path provided does not exist")?;

        Ok(())
    }
}

#[derive(Strand)]
pub struct ReadFrequencies {
    #[strand(trail)]
    pub file_name: String,
}
impl Command for ReadFrequencies {
    type State = EmptyState;

    type Err = String;

    fn action(self, _: &mut Self::State) -> Result<(), Self::Err> {
        if !Path::new(&self.file_name).exists() {
            return Err("File does not exist".into());
        }

        let file = File::open(self.file_name).map_err(|_| "Unable to open file")?;

        let frequencies = frequency_extractor::Frequencies::new(file);

        print!("[");
        for (c, f) in frequencies.0.iter() {
            print!("{:?} = {}, ", c, f);
        }
        println!("]");
        Ok(())
    }
}

fn main() {
    boot();

    loop {
        let current = env::current_dir().expect("Unable to get current directory");
        let canonical = CanonicalPath::new(current).expect("Unable to get canonical path");
        env::set_current_dir(canonical.view()).expect("Unable to set current directory");

        let prompt = format!("{}> ", truncate_path(&canonical));

        run_console::<Container>(
            &mut EmptyState,
            Some(&prompt),
            ". ".into(),
            "!".into(),
            Matcher::Single(MatcherSingle::Item(' ')),
            Matcher::List(&[MatcherSingle::Item(';'), MatcherSingle::Item('\n')]),
        )
        .expect("Unable to parse input");
    }
}
