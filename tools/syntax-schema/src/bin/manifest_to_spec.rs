use std::path::PathBuf;

use clap::Parser;

use codegen::schema::Grammar;

#[derive(Parser, Debug)]
struct ProgramArgs {
    #[clap(long)]
    manifest_input: String,

    #[clap(long)]
    documentation_folder: String,
}

fn main() {
    let args = ProgramArgs::parse();

    println!(" => Loading Manifest");
    let grammar = Grammar::from_manifest(&PathBuf::from(args.manifest_input));

    println!(" => Generating Specification");
    grammar.generate_spec(&PathBuf::from(args.documentation_folder));
}
