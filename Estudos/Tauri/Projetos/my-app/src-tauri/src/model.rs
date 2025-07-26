use serde::Serialise;

#[derive(Serialise)]
pub struct Song {
    pub title: String,
}