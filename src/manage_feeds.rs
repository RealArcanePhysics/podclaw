use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::{Duration, SystemTime};
use rss::Channel;
use crate::structs::Podcast;
use crate::TXTD;
use crate::utils::*;

// This function detects if a podcast's cache is outdated and updates it.
pub fn do_autocache(target_index: usize, mut podcasts: Vec<Podcast>) -> Vec<Podcast>
{
    let mut target_podcast = &mut podcasts[target_index];

    if target_podcast.is_locked {return podcasts}

    if let Ok(cache_age) = SystemTime::now().duration_since(target_podcast.cache_time)
    {
        if cache_age > target_podcast.update_interval
        {
            println!("{} This podcast's cache is outdated, updating...", TXTD.important);
            if let Ok(new_feed_raw) = reqwest::blocking::get(target_podcast.feed_url.clone())
            {
                if let Ok(updated_feed) = Channel::read_from(BufReader::new(new_feed_raw))
                {
                    target_podcast.cache_content = updated_feed.to_string();
                    target_podcast.cache_time = SystemTime::now();

                    write_to_bin(&podcasts, get_storage_path().as_path()).unwrap();

                    println!("{} Cache updated!", TXTD.completion)
                }

                else { println!("{} Failed to update cache automatically, due to receiving an invalid feed.", TXTD.important) }
            }

            else { println!("{} Failed to update cache automatically, due to receiving an invalid feed.", TXTD.important) }
        }
    }

    // Handles the current time being before the cache's timestamp.
    else
    {
        if let Ok(new_feed_raw) = reqwest::blocking::get(target_podcast.feed_url.clone())
        {
            if let Ok(updated_feed) = Channel::read_from(BufReader::new(new_feed_raw))
            {
                target_podcast.cache_content = updated_feed.to_string();
                target_podcast.cache_time = SystemTime::now();
                write_to_bin(&podcasts, get_storage_path().as_path()).unwrap();
            }
        }

        else { println!("{} Failed to update cache automatically, due to receiving an invalid feed.", TXTD.important) }
    }

    return podcasts;
}


pub fn add_podcast(new_alias: String, new_link: String, download_path: PathBuf, interval: usize, should_lock: bool, mut podcasts: Vec<Podcast>)
{
    for podcasts in &podcasts
    {
        if new_alias == podcasts.alias
        {
            println!("{} Alias already in use.", TXTD.error);
            return;
        }
    }

    println!("{} Registering new podcast with this alias: \'{}\'", TXTD.general, new_alias);
    let mut new_podcast = Podcast::new();
    new_podcast.alias = new_alias;



    if let Ok(new_feed_raw) = reqwest::blocking::get(new_link.clone())
    {
        if let Ok(new_feed) = Channel::read_from(BufReader::new(new_feed_raw))
        {
            println!("{} Podcast will use this link: {}", TXTD.general, italicize!(format!("\'{}\'", new_link)));
            new_podcast.feed_url = new_link;

            println!("{} This podcast will save its downloaded files to: {}", TXTD.general, italicize!(format!("\'{}\'", download_path.to_str().unwrap())));
            new_podcast.download_path = download_path;

            println!("{} This podcast will keep its cache for this many hours: {}", TXTD.general, italicize!(format!("\'{}\'", interval)));
            new_podcast.update_interval = Duration::from_secs( (interval as u64 * 60) * 60 );

            new_podcast.cache_time = SystemTime::now();
            new_podcast.cache_content = new_feed.to_string();
            println!("{} Parsed RSS feed and created initial cache.", TXTD.general);

            if should_lock { new_podcast.is_locked = true }
            else { new_podcast.is_locked = false }
        }

        else
        {
            println!("{} Failed to parse RSS feed.", TXTD.error);
            return;
        }
    }

    else
    {
        println!("{} Failed to request feed from provided link.", TXTD.error);
        return;
    }

    println!("{} Writing podcast and cache to storage...", TXTD.general);
    podcasts.push(new_podcast);
    write_to_bin(&podcasts, get_storage_path().as_path()).expect("Failed to write to storage!");

    println!("{} Done!", TXTD.completion);
}


pub fn remove_podcast(target_index: usize, mut podcasts: Vec<Podcast>)
{
    println!("{} Removing {} from podcasts...", TXTD.general, italicize!(format!("\'{}\'", podcasts[target_index].alias)));

    podcasts.swap_remove(target_index);
    write_to_bin(&podcasts, get_storage_path().as_path()).expect("Failed to write to storage!");

    println!("{} Done!", TXTD.completion);
}


pub fn edit_podcast(target_index: usize, new_alias_opt: Option<String>, new_link_opt: Option<String>, new_dl_path_opt: Option<PathBuf>, new_interval_opt: Option<usize>, mut podcasts: Vec<Podcast>)
{
    let mut podcast = &mut podcasts[target_index];

    if podcast.is_locked
    {
        println!("{} Podcast is locked.", TXTD.error);
        return;
    }

    println!("{} Editing {}...", TXTD.general, italicize!(format!("\'{}\'", podcast.alias)));

    if let Some(new_alias) = new_alias_opt.clone()
    {
        podcast.alias = new_alias.clone();
        println!("{} Changed alias to {}!", TXTD.general, italicize!(format!("\'{}\'", new_alias)));
    }

    if let Some(new_link) = new_link_opt.clone()
    {
        podcast.feed_url = new_link.clone();
        println!("{} Changed feed link to {}!", TXTD.general, italicize!(format!("\'{}\'", new_link)));
    }

    if let Some(new_dl_path) = new_dl_path_opt.clone()
    {
        podcast.download_path = new_dl_path.clone();
        println!("{} Changed download path to {}!", TXTD.general, italicize!(format!("\'{}\'", new_dl_path.to_str().unwrap())));
    }

    if let Some(new_interval) = new_interval_opt.clone()
    {
        podcast.update_interval = Duration::from_secs( (new_interval as u64 * 60) * 60 );
        println!("{} Changed update interval to {}!", TXTD.general, italicize!(format!("\'{}\'", new_interval)));
    }

    if new_alias_opt.is_none() && new_link_opt.is_none() && new_dl_path_opt.is_none() && new_interval_opt.is_none()
    {
        println!("{} No changes made.", TXTD.important);
    }

    else
    {
        write_to_bin(&podcasts, get_storage_path().as_path()).unwrap();
        println!("{} Successfully edited podcast!", TXTD.completion);
    }
}

pub fn update_podcast(target_index: usize, mut podcasts: Vec<Podcast>)
{
    let mut target_podcast = &mut podcasts[target_index];

    if target_podcast.is_locked
    {
        println!("{} Podcast is locked.", TXTD.error);
        return;
    }

    println!("{} Updating podcast {}...", TXTD.general, italicize!(format!("\'{}\'", target_podcast.alias)));
    if let Ok(new_feed_raw) = reqwest::blocking::get(target_podcast.feed_url.clone())
    {
        if let Ok(updated_feed) = Channel::read_from(BufReader::new(new_feed_raw))
        {
            target_podcast.cache_content = updated_feed.to_string();
            target_podcast.cache_time = SystemTime::now();

            write_to_bin(&podcasts, get_storage_path().as_path()).unwrap();

            println!("{} Cache updated!", TXTD.completion)
        } else { println!("{} Failed to update cache, due to receiving an invalid feed.", TXTD.important) }
    }
}

pub fn lock_podcast(target_index: usize, mut podcasts: Vec<Podcast>)
{
    let mut podcast = &mut podcasts[target_index];

    if podcast.is_locked
    {
        podcast.is_locked = false;
        println!("{} Successfully unlocked podcast!", TXTD.completion);
    }

    else
    {
        podcast.is_locked = true;
        println!("{} Successfully locked podcast!", TXTD.completion);
    }

    write_to_bin(&podcasts, get_storage_path().as_path()).unwrap();
}


pub fn inspect_podcast(target_index: usize, episode_index: usize, do_episode: bool, do_normal_episode_order: bool, podcasts: Vec<Podcast>)
{
    let feed = Channel::from_str
        (
            podcasts.get(target_index).unwrap()
                .cache_content.as_str()
        ).unwrap();

    if do_episode
    {
        let mut episodes = feed.items;
        if !do_normal_episode_order { episodes.reverse() }

        if episode_index > episodes.len() - 1
        {
            println!("{} Episode index is out of bounds.", TXTD.error);
            return;
        }

        let target_episode = episodes.get(episode_index).unwrap();

        println!("{} Displaying details for the requested episode...", TXTD.general);

        println!("  {} {}\n", make_bold!("Name: "), italicize!(format!("\'{}\'", target_episode.title.clone().unwrap() )));
        println!("  {} {}\n", make_bold!("Index: "), italicize!(format!("{}", episode_index)));
        println!("  {} {}\n", make_bold!("Description: "), italicize!(format!("\'{}\'", target_episode.description.clone().unwrap() )));
        println!("  {} {}", make_bold!("Link: "), italicize!(format!("\'{}\'", target_episode.enclosure.clone().unwrap().url )));
    }

    else
    {
        println!("{} Displaying details for the requested series...", TXTD.general);

        println!("  {} {}\n", make_bold!("Name: "), italicize!(format!("\'{}\'", feed.title )));
        println!("  {} {}\n", make_bold!("Creator(s): "), italicize!(format!("\'{}\'", feed.itunes_ext.unwrap().author.unwrap() )));
        println!("  {} {}", make_bold!("Description: "), italicize!(format!("\'{}\'", feed.description )));
    }
}

pub fn get_episode(target_index: usize, episode_index: usize, do_normal_episode_order: bool, podcasts: Vec<Podcast>)
{
    let podcast = podcasts.get(target_index).unwrap();
    let feed = Channel::from_str
        (
            podcast.cache_content.as_str()
        ).unwrap();

    let mut episodes = feed.items;
    if !do_normal_episode_order { episodes.reverse() }

    if episode_index < episodes.len() - 1
    {
        let target_episode = episodes.get(episode_index).unwrap();
        let full_download_path = podcast.download_path.join( Path::new( format!("[{} - {}] {}.mp3", podcast.alias, episode_index, target_episode.title.clone().unwrap()).as_str() ));

        println!("{} Downloading {}...", TXTD.general, italicize!( format!("\'{}\'", full_download_path.to_str().unwrap()) ));
        if let Ok(target_audio) = reqwest::blocking::get(target_episode.enclosure.clone().unwrap().url)
        {
            let mut new_file = File::create(full_download_path)
                .expect("Failed to create new file!");

            new_file.write_all( target_audio.bytes().unwrap().to_vec().as_ref()).expect("Failed to write to file!");
            println!("{} Done!", TXTD.completion);
        }

        else { println!("{} Failed to get audio file from the internet.", TXTD.error) }
    }

    else { println!("{} Episode index is out of bounds.", TXTD.error) }
}


pub fn list_podcasts_or_episodes(alias: Option<String>, do_normal_episode_order: bool, podcasts: Vec<Podcast>)
{
    if alias.is_none()
    {
        println!("{} Listing all registered podcasts...", TXTD.general);
        for (podcast_index, podcast) in podcasts.iter().enumerate()
        {
            println!("  {} {}", make_bold!(format!("#{}:", podcast_index)), italicize!(format!("\'{}\'", podcast.alias)));
        }
    }

    else
    {
        if let Some(target_index) = find_podcast(&alias.unwrap().to_lowercase(), &podcasts)
        {
            let feed = Channel::from_str
                (
                    podcasts.get(target_index).unwrap()
                        .cache_content.as_str()
                ).unwrap();

            let mut episodes = feed.items;
            if !do_normal_episode_order { episodes.reverse() }

            println!("{} Listing all episodes in the requested podcast...", TXTD.general);
            for (episode_index, episode) in episodes.iter().enumerate()
            {
                println!("  {} {}", make_bold!(format!("#{}:", episode_index)), italicize!(format!("\'{}\'", episode.title.clone().unwrap())))
            }
        }

        else { println!("{} There is no podcast with that alias.", TXTD.error) }
    }
}