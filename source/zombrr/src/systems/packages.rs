use bevy::asset::{AssetServerSettings, FileAssetIo};
use bevy::prelude::*;
use bevy_loading::Progress;
use zombrr_core::ZombrrPackages;

pub fn load_packages(
    mut packages: ResMut<ZombrrPackages>,
    asset_settings: Res<AssetServerSettings>,
) -> Progress {
    let mut path = FileAssetIo::get_root_path();
    path.push(&asset_settings.asset_folder);
    path.push("packages");
    if let Err(err) = packages.load(path) {
        error!("Failed to load packages: {}", err);
    }
    true.into()
}
