use std::fs::{create_dir, write};

use anyhow::Result;

/// Create a new Brack project.
/// It will create a directory with the given name and the following structure:
/// - docs/
///    - main.[]
/// - Brack.toml
/// - .gitignore
pub fn new_project(document_name: &str) -> Result<()> {
    create_dir(document_name)?;
    create_dir(format!("{}/docs", document_name))?;
    write(format!("{}/docs/main.[]", document_name), "")?;
    create_settings_template(document_name)?;
    create_gitignore_template(document_name)?;
    Ok(())
}

fn create_settings_template(document_name: &str) -> Result<()> {
    write(
        format!("{}/Brack.toml", document_name),
        format!(
            r#"[document]
name = "{}"
version = "0.1.0"
backend = ""
extension = ""
authors = ["your name <your email>"]

[plugins]
"#,
            document_name
        ),
    )?;
    Ok(())
}

fn create_gitignore_template(document_name: &str) -> Result<()> {
    write(
        format!("{}/.gitignore", document_name),
        r#"plugins
out
"#,
    )?;
    Ok(())
}
