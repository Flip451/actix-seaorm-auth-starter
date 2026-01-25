#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    // uiディレクトリ内の "fail_*.rs" ファイルをコンパイルし、
    // エラーメッセージが期待通りかチェックする
    t.compile_fail("tests/ui/fail_*.rs");
}
