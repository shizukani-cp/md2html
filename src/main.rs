use pulldown_cmark::{Parser, html};
use std::env;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input.md>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = input_file.replace(".md", ".html");

    let markdown_content = fs::read_to_string(input_file)?;

    let parser = Parser::new(&markdown_content);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let template_path = "template.html";
    let template_content = if Path::new(template_path).exists() {
        fs::read_to_string(template_path)?
    } else {

        String::from("<html>\n<head></head>\n<body>{}</body>\n</html>")
    };

    let final_html = template_content.replace("{}", &html_output);

    fs::write(&output_file, final_html)?;

    Ok(())
}
