use bevy::prelude::*;

/// Loads assets based on the path in the given game state. Will call the done callback once everything is finished loading.
///
/// Failed handles will also be sent to the done callback, but their LoadState will indicate if they succeeded or not
/// The order of the loaded asset arguments will match the order of the passed in requested assets.
///
/// Usage: [`load_assets<AssetType, AnyMarkerType>`]
///
/// The marker type is to differentiate this loading call from other loading calls, even if it's loading the same asset type.
/// You can just make a throw-away zero-sized struct for this, just make sure it's not being used by any other load_assets call in the same state.
pub fn load_assets<T: Asset, const N: usize>(
    app: &mut App,
    paths: [&'static str; N],
    done: impl Fn(&mut World, [Handle<T>; N]) + Send + Sync + 'static,
) {
    let prepare_assets = move |world: &mut World| {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let handles = paths.map(|x| asset_server.load(x));
        done(world, handles);
    };

    app.add_systems(Startup, prepare_assets.in_set(LoadAssetsSet));
}

/// Loads assets based on the path in the given game state. Will call the done callback once everything is finished loading.
///
/// Failed handles will also be sent to the done callback, but their LoadState will indicate if they succeeded or not
/// The order of the loaded asset arguments will match the order of the passed in requested assets.
///
/// Usage: [`load_assets<AssetType, AnyMarkerType>`]
///
/// The marker type is to differentiate this loading call from other loading calls, even if it's loading the same asset type.
/// You can just make a throw-away zero-sized struct for this, just make sure it's not being used by any other load_assets call in the same state.
pub fn load_atlas<const N_FRAMES: u32, const TEXTURE_DIMS: u32>(
    app: &mut App,
    path: &'static str,
    done: impl Fn(&mut World, (Handle<Image>, Handle<TextureAtlasLayout>)) + Send + Sync + 'static,
) {
    let prepare_assets = move |world: &mut World| {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let handle = asset_server.load::<Image>(path);
        let mut layouts = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        let layout =
            TextureAtlasLayout::from_grid(UVec2::splat(TEXTURE_DIMS), N_FRAMES, 1, None, None);
        let layout_handle = layouts.add(layout);

        done(world, (handle, layout_handle));
    };

    app.add_systems(Startup, prepare_assets.in_set(LoadAssetsSet));
}

#[derive(Hash, SystemSet, Debug, PartialEq, Eq, Clone, Copy)]
pub struct LoadAssetsSet;

pub(super) fn register(app: &mut App) {
    app.configure_sets(Startup, LoadAssetsSet);
}
