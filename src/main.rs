use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[macro_use]
mod utils;
use crate::utils::*;

mod structs;
use crate::structs::*;

mod gui;

mod manage_feeds;
use crate::manage_feeds::*;

mod manage_feeds_gui;

pub const TXTD: TextDeco =
    TextDeco {
        completion: "\x1b[1m\x1b[38;2;51;153;51m[✓]\x1b[0m\x1b[0m", // green
        general: "\x1b[1m\x1b[38;2;109;186;219m[*]\x1b[0m\x1b[0m", // light blue
        important: "\x1b[1m\x1b[38;2;255;140;0m[#]\x1b[0m\x1b[0m", // orange
        error: "\x1b[1m\x1b[38;2;255;50;50m[!]\x1b[0m\x1b[0m", // red
        prompt: "\x1b[1m\x1b[38;2;89;40;138m[?]\x1b[0m\x1b[0m", // darker purple
        input: "\x1b[1m\x1b[38;2;166;77;255m[>]\x1b[0m\x1b[0m", // bright purple
        verbose: "\x1b[1m\x1b[38;2;72;135;195m[+]\x1b[0m\x1b[0m" // darker blue
    };

#[derive(Parser)]
#[command(long_about = None)]
#[command(author = "ArcanePhysics")]
#[command(version = "1.0.1")]
#[command(about = "Podclaw is a small, pure-Rust CLI application for managing podcast RSS feeds and downloading episodes from those podcasts.")]
struct Args
{
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands
{
    /// Registers a podcast with Podclaw.
    Add
    {
        /// Case-insensitive. A nickname that's used to point to a podcast.
        alias: String,

        /// The URL that hosts a podcast's RSS feed.
        link: String,

        /// The path where Podclaw will put downloaded episodes.
        download_path: PathBuf,

        /// Interpreted as hours. The interval before a cache is deemed outdated. Outdated caches are typically automatically updated.
        interval: usize,

        /// Marks a podcast as locked. This means it won't automatically update its cache, and it can't be edited.
        #[arg(required = false, short = 'l', long = "lock")]
        should_lock: bool
    },

    /// Removes a registered podcast.
    Remove
    {
        /// Case-insensitive. Alias of the podcast to remove.
        alias: String
    },

    /// Inspects a podcast or episode, displaying their details.
    Inspect
    {
        /// Case-insensitive. Alias of the podcast to inspect.
        alias: String,

        /// Optional. An index to point to a specific episode. If provided, that episode will be inspected instead of the podcast itself.
        #[arg(required = false)]
        episode_index: Option<usize>,

        /// Flips the episode indices around. Technically, Podclaw does this by default, but this reverts it.
        #[arg(required = false, short = 'r', long = "reverse")]
        reversal_flag: bool
    },

    /// Downloads an episode of a podcast.
    Get
    {
        /// Case-insensitive. Alias of the podcast to get an episode from.
        alias: String,

        /// Index of the episode to download.
        episode_index: usize,

        /// Does nothing if an alias isn't provided. Flips the episode indices around. Technically, Podclaw does this by default, but this reverts it.
        #[arg(required = false, short = 'r', long = "reverse")]
        reversal_flag: bool
    },

    /// Edits a registered podcast. All arguments are optional.
    Edit
    {
        /// Case-insensitive. Alias of the podcast to edit.
        alias: String,

        /// Case-insensitive. A nickname that's used to point to a podcast.
        #[arg(required = false, short = 'a', long = "alias")]
        new_alias: Option<String>,

        /// The URL that hosts a podcast's RSS feed.
        #[arg(required = false, short = 'l', long = "link")]
        new_link: Option<String>,

        /// The path where Podclaw will put downloaded episodes.
        #[arg(required = false, short = 'p', long = "path")]
        new_download_path: Option<PathBuf>,

        /// Interpreted as hours. The interval before a cache is deemed outdated. Outdated caches are typically automatically updated.
        #[arg(required = false, short = 'i', long = "interval")]
        new_interval: Option<usize>,
    },

    /// Updates the cache of a podcast.
    Update
    {
        /// Case-insensitive. Alias of the podcast to update.
        alias: String,
    },

    /// Repairs the storage file. Since this will delete any registered podcasts, it must be ran with a confirmation flag to work.
    Repair
    {
        /// Confirms the repair process.
        #[arg(required = false, short = 'c', long = "confirm")]
        confirmation_flag: bool
    },

    /// Toggles the lock of a registered podcast.
    Lock
    {
        /// Case-insensitive. Alias of the podcast to lock/unlock.
        alias: String,
    },

    /// Lists all registered podcasts, or all episodes in a specific one.
    List
    {
        /// Optional. An alias for a podcast. If provided, the episodes of that podcast will listed instead of all podcasts.
        #[arg(required = false)]
        alias: Option<String>,

        /// Does nothing if an alias isn't provided. Flips the episode indices around. Technically, Podclaw does this by default, but this reverts it.
        #[arg(required = false, short = 'r', long = "reverse")]
        reversal_flag: bool
    }
}

fn main()
{
    let args = Args::parse();

    match args.command
    {
        Some(Commands::Add {alias, link, download_path, interval, should_lock}) =>
        {
            if alias == ""
            {
                println!("{} Invalid alias.", TXTD.error);
                return;
            }

            let podcasts = get_storage();
            add_podcast(alias.to_lowercase(), link, download_path, interval, should_lock, podcasts);
        }

        Some(Commands::Remove {alias}) =>
            {
                let podcasts = get_storage();

                if let Some(index) = find_podcast(&alias.to_lowercase(), &podcasts)
                {
                    remove_podcast(index, podcasts);
                }

                else { println!("{} There is no podcast with that alias.", TXTD.error) }
            }

        Some(Commands::Inspect {alias, episode_index, reversal_flag}) =>
        {
            let mut podcasts = get_storage();

            if let Some(podcast_index) = find_podcast(&alias.to_lowercase(), &podcasts)
            {

                podcasts = do_autocache(podcast_index, podcasts);

                if episode_index != None
                {
                    println!("Episode index provided: {}", episode_index.unwrap());
                    inspect_podcast_cli(podcast_index, episode_index.unwrap(), true, reversal_flag, podcasts);
                }

                else { inspect_podcast_cli(podcast_index, 0, false, reversal_flag, podcasts) }
            }

            else { println!("{} There is no podcast with that alias.", TXTD.error) }
        }

        Some(Commands::Get { alias, episode_index, reversal_flag }) =>
        {
            let mut podcasts = get_storage();

            if let Some(podcast_index) = find_podcast(&alias.to_lowercase(), &podcasts)
            {
                podcasts = do_autocache(podcast_index, podcasts);
                get_episode(podcast_index, episode_index, reversal_flag, podcasts);
            }
        }

        Some(Commands::Edit { alias, new_alias, new_link, new_download_path, new_interval }) =>
        {
            let podcasts = get_storage();

            if let Some(index) = find_podcast(&alias.to_lowercase(), &podcasts)
            {
                edit_podcast(index, new_alias, new_link, new_download_path, new_interval, podcasts);
            }

            else { println!("{} There is no podcast with that alias.", TXTD.error) }
        }

        Some(Commands::Update {alias}) =>
            {
                let podcasts = get_storage();

                if let Some(index) = find_podcast(&alias.to_lowercase(), &podcasts)
                {
                    update_podcast(index, podcasts);
                }

                else { println!("{} There is no podcast with that alias.", TXTD.error) }
            }

        Some(Commands::Repair {confirmation_flag}) =>
        { repair_storage(confirmation_flag) }

        Some(Commands::Lock {alias}) =>
        {
            let podcasts = get_storage();

            if let Some(index) = find_podcast(&alias.to_lowercase(), &podcasts)
            {
                lock_podcast(index, podcasts);
            }

            else { println!("{} There is no podcast with that alias.", TXTD.error) }
        }

        Some(Commands::List { alias, reversal_flag }) =>
        {
            let podcasts = get_storage();
            list_podcasts_or_episodes_cli(alias, reversal_flag, podcasts);
        }

        None => { println!("{} No commands provided. Run \'help\' to see all options. Eventually, the GUI will start here.", TXTD.error) }
    }
}