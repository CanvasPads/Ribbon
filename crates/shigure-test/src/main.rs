use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    overwrite_ast: bool,
}

#[derive(Clone, Serialize, Deserialize)]
struct TestFile {
    file: String,
    module: String,
    ast: String,
    stdout: String,
}

struct Test {
    file: String,
    module: String,
    ast: String,
}

impl<'a> From<TestFile> for Test {
    fn from(value: TestFile) -> Self {
        Self {
            file: value.file,
            ast: value.ast,
            module: value.module,
        }
    }
}

fn run_test(dir: &Path, test: Test, overwrite_ast: bool) {
    let program_file = dir.join(&test.file);
    let program_file_name = &program_file
        .to_str()
        .expect("Invalid file name")
        .to_string();
    let program_file_source =
        fs::read_to_string(program_file.clone()).expect("Cannot read source file");

    let ast_file = dir.join(&test.ast);

    let mut parser =
        shigure_parser::lang::parser::Parser::new(program_file_name, &program_file_source);
    let node = match parser.parse_all() {
        Ok(node) => node,
        Err(err) => {
            println!("Parse error: {:?}", err);
            return;
        }
    };

    let ast_json = serde_json::to_string(&node).expect("Failed to serialize ast");
    println!("parsing result:");
    println!("{}", ast_json);

    if overwrite_ast {
        let mut file = fs::File::create(ast_file.clone()).expect("Failed to create ast file");
        file.write_all(ast_json.as_bytes())
            .expect("Cannot write ast file");
    }

    let ast_file_source = fs::read_to_string(ast_file).expect("Cannot read ast file");
    if ast_json == ast_file_source {
        println!("AST test successed");
    } else {
        println!("AST test failed")
    }
}

fn main() {
    let args = Args::parse();

    let dir = Path::new(&args.path);
    let test_file_path = dir.join(".shigure-test");
    let test_file_str = fs::read_to_string(test_file_path).expect("Cannot read `.shigure-test`");
    let test_file: TestFile = serde_json::from_str(&test_file_str).expect("Invalid test file");
    run_test(dir, test_file.into(), args.overwrite_ast);
}
