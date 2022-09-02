mod config;
mod interaction;
mod languages;
mod lsp;
mod print;
mod scan;
mod test;

use clap::{Parser, Subcommand};
use scan::{run_with_config, run_with_pattern, ScanArg};
use std::io::Result;
use test::{run_test_rule, TestArg};

use languages::SupportLang;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
/**
 * TODO: add some description for ast-grep: sg
 * Example:
 * sg -p ""
 */
pub struct Args {
  #[clap(subcommand)]
  command: Option<Commands>,

  /// AST pattern to match
  #[clap(short, long, requires = "lang")]
  pattern: Option<String>,

  /// String to replace the matched AST node
  #[clap(short, long)]
  rewrite: Option<String>,

  /// Print query pattern's tree-sitter AST
  #[clap(long, parse(from_flag))]
  debug_query: bool,

  /// The language of the pattern query
  #[clap(short, long)]
  lang: Option<SupportLang>,

  #[clap(short, long, parse(from_flag))]
  interactive: bool,

  /// The path whose descendent files are to be explored.
  #[clap(value_parser, default_value = ".")]
  path: String,

  /// Include hidden files in search
  #[clap(short, long, parse(from_flag))]
  hidden: bool,
}

#[derive(Subcommand)]
enum Commands {
  /// Scan and rewrite code
  Scan(ScanArg),
  /// test ast-grep rule
  Test(TestArg),
  /// starts language server
  Lsp,
  /// generate rule docs for current configuration
  Docs,
}

fn main() -> Result<()> {
  let mut args = Args::parse();
  let command = args.command.take();
  if command.is_none() {
    return run_with_pattern(args);
  }
  match command.unwrap() {
    Commands::Scan(arg) => run_with_config(arg),
    Commands::Test(arg) => run_test_rule(arg),
    Commands::Lsp => lsp::run_language_server(),
    Commands::Docs => todo!("todo, generate rule docs based on current config"),
  }
}
