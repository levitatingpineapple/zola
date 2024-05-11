use cached::proc_macro::cached;
use std::sync::Arc;
use tree_sitter_highlight::{Highlight, HighlightConfiguration, Highlighter, HtmlRenderer};

const NAMES: [&str; 18] = [
    "attribute",
    "comment",
    "constant",
    "constructor",
    "escape",
    "function",
    "include",
    "keyword",
    "label",
    "namespace",
    "number",
    "operator",
    "property",
    "punctuation",
    "repeat",
    "string",
    "type",
    "variable",
];

#[cached(sync_writes = true)]
fn swift_config() -> Arc<HighlightConfiguration> {
    println!("Treesitter: Loading swift config");
    let mut config = HighlightConfiguration::new(
        tree_sitter_swift::language(),
        "swift",
        tree_sitter_swift::HIGHLIGHTS_QUERY,
        tree_sitter_swift::LOCALS_QUERY,
        tree_sitter_swift::INJECTIONS_QUERY,
    )
    .unwrap();
    config.configure(&NAMES);
    Arc::new(config)
}

#[cached]
fn classes() -> Vec<String> {
    NAMES.map(|n| format!(r#"class="ts-{}""#, n.replace(".", "-"))).to_vec()
}

pub fn highlight_swift(source: &str) -> String {
    let config = swift_config();
    let mut highlighter = Highlighter::new();
    let events = highlighter.highlight(&config, source.as_bytes(), None, |_| None).unwrap();
    let mut renderer = HtmlRenderer::new();
    let classes = classes();
    match renderer.render(events, source.as_bytes(), &|h: Highlight| match classes.get(h.0) {
        Some(class) => class.as_bytes(),
        None => "".as_bytes(),
    }) {
        Ok(_) => renderer.lines().collect(),
        Err(err) => {
            eprintln!("Treesitter error: {}", err);
            String::try_from(source).expect("Valid")
        }
    }
}
