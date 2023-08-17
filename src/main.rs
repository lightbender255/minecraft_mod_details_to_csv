// "Certainly! Here's the modified code with all the fields from the example `fabric.mod.json` added to the `FabricModInfo` struct. You can customize the `FabricModInfo` struct
// to include any additional fields you might want to extract from your `fabric.mod.json` files:", said ChatGPT 3

use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use serde_json;

#[derive(Deserialize)]
struct FabricModInfo {
    schema_version: u32,
    id: String,
    version: String,
    environment: String,
    entrypoints: EntryPoints,
    custom: Option<Custom>,
    depends: Depends,
    recommends: Option<Recommends>,
    name: String,
    description: String,
    icon: String,
    authors: Vec<String>,
    contact: Contact,
}

#[derive(Deserialize)]
struct EntryPoints {
    client: Vec<String>,
    modmenu: Vec<String>,
}

#[derive(Deserialize)]
struct Custom {
    modupdater: ModUpdater,
}

#[derive(Deserialize)]
struct ModUpdater {
    strategy: String,
    url: String,
}

#[derive(Deserialize)]
struct Depends {
    fabric: String,
    #[serde(rename = "cloth-config2")]
    cloth_config: String,
}

#[derive(Deserialize)]
struct Recommends {
    modmenu: String,
}

#[derive(Deserialize)]
struct Contact {
    homepage: String,
    sources: String,
    issues: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    // load_config();
    let mod_directory = "c";
    let output_csv_path: &str = "./";
    let mut csv_content = String::new();
    csv_content.push_str("Schema Version,Mod ID,Version,Environment,Entry Client,Entry ModMenu,Custom Strategy,Custom URL,Depends Fabric,Depends Cloth-Config2,Recommends ModMenu,Name,Description,Icon,Authors,Contact Homepage,Contact Sources,Contact Issues\n");

    for entry in fs::read_dir(mod_directory)? {
        let entry = entry?;
        let file_path = entry.path();
        if let Some(extension) = file_path.extension() {
            if extension == "json" {
                let mut file = File::open(&file_path)?;
                let mut content = String::new();
                file.read_to_string(&mut content)?;

                let mod_info: FabricModInfo = serde_json::from_str(&content)?;

                // Append mod information to the CSV content
                csv_content.push_str(&format!(
                    "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                    mod_info.schema_version,
                    mod_info.id,
                    mod_info.version,
                    mod_info.environment,
                    mod_info.entrypoints.client.join(","),
                    mod_info.entrypoints.modmenu.join(","),
                    mod_info.custom.as_ref().map_or("", |c| &c.modupdater.strategy),
                    mod_info.custom.as_ref().map_or("", |c| &c.modupdater.url),
                    mod_info.depends.fabric,
                    mod_info.depends.cloth_config,
                    mod_info.recommends.as_ref().map_or("", |r| &r.modmenu),
                    mod_info.name,
                    mod_info.description,
                    mod_info.icon,
                    mod_info.authors.join(","),
                    mod_info.contact.homepage,
                    mod_info.contact.sources,
                    mod_info.contact.issues
                ));
            }
        }
    }

    // Write CSV content to the output file
    let mut csv_file = File::create(output_csv_path)?;
    csv_file.write_all(csv_content.as_bytes())?;

    println!("CSV file with mod information created: {}", output_csv_path);

    Ok(())
}

// This code will now extract all known fields from the `fabric.mod.json`
// files and write the information to a CSV file. Make sure to adjust the
// Config.toml