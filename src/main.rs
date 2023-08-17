use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use zip::read::ZipArchive;

#[derive(Serialize)] // Implement Serialize for the ErrorInfo struct
struct ErrorInfo {
    error: String,
}
#[derive(Deserialize)]
struct FabricModInfo {
    #[serde(default)]
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
    error: Option<String>,
}

#[derive(Deserialize)]
struct EntryPoints {
    #[serde(default)]
    client: Vec<String>,
    #[serde(default)]
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

#[derive(Serialize, Deserialize)]
struct Depends {
    #[serde(flatten)]
    other_depends: HashMap<String, String>,
}

#[derive(Deserialize)]
struct Recommends {
    modmenu: String,
}

#[derive(Deserialize)]
struct Contact {
    #[serde(default)]
    homepage: String,
    #[serde(default)]
    sources: String,
    issues: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mod_directory = "c:/Users/kevinv/AppData/Roaming/com.modrinth.theseus/profiles/Fabulously Optimized 5.2.3/mods/";
    let output_csv_path = "./output/"; // Specify the output CSV path

    let mut csv_file = File::create(&output_csv_path)?;

    let mut csv_content = String::new();
    csv_content.push_str("Schema Version,Mod ID,Version,Environment,Entry Client,Entry ModMenu,Custom Strategy,Custom URL,Depends Fabric,Depends Cloth-Config2,Recommends ModMenu,Name,Description,Icon,Authors,Contact Homepage,Contact Sources,Contact Issues\n");
    csv_file.write_all(csv_content.as_bytes())?;

    let error_dir = std::path::Path::new(&output_csv_path).join("error");
    fs::create_dir_all(&error_dir)?;

    for entry in fs::read_dir(mod_directory)? {
        let entry = entry?;
        let file_path = entry.path();
        if let Some(extension) = file_path.extension() {
            if extension == "jar" {
                let jar_file_name = file_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
                let jar_file = File::open(&file_path)?;
                let mut archive = ZipArchive::new(jar_file)?;

                for i in 0..archive.len() {
                    let mut file = archive.by_index(i)?;
                    let file_name = file.name();

                    if file_name == "fabric.mod.json" {
                        let mut content = String::new();
                        file.read_to_string(&mut content)?;

                        let mut mod_info: FabricModInfo = serde_json::from_str(&content)?;

                        match serde_json::from_str::<FabricModInfo>(&content) {
                            Ok(mod_info) => {
                                // Serialize the entire `depends` field into a JSON string
                                let depends_json = serde_json::to_string(&mod_info.depends)?;

                                // Append mod information to the CSV content
                                csv_content.push_str(&format!(
                                    "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                                    mod_info.schema_version,
                                    mod_info.id,
                                    mod_info.version,
                                    mod_info.environment,
                                    mod_info.entrypoints.client.join(","),
                                    mod_info.entrypoints.modmenu.join(","),
                                    mod_info
                                        .custom
                                        .as_ref()
                                        .map_or("", |c| &c.modupdater.strategy),
                                    mod_info.custom.as_ref().map_or("", |c| &c.modupdater.url),
                                    mod_info.recommends.as_ref().map_or("", |r| &r.modmenu),
                                    mod_info.name,
                                    mod_info.description,
                                    mod_info.icon,
                                    mod_info.authors.join(","),
                                    mod_info.contact.homepage,
                                    mod_info.contact.sources,
                                    mod_info.contact.issues,
                                    depends_json
                                ));
                            }
                            Err(err) => {
                                eprintln!(
                                    "Error deserializing JSON in archive '{}': {}",
                                    jar_file_name, err
                                );
                                mod_info.error = Some(err.to_string()); // Set the error field

                                // Create an ErrorInfo struct for serializing the error
                                let error_info = ErrorInfo {
                                    error: err.to_string(),
                                };

                                let error_json = serde_json::to_string(&error_info)?;
                                // Append error information to the CSV content
                                csv_content
                                    .push_str(&format!(",,,,,,,,,,,,,,,,,,{}\n", error_json));
                                // Write the problematic fabric.mod.json to the "error" directory
                                let error_file_path =
                                    error_dir.join(&format!("{}_fabric.mod.json", jar_file_name));
                                let error_file = File::create(&error_file_path)?;
                                serde_json::to_writer_pretty(&error_file, &error_info)?; // Write the JSON with the error field

                                println!(
                                    "Problematic fabric.mod.json written to: {:?}",
                                    error_file_path
                                );
                            }
                        }
                    }
                }
                // Write the current CSV content to the CSV file after processing each JAR file
                csv_file.write_all(csv_content.as_bytes())?;
            }
        }
    }

    // Write CSV content to the output file
    let mut csv_file = File::create(output_csv_path)?;
    csv_file.write_all(csv_content.as_bytes())?;

    println!("CSV file with mod information created: {}", output_csv_path);

    Ok(())
}
