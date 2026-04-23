use std::env;

mod codegen;
mod fns;
mod html;
mod infer_types;
mod scraper;
mod types;
mod vars;

fn help(pname: &str) {
    println!(
        r#"
Usage: {} [OPTIONS]

Options:    
  -h    Print this help message
  -f    Scrape studien.ost.ch for data
  -t    Generate rs types from data
  -s    Generate SQL
  -g    Generate HTML"#,
        pname
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && !args.iter().any(|a| a == "-h") {
        // TODO: split fetch and typegen
        if args.iter().any(|a| a == "-f") {
            scraper::scrape();
        }
        if args.iter().any(|a| a == "-t") {
            codegen::gen_types();
        }
        if args.iter().any(|a| a == "-s") {
            codegen::gen_sql();
        }
        if args.iter().any(|a| a == "-g") {
            html::gen_html();
        }
    } else {
        help(&args[0]);
    }
}
