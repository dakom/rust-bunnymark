pub const N_BUNNIES_PER_TICK:usize = 2;

pub fn get_media_href(path:&str) -> String {
    format!("media/{}", path)
}
