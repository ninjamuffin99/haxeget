use super::common;
use crate::cache_directory::Cache;
use color_eyre::eyre::{eyre, Result};
use console::style;

/*
 * Gets the Haxe archive from github
 */
pub async fn download(cache: &Cache, version: &String) -> Result<String> {
    let client = reqwest::Client::new();
        
    let octocrab = octocrab::instance();
    let repos = octocrab.repos("HaxeFoundation", "haxe");

    let release = match version.as_str() {
        "latest" => 
        repos.releases().get_latest().await?,
        _ => repos.releases().get_by_tag(version).await?,
    };

    let asset_version = release.tag_name;

    println!("Downloading Haxe {}", style(&asset_version).yellow());

    let file_name =
        common::get_haxe_archive(&asset_version).expect("Unable to infer the file name of the tar file");

    // Now we can find the url that matches that file name
    let binary_url = &release.assets
    .iter()
    .find(|&asset| asset.name == file_name)
    .expect("There was not a valid asset for that version and target...")
    .browser_download_url.to_string();

    let path = format!("{}/bin/{file_name}", cache.location);
    common::download_file(&client, binary_url, &path)
        .await
        .unwrap();

    Ok(file_name)
}
