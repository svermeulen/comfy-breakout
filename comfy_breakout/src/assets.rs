use crate::*;

pub static ASSET_DIR: include_dir::Dir<'_> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/assets");

fn base_path(path: &str) -> String {
    format!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/{}"), path)
}

pub fn load_assets() {
    init_asset_source(&ASSET_DIR, base_path);

    let textures = vec![("ball1", "img/ball1.png")];

    load_multiple_textures(
        textures
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect_vec(),
    );

    let sounds = vec![("ball-hit", "sfx/ball-hit.oga")];

    load_multiple_sounds(
        sounds
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect_vec(),
    );
}
