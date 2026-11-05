//! Epic: the launcher's install manifests on Windows
//! (`%ProgramData%\Epic\EpicGamesLauncher\Data\Manifests\*.item`) and, on any
//! system, legendary's `installed.json`, which Heroic keeps too.
//!
//! A game's key is `epic:<CatalogItemId>`, Epic's catalog id, the id droidtop
//! gives the same game (Droidtop/tracker#871). Its `AppName`, the key this
//! scanner gave before, is kept as an alias. When the catalog id cannot be
//! found, the key is `epic:<AppName>`, as before.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

use super::{env_dir, home, pc_game, Found};
use crate::Options;

/// The key from a catalog id and an AppName, with the AppName's key as an
/// alias when the two differ.
fn keyed(mut found: Found, app: &str, catalog: Option<&str>) -> Found {
    if let Some(catalog) = catalog.filter(|c| !c.is_empty() && *c != app) {
        found.game.key = format!("epic:{catalog}");
        found.aliases.push(format!("epic:{app}"));
    }
    found
}

/// One launcher manifest (`.item`), or None for a DLC, an add-on or an
/// install that did not finish.
fn from_item(item: &Value) -> Option<Found> {
    let app = item["AppName"].as_str().unwrap_or_default();
    let main = item["MainGameAppName"].as_str().unwrap_or(app);
    if app.is_empty() || app != main || item["bIsIncompleteInstall"].as_bool() == Some(true) {
        return None;
    }
    let title = item["DisplayName"].as_str().unwrap_or(app).to_string();
    let base = item["InstallLocation"].as_str().map(PathBuf::from).filter(|p| p.is_dir());
    let size = item["InstallSize"].as_u64().unwrap_or(0);
    let version = item["AppVersionString"].as_str().map(str::to_string);
    let found = pc_game(format!("epic:{app}"), title, base, "epic", size, version);
    Some(keyed(found, app, item["CatalogItemId"].as_str()))
}

fn launcher_manifests() -> Vec<Found> {
    let Some(data) = env_dir("ProgramData") else { return Vec::new() };
    let dir = data.join(r"Epic\EpicGamesLauncher\Data\Manifests");
    let Ok(read) = fs::read_dir(dir) else { return Vec::new() };
    let mut out = Vec::new();
    for entry in read.flatten() {
        if entry.path().extension().is_none_or(|e| e != "item") {
            continue;
        }
        let Ok(text) = fs::read_to_string(entry.path()) else { continue };
        let Ok(item) = serde_json::from_str::<Value>(&text) else { continue };
        out.extend(from_item(&item));
    }
    out
}

/// legendary's own folder, and Heroic's copy of it on each system: each
/// holds `installed.json` and a `metadata` folder.
fn legendary_dirs() -> Vec<PathBuf> {
    let h = home();
    let mut out = vec![
        h.join(".config/legendary"),
        h.join(".config/heroic/legendaryConfig/legendary"),
        h.join(".var/app/com.heroicgameslauncher.hgl/config/heroic/legendaryConfig/legendary"),
        h.join("Library/Application Support/heroic/legendaryConfig/legendary"),
    ];
    if let Some(appdata) = env_dir("APPDATA") {
        out.push(appdata.join(r"heroic\legendaryConfig\legendary"));
    }
    out
}

/// A game's catalog id from legendary's `metadata/<AppName>.json`, which
/// keeps Epic's catalog entry with the catalog id as `metadata.id`.
fn legendary_catalog_id(dir: &Path, app: &str) -> Option<String> {
    let text = fs::read_to_string(dir.join("metadata").join(format!("{app}.json"))).ok()?;
    let meta = serde_json::from_str::<Value>(&text).ok()?;
    meta["metadata"]["id"].as_str().map(str::to_string)
}

/// The games in one legendary folder's `installed.json`.
fn legendary_in(dir: &Path) -> Vec<Found> {
    let mut out = Vec::new();
    let Ok(text) = fs::read_to_string(dir.join("installed.json")) else { return out };
    let Ok(Value::Object(games)) = serde_json::from_str::<Value>(&text) else { return out };
    for (app, g) in games {
        if g["is_dlc"].as_bool() == Some(true) {
            continue;
        }
        let title = g["title"].as_str().unwrap_or(&app).to_string();
        let base = g["install_path"].as_str().map(PathBuf::from).filter(|p| p.is_dir());
        let size = g["install_size"].as_u64().unwrap_or(0);
        let version = g["version"].as_str().map(str::to_string);
        let found = pc_game(format!("epic:{app}"), title, base, "heroic", size, version);
        out.push(keyed(found, &app, legendary_catalog_id(dir, &app).as_deref()));
    }
    out
}

fn legendary() -> Vec<Found> {
    legendary_dirs().iter().flat_map(|dir| legendary_in(dir)).collect()
}

pub fn scan(_settings: &Options) -> Result<Vec<Found>, String> {
    let mut out = launcher_manifests();
    out.extend(legendary());
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn a_launcher_manifest_is_keyed_by_its_catalog_id() {
        let item = json!({
            "AppName": "Fortnite",
            "MainGameAppName": "Fortnite",
            "CatalogItemId": "4fe75bbc5a674f4f9b356b5c90567da5",
            "DisplayName": "Fortnite",
        });
        let found = from_item(&item).unwrap();
        assert_eq!(found.game.key, "epic:4fe75bbc5a674f4f9b356b5c90567da5");
        assert_eq!(found.aliases, ["epic:Fortnite"]);
        assert_eq!(found.game.title, "Fortnite");
    }

    #[test]
    fn a_manifest_without_a_catalog_id_keeps_its_app_name() {
        let item = json!({"AppName": "Sugar", "DisplayName": "Sugar"});
        let found = from_item(&item).unwrap();
        assert_eq!(found.game.key, "epic:Sugar");
        assert!(found.aliases.is_empty());
    }

    #[test]
    fn add_ons_and_unfinished_installs_are_not_games() {
        let addon = json!({"AppName": "Pack", "MainGameAppName": "Sugar", "CatalogItemId": "c1"});
        let partial = json!({"AppName": "Sugar", "CatalogItemId": "c2", "bIsIncompleteInstall": true});
        assert!(from_item(&addon).is_none());
        assert!(from_item(&partial).is_none());
    }

    #[test]
    fn legendary_takes_the_catalog_id_from_its_metadata() {
        let dir = std::env::temp_dir().join(format!("dta-legendary-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("metadata")).unwrap();
        let installed = json!({
            "Sugar": {"title": "Sugar", "version": "1.2"},
            "Salt": {"title": "Salt"},
            "SugarDlc": {"title": "Sugar DLC", "is_dlc": true},
        });
        fs::write(dir.join("installed.json"), installed.to_string()).unwrap();
        let meta = json!({"app_name": "Sugar", "metadata": {"id": "a1b2c3", "namespace": "ns"}});
        fs::write(dir.join("metadata").join("Sugar.json"), meta.to_string()).unwrap();

        let mut found = legendary_in(&dir);
        found.sort_by(|a, b| a.game.title.cmp(&b.game.title));
        let keys: Vec<_> = found.iter().map(|f| (f.game.key.as_str(), f.aliases.clone())).collect();
        assert_eq!(keys, [("epic:Salt", vec![]), ("epic:a1b2c3", vec!["epic:Sugar".to_string()])]);
        assert_eq!(found[1].game.install.launcher.as_deref(), Some("heroic"));
        fs::remove_dir_all(&dir).unwrap();
    }
}
