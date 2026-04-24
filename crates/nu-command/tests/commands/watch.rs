use nu_protocol::test_table;
use nu_test_support::{fs::Stub, prelude::*};

#[test]
fn watch_stream() -> Result {
    Playground::setup("streaming_watch_fs", |dirs, _| {
        let foo_txt = &*dirs.test().join("foo.txt");
        let bar_txt = &*dirs.test().join("bar.txt");

        let code = r#"
            [
                {|| touch foo.txt }
                {|| "meow" | save -f foo.txt }
                {|| mv foo.txt bar.txt }
                {|| rm bar.txt }
            ]
            | each {|fn| null; do $fn; {}}
            | zip { watch . --quiet }
            | each { into record }
        "#;

        let expected = test_table![
            ["operation",  "path", "new_path"];
            [   "Create", foo_txt,         ()],
            [    "Write", foo_txt,         ()],
            [   "Rename", foo_txt,    bar_txt],
            [   "Remove", bar_txt,         ()],
        ];

        test().cwd(dirs.test()).run(code).expect_value_eq(expected)
    })
}

#[test]
fn watch_stream_outside() -> Result {
    Playground::setup("streaming_watch_fs", |dirs, sandbox| {
        sandbox
            .mkdir("watched_dir")
            .with_files(&[Stub::EmptyFile("foo.txt")]);

        let mut foo_txt = dirs.test().to_owned();
        foo_txt.push("watched_dir");
        foo_txt.push("foo.txt");
        let foo_txt = &*foo_txt;

        let code = "
            [
                {|| mv ../foo.txt ./ }
                {|| mv foo.txt ../ }
            ]
            | each {|fn| null; do $fn; {}}
            | zip { watch . --quiet }
            | each { into record }
        ";

        let expected = test_table![
            ["operation",  "path", "new_path"];
            [   "Rename",      (),    foo_txt],
            [   "Rename", foo_txt,         ()],
        ];

        test()
            .cwd(dirs.test().join("watched_dir"))
            .run(code)
            .expect_value_eq(expected)
    })
}
