use syntect::{
    parsing::{SyntaxReference, SyntaxSet},
    html::{ClassedHTMLGenerator, ClassStyle},
    util::LinesWithEndings,
};

lazy_static::lazy_static! {
    static ref PS: SyntaxSet = SyntaxSet::load_defaults_newlines();
}

fn get_syntax(name: &[u8]) -> Option<&'static SyntaxReference> {
    let name_str = std::str::from_utf8(name).ok()?;
    dbg!(name_str);

    PS.find_syntax_by_token(name_str)
}

// Returns highlighted html to a code block
fn highlight_html(code: &[u8], lang: &[u8]) -> Vec<u8> {
    let syntax = get_syntax(lang);
    let mut result = Vec::new();
    let code_str = std::str::from_utf8(code);
    match (syntax, code_str) {
        (Some(sr), Ok(code_str)) => {
            let mut html_generator = ClassedHTMLGenerator::new_with_class_style(sr, &PS, ClassStyle::SpacedPrefixed { prefix: "st-" });
            for line in LinesWithEndings::from(code_str) {
                html_generator.parse_html_for_line_which_includes_newline(&line);
            }
            let html = html_generator.finalize();
            result.extend(html.bytes());
        }
        _ => {
            result.extend(b"<pre>".into_iter());
            result.extend(code.iter());
            result.extend(b"</pre>".into_iter());
        }
    }

    result
}

fn main() {
    let code_str = r#"
        fn hello(args: Vec<u8>) {
            println!("World: {}", args);
        }
    "#;

    let s = highlight_html(
        &code_str.bytes().collect::<Vec<_>>()[..],
        &"Rust".bytes().collect::<Vec<_>>()[..]
    );
    println!("Result: {}", String::from_utf8(s).unwrap());
}
