use std::{fs, io};

use crate::utils::paths::get_app_chains_home;
use crate::utils::toml::regenerate_app_config;

pub fn list() {
    match get_apps_list() {
        Ok(apps) => {
            log::info!("App Chain: {:?}", apps);
        }
        Err(err) => {
            panic!("Failed to list: {}", err);
        }
    }
}

/// Assumes that all the app configs are saved at "~/.madara/app-chains/{app}/{app}-config.toml"
/// But return app names after validating the {app}-config.toml
pub fn get_apps_list() -> Result<Vec<String>, io::Error> {
    let app_configs = get_app_chains_home()?;
    let app_names: Vec<String> = match fs::read_dir(app_configs) {
        Ok(entries) => entries
            .filter_map(|entry| {
                entry.ok().and_then(|entry| {
                    entry.file_name().into_string().ok().and_then(|file_name| match regenerate_app_config(&file_name) {
                        Ok(response) => {
                            if response.1 {
                                return Some(file_name);
                            }
                            log::warn!("Invalid toml: {}", file_name);
                            None
                        }
                        Err(_) => {
                            log::warn!("Failed to parse toml: {}", file_name);
                            None
                        }
                    })
                })
            })
            .collect(),
        Err(err) => {
            log::warn!("Error reading directory: {:?}", err);
            vec![] // Return an empty Vec in case of an error
        }
    };

    Ok(app_names)
}
