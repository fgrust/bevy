use bevy_asset::{Assets, Handle};
use bevy_render::{draw::{DrawContext, Draw}, texture::Texture, Color};
use bevy_sprite::{ColorMaterial, ComMut, Quad, TextureAtlas};
use bevy_text::{Font, FontAtlasSet, TextStyle};
use legion::prelude::{Com, Res, ResMut};

pub struct Label {
    pub text: String,
    pub font: Handle<Font>,
    pub style: TextStyle,
}

impl Default for Label {
    fn default() -> Self {
        Label {
            text: String::new(),
            style: TextStyle {
                color: Color::WHITE,
                font_size: 12.0,
            },
            font: Handle::default(),
        }
    }
}

impl Label {
    // PERF: this is horrendously inefficient. (1) new texture per label per frame (2) no atlas
    pub fn label_system(
        mut color_materials: ResMut<Assets<ColorMaterial>>,
        mut textures: ResMut<Assets<Texture>>,
        fonts: Res<Assets<Font>>,
        mut font_atlas_sets: ResMut<Assets<FontAtlasSet>>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        label: Com<Label>,
        quad: Com<Quad>,
        color_material_handle: Com<Handle<ColorMaterial>>,
    ) {
        // ensure the texture is at least 1x1
        let width = quad.size.x().max(1.0);
        let height = quad.size.y().max(1.0);

        if let Some(font) = fonts.get(&label.font) {
            let font_atlases = font_atlas_sets
                .get_or_insert_with(Handle::from_id(label.font.id), || {
                    FontAtlasSet::new(label.font)
                });
            font_atlases.add_glyphs_to_atlas(
                &fonts,
                &mut texture_atlases,
                &mut textures,
                label.style.font_size,
                &label.text,
            );
            let texture = font.render_text(
                &label.text,
                label.style.color,
                label.style.font_size,
                width as usize,
                height as usize,
            );

            let material = color_materials.get_or_insert_with(*color_material_handle, || {
                ColorMaterial::from(Handle::<Texture>::new())
            });
            if let Some(texture_handle) = material.texture {
                textures.set(texture_handle, texture);
            } else {
                material.texture = Some(textures.add(texture));
            }
        }
    }

    pub fn draw_label_system(
        mut _draw_context: DrawContext,
        _fonts: Res<Assets<Font>>,
        _font_atlas_sets: Res<Assets<FontAtlasSet>>,
        _texture_atlases: Res<Assets<TextureAtlas>>,
        mut _draw: ComMut<Draw>,
        _label: Com<Label>,
        _quad: Com<Quad>,
    ) {
        // let mut drawable_text = DrawableText::new(
        //     fonts.get(&label.font).unwrap(),
        //     font_atlas_sets
        //         .get(&label.font.as_handle::<FontAtlasSet>())
        //         .unwrap(),
        //     &texture_atlases,
        //     quad.position,
        //     &label.style,
        //     &label.text,
        // );
        // drawable_text.draw(&mut draw, &mut draw_context).unwrap();
    }
}
