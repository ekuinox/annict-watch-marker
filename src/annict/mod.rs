pub mod episode;
pub mod record;
pub mod status;
pub mod work;

use derive_new::new;

#[derive(new, Debug)]
pub struct AnnictClient {
    access_token: String,
}
