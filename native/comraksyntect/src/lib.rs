use comrak::{ComrakOptions};
use rustler::{NifStruct};

mod atoms {
    rustler::atoms! {
        ok,
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

#[rustler::nif(name = "hello")]
fn add<'a>(a: i64, b: i64) -> (rustler::Atom, i64) {
    (atoms::ok(), a + b)
}

#[rustler::nif]
fn markdown_to_html(markdown: &str, options: ProxyComrakOptions) -> (rustler::Atom, String) {
    let input_options = ComrakOptions {
        extension: options.extension.to_comrak_extension_options(),
        parse: options.parse.to_comrak_parse_options(),
        render: options.render.to_comrak_render_options(),
    };

    let html = comrak::markdown_to_html(markdown, &input_options);

    (atoms::ok(), html)
}
