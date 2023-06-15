use nanoid::nanoid;

pub fn get_url_short_id() -> String {
    let max_url_length: usize = 8;
    let short_hash = nanoid!(max_url_length);
    short_hash
}
