#![feature(stmt_expr_attributes)]

mod cli;

use std::{
    collections::BTreeMap,
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Context;
use cli::Args;
use handlebars::Handlebars;
use toml::Value;

fn main() -> anyhow::Result<()> {
    let args = Args::new()?;

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("template", &args.template)
        .expect("Could not render template");

    let mut data = String::new();
    if let Some(path) = &args.data {
        File::open(path)?.read_to_string(&mut data)?;
    }
    let data: BTreeMap<String, Value> = toml::from_str(&data)?;

    if let Ok(rendered) = handlebars.render("template", &data) {
        match args.output {
            cli::Output::Stdout => write_to_dest(&mut io::stdout(), &rendered),
            cli::Output::Automatic => write_to_dest(
                &mut File::create(generate_automatic_output_path(
                    &mut handlebars,
                    &data,
                    &args.template,
                )?)?,
                &rendered,
            ),
            cli::Output::Specific(path) => write_to_dest(&mut File::create(&path)?, &rendered),
        }
    }

    Ok(())
}

/// Generate the automatic output path. Replaces `template` in the template file
/// name with the value of the attribute `filename`. Strips the final file
/// extension.
fn generate_automatic_output_path(
    handlebars: &mut Handlebars,
    data: &BTreeMap<String, Value>,
    tpl_path: &Path,
) -> anyhow::Result<PathBuf> {
    let name = tpl_path.file_stem().unwrap().to_string_lossy();
    let name = if let Some(filename) = data.get("filename") {
        if filename.is_table() {
            handlebars.register_template_string("filename", name)?;
            handlebars
                .render("filename", filename.as_table().unwrap())
                .context("Could not render file name")?
        } else {
            name.replace("template", filename.as_str().unwrap())
        }
    } else {
        name.to_string()
    };
    Ok(tpl_path
        .parent()
        .map(PathBuf::from)
        .unwrap_or_default()
        .join(name))
}

/// Write the generated text to the given destination.
fn write_to_dest<T: Write>(dest: &mut T, content: &str) {
    dest.write_all(content.as_bytes())
        .expect("Could not write rendered template")
}
