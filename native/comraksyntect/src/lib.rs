use comrak::{ComrakOptions, Arena, nodes::{Ast, AstNode, NodeValue}, parse_document, format_html};
use syntect::{
    parsing::{SyntaxReference, SyntaxSet},
    html::{ClassedHTMLGenerator, ClassStyle},
    util::LinesWithEndings,
};
use rustler::{NifStruct};

mod atoms {
    rustler::atoms! {
        ok,
        err,
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

#[derive(Debug, NifStruct)]
#[module = "ComrakSyntect.Native.ExtensionOptions"]
struct ProxyComrakExtensionOptions {
    strikethrough: bool,
    tagfilter: bool,
    table: bool,
    autolink: bool,
    tasklist: bool,
    superscript: bool,
    header_ids: Option<String>,
    footnotes: bool,
    description_lists: bool,
}

impl ProxyComrakExtensionOptions {
    fn to_comrak_extension_options(self) -> comrak::ComrakExtensionOptions {
        comrak::ComrakExtensionOptions {
            strikethrough: self.strikethrough,
            tagfilter: self.tagfilter,
            table: self.table,
            autolink: self.autolink,
            tasklist: self.tasklist,
            superscript: self.superscript,
            header_ids: self.header_ids,
            footnotes: self.footnotes,
            description_lists: self.description_lists,
        }
    }
}

#[derive(Debug, NifStruct)]
#[module = "ComrakSyntect.Native.ParseOptions"]
struct ProxyComrakParseOptions {
    smart: bool,
    default_info_string: Option<String>,
}

impl ProxyComrakParseOptions {
    fn to_comrak_parse_options(self) -> comrak::ComrakParseOptions {
        comrak::ComrakParseOptions {
            smart: self.smart,
            default_info_string: self.default_info_string,
        }
    }
}

#[derive(Debug, NifStruct)]
#[module = "ComrakSyntect.Native.RenderOptions"]
struct ProxyComrakRenderOptions {
    hardbreaks: bool,
    github_pre_lang: bool,
    width: usize,
    unsafe_: bool,
    escape: bool,
}

impl ProxyComrakRenderOptions {
    fn to_comrak_render_options(self) -> comrak::ComrakRenderOptions {
        comrak::ComrakRenderOptions {
            hardbreaks: self.hardbreaks,
            github_pre_lang: self.github_pre_lang,
            width: self.width,
            unsafe_: self.unsafe_,
            escape: self.escape,
        }
    }
}

#[derive(Debug, NifStruct)]
#[module = "ComrakSyntect.Native.Options"]
struct ProxyComrakOptions {
    extension: ProxyComrakExtensionOptions,
    parse: ProxyComrakParseOptions,
    render: ProxyComrakRenderOptions,
}

rustler::init!(
    "Elixir.ComrakSyntect.Native",
    [
    add,
    markdown_to_html,
    ]
);

lazy_static::lazy_static! {
    static ref PS: SyntaxSet = SyntaxSet::load_defaults_newlines();
}

#[rustler::nif(name = "hello")]
fn add<'a>(a: i64, b: i64) -> (rustler::Atom, i64) {
    (atoms::ok(), a + b)
}

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
    where F : Fn(&'a AstNode<'a>) {
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}

#[rustler::nif]
fn markdown_to_html(markdown: &str, options: ProxyComrakOptions) -> (rustler::Atom, String) {
    let md_opts = ComrakOptions {
        extension: options.extension.to_comrak_extension_options(),
        parse: options.parse.to_comrak_parse_options(),
        render: options.render.to_comrak_render_options(),
    };

    let arena = Arena::new();

    let root = parse_document(
        &arena,
        markdown,
        &md_opts
    );

    iter_nodes(root, &|node| {
        let replace_node = match &mut node.data.borrow_mut().value {
            &mut NodeValue::CodeBlock(ref mut code) => {
                let highlighted = highlight_html(&code.literal, &code.info);

                Some(NodeValue::HtmlInline(highlighted))
            }
            _ => None
        };

        if let Some(n) = replace_node {
            node.data.replace(Ast::new(n));
        }
    });


    let mut html = Vec::new();
    let result = format_html(root, &md_opts, &mut html);

    let html = match result {
        Ok(_) => String::from_utf8(html).unwrap(),
        Err(err) => { return (atoms::err(), err.to_string()); },
    };

    (atoms::ok(), html)
}

fn get_syntax(name: &[u8]) -> Option<&'static SyntaxReference> {
    let name_str = std::str::from_utf8(name).ok()?;

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
