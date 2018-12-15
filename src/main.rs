extern crate structopt;
extern crate shiplift;
extern crate tui;
extern crate termion;

use structopt::StructOpt;
use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10)
                ].as_ref()
            )
            .split(size);
        Block::default()
             .title("Block")
             .borders(Borders::ALL)
             .render(&mut f, chunks[0]);
        Block::default()
             .title("Block 2")
             .borders(Borders::ALL)
             .render(&mut f, chunks[2]);
    })
}