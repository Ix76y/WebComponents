use clap::Parser;
use std::fs::File;
use std::io::{Write, Error};
use std::path::PathBuf;

mod cli;
use cli::Cli;


fn create_class_list(classes: Vec<String>) -> String {
    let mut list = String::new();
    for class in classes {
        let js = format!("        wrapper.classList.add({});\n", class);
        list += &js;
    }
    return list;
}

fn generate_component(cli: Cli) -> Result<(), Error> {
    let mut path = PathBuf::new();
    path.push(cli.folder);
    path.push(&cli.tag_name);
    path.set_extension("js");
    println!("Creating file: {:?}", path);

    let mut output = File::create(path)?;

    let class_list = create_class_list(cli.classes);

    let js = format!(
"class extends HTMLElement {{
    constructor() {{
        super();
        const shadow = this.attachShadow({{mode: 'open'}});
        const wrapper = document.createElement('{}');

{}

        shadow.appendChild(wrapper);
    }}
}}", 
    cli.wrapper,
    class_list);

    write!(output, "customElements.define('{}', {});", cli.tag_name, js);

    println!("File successfully generated.");
    Ok(())
}

/// ComponentGenerator <tag-name> --output <output_folder> --wrapper div --classes val1 val2 val3
fn main() {
    println!("WebComponent Generator");
    let args = Cli::parse();
    println!("Arguments: \n{}", args);
    _ = generate_component(args);
}

