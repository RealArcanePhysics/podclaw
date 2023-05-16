
# Podclaw

Podclaw is a small, pure-Rust CLI application for managing podcast RSS feeds and downloading episodes from those podcasts.


## Installation

The preferred method is to grab it through Cargo, if you have it installed. You can use this command if you do:

```bash
cargo install podclaw
```

You can also grab a pre-compiled release, though there's currently only a Linux one. You might want to put the executable on your PATH for easier access.
    
## Building from Source

Mostly the same as any other Rust project. Clone the repo into a folder, open a terminal, and run 'cargo build'. You might need the 'librust-openssl-dev' package on Linux; not sure about Windows.

## Usage

To show off basic usage of Podclaw, this will guide you through registering a podcast and downloading the first episode from it. Quickly before that, it's worth mentioning I used TrueColor for this program. It might not look right if you're on a terminal that doesn't support it.

### Adding a Podcast

In a terminal, run a command like this.

```bash
podclaw add example [LINK] 'Podcasts' 1
```
This command adds a podcast with an alias of 'example'. An alias is a simple, case-insensitive name used to refer to this podcast in other commands. '[LINK]', of course, should be replaced with a link to an RSS feed. We then set this podcast's download path to a folder named 'Podcasts', which will be where are downloaded episodes will be placed. Finally, we set the hour interval between automatic cache updates to 1. Everytime this duration is elapsed, Podclaw will cache the RSS feed or update this cache.

### Downloading an Episode

```bash
podclaw get example 0
```
After adding a podcast, we can use a command like above to download the episode at index 0 for the podcast with the alias 'example'.

### Some Other Features
Here's some bullet points for a few other commands that Podclaw features.

- 'list', which can list all registered podcasts or their episodes
- 'inspect', which allows you see details on any podcast or episode
- 'lock', which lets you lock a podcast to prevent it from being edited, updated, or removed(until unlocked that is)

Finally, if you should need it, you can find Podclaw's storage file in '~/.config/podclaw' on Linux.
## Contributing

I'm still pretty new to Rust, and I'd love to hear some feedback. Feel free to open an issue or pull request, if you'd like.

## Possible Future Features

Here's some things I'm considering adding:

- A graphical interface, using EGUI.
- Concurrent downloads.

## Used Crates

- [Clap](https://crates.io/crates/clap)
- [Dirs](https://crates.io/crates/dirs)
- [Reqwest](https://crates.io/crates/reqwest)
- [RSS](https://crates.io/crates/rss)
- [Serde](https://serde.rs/)
- [Bincode](https://crates.io/crates/bincode)
