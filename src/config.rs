pub const N_BUNNIES_PER_TICK:usize = 100;

pub fn get_media_href(path:&str) -> String {
    format!("media/{}", path)
}
