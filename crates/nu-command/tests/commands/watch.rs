use nu_protocol::test_table;
use nu_test_support::prelude::*;

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
