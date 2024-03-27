use hecto_editor::editor::Editor;

fn main() -> anyhow::Result<()> {
    let mut editor = Editor::new()?;
    while editor.tick()? {}
    Ok(())
}
