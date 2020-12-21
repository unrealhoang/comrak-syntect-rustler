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

#[derive(Debug, NifStruct)]
#[module = "ComrakSyntect.Native.Options"]
struct ProxyComrakOptions {
    extension: ProxyComrakExtensionOptions,
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
        extension: comrak::ComrakExtensionOptions {
            strikethrough: options.extension.strikethrough,
            tagfilter: options.extension.tagfilter,
            table: options.extension.table,
            autolink: options.extension.autolink,
            tasklist: options.extension.tasklist,
            superscript: options.extension.superscript,
            header_ids: options.extension.header_ids,
            footnotes: options.extension.footnotes,
            description_lists: options.extension.description_lists,
        },
        .. Default::default()
    };

    let html = comrak::markdown_to_html(markdown, &input_options);

    (atoms::ok(), html)
}
