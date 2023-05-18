use rss::Item;
use crate::structs::Podcast;

struct PodclawGUI
{
   podcasts: Vec<Podcast>,
   selection_podcast: Podcast,
   selection_episodes: Vec<Item>,
   new_podcast: Podcast
}