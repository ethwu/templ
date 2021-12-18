use std::path::{Path, PathBuf};

use anyhow::{bail, Context};
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

/// Where to output the interpolated template.
#[derive(Debug)]
pub enum Output {
    /// Output directly to stdout.
    Stdout,
    /// Output to an automatically-generated file path.
    Automatic,
    /// Output to an explicit path.
    Specific(PathBuf),
}

/// Container for CLI arguments.
#[derive(Debug)]
pub struct Args {
    /// Location of the template file.
    pub template: PathBuf,
    /// Where to get data to interpolate in the template.
    pub data: Option<PathBuf>,
    /// Where to output the
    pub output: Output,
}

impl Args {
    pub fn new() -> Result<Self, anyhow::Error> {
        let app = App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .help_message("Display help. Use `--help` for more information.")
            .version_message("Display version.")
            .max_term_width(80)
            .arg(
                Arg::with_name("template")
                    .value_name("TEMPLATE")
                    .help("Template file to render.")
                    .long_help(
                        "Path to a template file. The template _must_ have the extension `.hbs`.",
                    )
                    .required(true)
                    .empty_values(false),
            )
            .arg(
                Arg::with_name("data")
                    .short("d")
                    .long("data")
                    .value_name("path")
                    .help("Data to interpolate.")
                    .long_help(
"Path to a TOML file containing the data to interpolate. If not specified, either a file with the same name as the template with the `.hbs` extension replaced with `.toml` (e.g., `template.txt.hbs` and `template.txt.toml`) or `template.toml` in the current working directory will be used instead.",
                    )
                    .empty_values(false),
            )
            .arg(
                Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .value_name("path")
                    .takes_value(true)
                    .help("Output file path.")
                    .long_help(
r#"Select a specific output file path. If left unspecified, the output file path will be generated from the template file name:
  - If a table `filename` exists in the data file, the name of the template file is used as a Handlebars template, and keys from the `filename` table are used to render the file name.
  - If the key `filename` exists in the data file, any instances of the string `template` in the template file name are replaced with the value of `filename`.
  - If the key `filename` is left undefined, no such substitution happens.
The `.hbs` file extension is stripped from the file name.

For example, with the data file
    # template.md.toml
    name = "Alice"
    age = 22
    profession = "Cryptographer"

    filename = "profile"
and the template file
    <!-- template.md.hbs -->
    # {{name}} #
    {{name}} is a {{age}} year old {{profession}}.
the automatically generated file name would be `profile.md`. If the `filename` key were left out, the name would be `template.md`. If the template file were named `input.md.hbs` instead of `template.md.hbs`, the output file would be `input.md`.

With the data file
    # template.toml
    secret = 42
    [filename]
    secret = "noodle"
    flavor = "spicy"
and the template file
    # {{secret}}-reveal ({{flavor}}).py.hbs
    assert "{{secret}}" == 42
the automatically generated file name would be `noodle-reveal (spicy).py`.
"#,
                    ),
            )
            .arg(
                Arg::with_name("stdout")
                    .short("s")
                    .long("stdout")
                    .help("Emit rendered template to stdout.")
                    .long_help("Emit the rendered template to stdout instead of writing to a file.")
                    .conflicts_with("output"),
            );

        let matches = app.get_matches();

        let template = PathBuf::from(matches.value_of("template").unwrap())
            .canonicalize()
            .context("Template does not exist")?;
        if template.extension().is_none() || template.extension().unwrap() != "hbs" {
            bail!("Template is not a .hbs file");
        }

        let data = get_data_path(&template, matches.value_of("data"));

        Ok(Self {
            template,
            data,
            output: if matches.is_present("stdout") {
                Output::Stdout
            } else if let Some(output) = matches.value_of("output") {
                Output::Specific(PathBuf::from(output))
            } else {
                Output::Automatic
            },
        })
    }
}

/// Get the path to the data file. If it is specifed, use that. If it is not, or
/// if the specified file does not exist, look for a file with the same name as
/// the template but with a `.toml` extension.
fn get_data_path(tpl_path: &Path, data_path: Option<&str>) -> Option<PathBuf> {
    // First check if a path was specified explicitly, and whether that is valid.
    if let Some(data_path) = data_path {
        let data_path = PathBuf::from(data_path).canonicalize().ok();
        if data_path.is_some() {
            return data_path;
        }
    }

    // Next see if a toml file exists with the same filename as the template.
    let data_path = tpl_path.with_extension("toml");
    let data_path = data_path.canonicalize().ok();
    if data_path.is_some() {
        return data_path;
    }

    // Finally, look for a `template.toml` file.
    tpl_path
        .parent()
        .map(PathBuf::from)
        .unwrap_or_default()
        .join("template.toml")
        .canonicalize()
        .ok()
}
