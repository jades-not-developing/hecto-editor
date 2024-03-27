use hecto_editor::{Editor, TryDefault};

fn main() -> anyhow::Result<()> {
    let mut editor = Editor::try_default()?;
    while editor.tick()? {}
    Ok(())
}
