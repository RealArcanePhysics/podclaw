use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Podcast
{
    pub alias: String,
    pub feed_url: String,
    pub download_path: PathBuf,
    pub update_interval: Duration,
    pub cache_time: SystemTime,
    pub cache_content: String,
    pub is_locked: bool
}

impl Podcast
{
    pub fn new() -> Self
    {
        Podcast
        {
            alias: String::new(),
            feed_url: String::new(),
            download_path: PathBuf::new(),
            update_interval: Duration::default(),
            cache_time: UNIX_EPOCH,
            cache_content: String::new(),
            is_locked: false
        }
    }
}

#[derive(Clone)]
pub struct TextDeco
{
    pub completion: &'static str,
    pub general: &'static str,
    pub important: &'static str,
    pub error: &'static str,
    pub prompt: &'static str,
    pub input: &'static str,
    pub verbose: &'static str
}

// These enums are all for communication between the main and GUI threads.
#[derive(Clone)]
pub enum PodclawJob
{
AddPodcast(Podcast, Vec<Podcast>),
RemovePodcast(String, Vec<Podcast>),
EditPodcast(String, Vec<Podcast>),
UpdatePodcast(String),
DownloadEpisode(String, usize, bool)
}

pub enum PodclawSuccess
{
    AddedPodcast(Vec<Podcast>),
    RemovedPodcast(Vec<Podcast>),
    EditedPodcast(Vec<Podcast>),
    UpdatedPodcast,
    AutoUpdatedPodcast(Vec<Podcast>),
    DownloadedEpisode(String)
}

pub enum PodclawError
{
    Generic,
    RequestFailed,
    UpdateFailed,
    PodcastLocked,
    PodcastNotFound,
    AudioNotFound,
    InvalidFeed
}