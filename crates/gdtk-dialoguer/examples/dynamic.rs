use gdtk_dialoguer::{DynamicPrompt, QueryResult};

fn query(_: &DynamicPrompt) -> QueryResult<'static> {
    let items = vec![
        "foo", "bar", "foo", "bar", "foo", "bar", "foo", "bar", "foo", "bar", "foo", "bar", "foo",
        "bar", "baz",
    ];

    std::ops::ControlFlow::Continue(("abc", items))
}

fn main() -> gdtk_dialoguer::Result<()> {
    let answer = DynamicPrompt::builder().query(query).build().interact()?;

    dbg!(answer);

    Ok(())
}
