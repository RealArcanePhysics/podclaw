// All the functions from 'manage_feeds', but modified for use with the GUI.
// Not the best option for this, but that's what happens when you duct tape a GUI to a CLI program I guess.
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::{Duration, SystemTime};
use rss::Channel;
use crate::structs::{Podcast, PodclawError, PodclawSuccess};
use crate::structs::PodclawSuccess::AutoUpdatedPodcast;
use crate::TXTD;
use crate::utils::*;

/// This function detects if a podcast's cache is outdated and updates it.
pub fn do_autocache_gui(mut podcasts: Vec<Podcast>) -> Result<PodclawSuccess, PodclawError>//Vec<Podcast>
{
    todo!()
}

pub fn add_podcast_gui()
{
    todo!()
}

pub fn remove_podcast_gui()
{
    todo!()
}

pub fn update_podcast_gui()
{
    todo!()
}

pub fn download_episode_gui()
{
    todo!()
}