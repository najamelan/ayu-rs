use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let crates_io = include_str!("../../css/crates.io.css");
    let crates_io_docs = include_str!("../../css/docs.crates.io.css");
    let docs_rs = include_str!("../../css/docs.rs.css");
    let playground = include_str!("../../css/playground.css");
    let rustdoc = include_str!("../../css/rustdoc.css");

    let output = "@-moz-document domain(\"docs.rs\") {\n".to_owned()
                 + docs_rs +
                 "\n}\n\n\
                 @-moz-document domain(\"play.rust-lang.org\"), domain(\"play.integer32.com\") {\n"
                 + playground +
                 "\n}\n\n\
                 @-moz-document domain(\"doc.rust-lang.org\"), domain(\"docs.rs\") {\n"
                 + rustdoc +
                 "\n}\n\n\
                 @-moz-document domain(\"crates.io\") {\n"
                 + crates_io +
                 "\n}\n\n\
                 @-moz-document domain(\"doc.crates.io\") {\n"
                 + crates_io_docs +
                 "\n}";

    let file = File::create("ayu-rs_release.css").expect("file");
    let mut writer = BufWriter::new(file);
    writer.write_all(output.as_bytes()).expect("written bytes");
}
