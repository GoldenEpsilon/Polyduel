use bevy::{prelude::*, utils::{HashMap, BoxedFuture}, reflect::{TypePath, TypeUuid}, asset::{AssetLoader, LoadContext, LoadedAsset}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, TypeUuid, TypePath)]
#[uuid = "59594599-4785-408e-8e11-89eb9f1c5b59"]
pub struct AnimationInfo {
    animations: HashMap<String, SpriteInfo>
}

#[derive(Default)]
pub struct AnimationInfoLoader;

impl AssetLoader for AnimationInfoLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let mut custom_asset = ron::de::from_bytes::<AnimationInfo>(bytes)?;

            let mut revised_hashmap = HashMap::new();
            for (animation_name, animation_info) in custom_asset.animations {
                revised_hashmap.insert(animation_name.to_lowercase(), animation_info);
            }

            custom_asset.animations = revised_hashmap;

            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["anim.ron"]
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct SpriteInfo {
    x_offset: u32,
    y_offset: u32,
    x_frames: u32,
    y_frames: u32
}

#[derive(Component)]
pub struct AnimationData {
    timer: Timer,
    pub index: usize,
    pub just_finished: bool,
    pub animation_name: String,
    anim_handles: Vec<Handle<Image>>,
}

impl AnimationData {
    pub fn new(animation_name: String, atlas: &SpriteAtlas) -> AnimationData {
        if let Some(animation_data) = atlas.animation_data.get(&animation_name.to_lowercase()) {
            return AnimationData { 
                timer: Timer::from_seconds(2./30., TimerMode::Repeating), 
                index: 0,
                just_finished: false, 
                anim_handles: animation_data.anim_handles.to_owned(), 
                animation_name };
        } else {
            return AnimationData { 
                timer: Timer::from_seconds(0.2, TimerMode::Repeating), 
                index: 0,
                just_finished: false, 
                anim_handles: vec![], 
                animation_name };
        }
    }
    pub fn get_atlas_index(&self, atlas: &SpriteAtlas,
        texture_atlases: &Res<Assets<TextureAtlas>>,) -> Option<usize> {
        if let Some(atlas) = texture_atlases.get(&atlas.atlas) {
            return atlas.get_texture_index(&self.anim_handles[self.index]);
        } else {
            return None;
        }
    }
}

#[derive(Resource)]
pub struct SpriteRes {
    pub atlases: HashMap<String, SpriteAtlas>
}

pub struct SpriteAtlas {
    pub atlas: Handle<TextureAtlas>,
    pub animation_data: HashMap<String, SpriteData>
}

#[derive(Debug)]
pub struct SpriteData {
    anim_handles: Vec<Handle<Image>>,
}

pub fn spriteset_setup(
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Image>>,
    mut spriteres: ResMut<SpriteRes>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    animation_info: ResMut<Assets<AnimationInfo>>){
    let mut texture_atlas_builders: HashMap<String, (TextureAtlasBuilder, HashMap<String, SpriteData>, usize)> = HashMap::new();
    let _handles: Vec<HandleUntyped> = asset_server.load_folder("./").unwrap();
    for _handle in _handles {
        let handle = _handle.typed_weak();
        if let Some(texture) = textures.get(&handle) {
            match asset_server.get_handle_path(handle.to_owned()) {
                Some(assetpath) => {
                    if let Some(parent_folder) = assetpath.path().parent() {
                        if let Some(name) = parent_folder.file_name() {
                            let key: String = name.to_string_lossy().to_string().to_lowercase();
                            if !texture_atlas_builders.contains_key(&key) {
                                texture_atlas_builders.insert(key.to_owned(), (TextureAtlasBuilder::default(), HashMap::new(), 0));
                            }
                            if let Some(atlas) = texture_atlas_builders.get_mut(&key) {
                                if let Some(file_osstr) = assetpath.path().file_stem() {
                                    let file_name: String = file_osstr.to_string_lossy().to_string().to_lowercase();
                                    let info_handle: Handle<AnimationInfo> = asset_server.load(parent_folder.join("animation_info.anim.ron"));
                                    match animation_info.get(&info_handle) {
                                        Some(anim_info) => {
                                            if anim_info.animations.contains_key(&file_name) {
                                                if let Ok(sprite) = texture.to_owned().try_into_dynamic(){
                                                    let mut i = 0;
                                                    let SpriteInfo{x_frames, y_frames, x_offset, y_offset} = anim_info.animations[&file_name];
                                                    let (width, height) = (sprite.width() / x_frames.max(1), sprite.height() / y_frames.max(1));
                                                    while i < anim_info.animations[&file_name].x_frames {
                                                        let texture_handle = textures.add(
                                                            Image::from_dynamic(sprite.crop_imm(
                                                                x_offset + width * (i % x_frames), 
                                                                y_offset + height * (((i / x_frames) as f32).floor() as u32), 
                                                                width, 
                                                                height), 
                                                                true)
                                                        );
                                                        let texture = textures.get(&texture_handle).unwrap();
                                                        atlas.0.add_texture(texture_handle.to_owned(), texture);
                                                        if let Some(anim_info) = atlas.1.get_mut(&file_name) {
                                                            anim_info.anim_handles.push(texture_handle);
                                                        }else {
                                                            atlas.1.insert(file_name.to_owned(), SpriteData { anim_handles: vec![texture_handle] });
                                                        }
                                                        atlas.2 += 1;
                                                        i += 1;
                                                    }
                                                }
                                            } else {
                                                atlas.0.add_texture(handle.to_owned(), texture);
                                                if let Some(anim_info) = atlas.1.get_mut(&file_name) {
                                                    anim_info.anim_handles.push(handle);
                                                }else {
                                                    atlas.1.insert(file_name.to_owned(), SpriteData { anim_handles: vec![handle] });
                                                }
                                                atlas.2 += 1;
                                            }
                                        }
                                        _ => {
                                            atlas.0.add_texture(handle.to_owned(), texture);
                                            if let Some(anim_info) = atlas.1.get_mut(&file_name) {
                                                anim_info.anim_handles.push(handle);
                                            }else {
                                                atlas.1.insert(file_name.to_owned(), SpriteData { anim_handles: vec![handle] });
                                            }
                                            atlas.2 += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    for (key, (atlas, animation_data, _index)) in texture_atlas_builders {
        if let Ok(atlas) = atlas.finish(&mut textures) {
            if spriteres.atlases.contains_key(&key) {
                spriteres.atlases.remove(&key);
            }

            spriteres.atlases.insert(key, SpriteAtlas {
                atlas: texture_atlases.add(atlas),
                animation_data
            });
        }
    }
}

pub fn animation_system(
    mut q_entities: Query<(
        &Handle<TextureAtlas>, 
        &mut TextureAtlasSprite, 
        &mut AnimationData
    )>, time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,){
    for (atlas, mut sprite, mut animdata) in &mut q_entities {
        if let Some(atlas) = texture_atlases.get(atlas) {
            animdata.timer.tick(time.delta());
            if animdata.timer.just_finished() {
                animdata.index = if animdata.index >= animdata.anim_handles.len() - 1 {
                    animdata.just_finished = true;
                    0
                } else {
                    animdata.index + 1
                };
                if let Some(index) = atlas.get_texture_index(&animdata.anim_handles[animdata.index]) {
                    sprite.index = index;
                }
            }
        }
    }
}