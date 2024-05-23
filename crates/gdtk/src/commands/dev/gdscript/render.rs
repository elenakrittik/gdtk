const SOURCE: &str = r#"
if true:
    return
else:
    print("hello")
"#;

pub fn run() -> anyhow::Result<()> {
    Ok(())
}
