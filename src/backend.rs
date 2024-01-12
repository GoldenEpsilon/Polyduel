use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationEvents {
    pub just_finished: bool,
}

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Resource)]
pub struct SpriteResource {

}

pub fn get_spriteset(folder: String, sprites: Res<SpriteResource>) -> TextureAtlas {
    
}

pub fn add_spriteset(folder: String) -> TextureAtlas {
    let folder = asset_server.load_folder("textures/rpg");
    // Build a `TextureAtlas` using the individual sprites
    let mut texture_atlas_builder =
        TextureAtlasBuilder::default().padding(padding.unwrap_or_default());
    for handle in folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(texture) = textures.get(id) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };

        texture_atlas_builder.add_texture(id, texture);
    }

    let texture_atlas = texture_atlas_builder.finish(textures).unwrap();

    // Update the sampling settings of the texture atlas
    let image = textures.get_mut(&texture_atlas.texture).unwrap();
    image.sampler = sampling.unwrap_or_default();

    return texture_atlas;
}

pub fn animation_system(
    mut q_entities: Query<(
        &mut TextureAtlasSprite, 
        &AnimationIndices, 
        &mut AnimationTimer, 
        Option<&mut AnimationEvents>
    )>, time: Res<Time>){
    for (mut sprite, indices, mut timer, mut events) in &mut q_entities {
        timer.tick(time.delta());
        sprite.index = if sprite.index == indices.last {
            indices.first
        } else {
            sprite.index + 1
        };
        if let Some(mut events) = events {
            events.just_finished = sprite.index == indices.first;
        }
    }
}