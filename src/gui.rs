use std::sync::mpsc::{Receiver, Sender};
use rss::Item;
use crate::structs::{Podcast, PodclawError, PodclawJob, PodclawSuccess};

pub fn start_gui(job_to_main: Sender<PodclawJob>, ok_from_main: Receiver<PodclawSuccess>, err_from_main: Receiver<PodclawError>)
{
   todo!()
}

struct PodclawGUI
{
   podcasts: Vec<Podcast>,
   selection_podcast: Podcast,
   selection_episodes: Vec<Item>,
   new_podcast: Podcast
}