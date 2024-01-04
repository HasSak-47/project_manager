use clap::Parser;

#[derive(Debug, Parser)]
enum CliTree{
    Print,
    Select,
    Create,
    MarkFeature,
    TUI,
}
