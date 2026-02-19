use bevy::prelude::*;
use bevy_ecs_ldtk::{
    LdtkEntity, LdtkPlugin, LdtkWorldBundle, LevelSelection, app::LdtkEntityAppExt,
    assets::LdtkProject,
};

use crate::{asset_tracking::LoadResource, audio::music, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin);
    app.load_resource::<LevelAssets>();
    app.insert_resource(LevelSelection::index(0));
    // Entities
    app.register_ldtk_entity::<CastleBundle>("Castle");
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,

    #[dependency]
    ldtk_project: Handle<LdtkProject>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
            ldtk_project: assets.load("map/auto-battler-world.ldtk"),
        }
    }
}

#[derive(Default, Bundle, LdtkEntity)]
struct CastleBundle {
    #[sprite_sheet(no_grid)]
    sprite_sheet: Sprite,
}

/// A system that spawns the main level.
pub fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn((
        Name::new("Level"),
        LdtkWorldBundle {
            ldtk_handle: level_assets.ldtk_project.clone().into(),
            ..Default::default()
        },
        DespawnOnExit(Screen::Gameplay),
        children![
            Name::new("Gameplay Music"),
            music(level_assets.music.clone())
        ],
    ));
}
