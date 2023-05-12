use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::fs::{create_dir, File};
use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::{TXTD};
use crate::structs::Podcast;

macro_rules! italicize
{
    ($string:expr) =>
    {
        format!("\x1b[3m{}\x1b[0m", $string)
    }
}


macro_rules! make_bold
{
    ($string:expr) =>
    {
        format!("\x1b[1m{}\x1b[0m", $string)
    }
}


pub fn write_to_bin<T: Serialize>(payload: &T, path: &Path) -> Result<(), Box<dyn std::error::Error>>
{
    let mut target_file = File::create(path)?;
    let encoded_payload: Vec<u8> = bincode::serialize(&payload)?;

    target_file.write_all(&encoded_payload)?;

    Ok(())
}


pub fn read_from_bin<T: DeserializeOwned>(path: &Path) -> Result<T, Box<dyn std::error::Error>>
{
    let mut target_file = File::open(path)?;
    let mut retrieved_data: Vec<u8> = Vec::new();
    target_file.read_to_end(&mut retrieved_data)?;
    //target_file.read_to_string(&mut str_from_ron)?;
    let decoded_data: T = bincode::deserialize(&retrieved_data)?;

    Ok(decoded_data)
}


pub fn find_podcast(target_alias: &String, podcasts: &[Podcast]) -> Option<usize>
{
    for (index, podcast) in podcasts.iter().enumerate()
    {
        if target_alias == &podcast.alias
        {
            return Some(index);
        }
    }

    None
}

pub fn get_storage() -> Vec<Podcast>
{
    if let Ok(storage) = read_from_bin::<Vec<Podcast>>(Path::new(get_storage_path().as_path()))
    {storage}

    else
    {
        panic!("{} Storage seems invalid. Try running the 'repair' command!", TXTD.error);
    }
}

pub fn get_storage_path() -> PathBuf
{
    return if let Some(config_path) = dirs::config_dir()
    {
        let mut storage_path = config_path.join(Path::new("podclaw"));

        if !storage_path.try_exists().unwrap()
        { create_dir(storage_path.clone()).expect("Failed to create Podclaw's config path.") }

        storage_path = storage_path.join(Path::new("podclaw_storage.bin"));
        storage_path
    }

    else
    {
        println!("{} Can't find config directory on this system, keeping storage file in working directory...", TXTD.important);
        return PathBuf::from("podclaw_storage.bin");
    }
}

pub fn repair_storage(confirmation: bool)
{
    if confirmation
    {
        println!("{} Repairing storage...", TXTD.general);

        let new_storage: Vec<Podcast> = Vec::new();
        write_to_bin(&new_storage, get_storage_path().as_path()).expect(format!("Failed to repair \'{}\'", get_storage_path().to_str().unwrap()).as_str());

        println!("{} Done!", TXTD.completion);
    }

    else { println!("{} No confirmation flag was set.", TXTD.error) }
}