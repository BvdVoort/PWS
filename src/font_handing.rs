use bevy::{
    app::{
        Plugin, 
        PreStartup
    }, 
    asset::{
        AssetServer, 
        Handle
    }, 
    prelude::{
        Commands, 
        Res, 
        Resource
    }, 
    text::Font
};

const FONT_DIRECTORY_PATH: &str = "fonts/";

#[derive(Resource)]
pub struct FontHandles {
    fira_sans: Handle<Font> 
} 

impl FontHandles {
    pub fn default_font(&self) -> Handle<Font> { self.fira_sans.clone() }
}

fn load_fonts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(FontHandles {
        fira_sans: asset_server.load([FONT_DIRECTORY_PATH, "FiraSans-Bold.ttf"].concat()),
    });
}

// "FiraCode/FiraCode-Regular.ttf"

pub struct FontPlugin;
impl Plugin for FontPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreStartup, load_fonts);
    }
}

#[warn(dead_code)]
pub fn font_initialized(font: Option<Res<FontHandles>>) -> bool { font.is_some() }