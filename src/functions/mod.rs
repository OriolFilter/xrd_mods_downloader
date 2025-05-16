// Functions

use std::{fs, io};
use std::fs::File;
use std::path::Path;
use std::process::exit;
use std::time::Duration;
use downloader::{Download,downloader::Builder};
use zip::ZipArchive;

use crate::stuff::*;

pub fn get_xrd_folder_from_file (steam_vdf_file_path: String) -> std::io::Result<String>  {
    let contents = fs::read_to_string(steam_vdf_file_path)?.replace("\t"," ");

    let mut xrd_line: i32=-1;
    let xrd_game_id_txt = "\"520440\"";

    let mut file_lines = contents.lines();
    let mut last_storage_path:String="".to_string();
    let mut current_line_count = 0;
    let mut current_line_string: String;

    while xrd_line < 0 && current_line_count < contents.lines().count() {
        current_line_string = file_lines.next().unwrap().to_string();

        if current_line_string.contains(xrd_game_id_txt)  {
            xrd_line = current_line_count as i32;
        }

        if current_line_string.contains("\"path\"") && xrd_line < 0 {
            let mut tmp_path: String = current_line_string;
            tmp_path = tmp_path.trim().to_string(); // remove extra spaces left right
            tmp_path = tmp_path.strip_prefix("\"path\"").unwrap().to_string(); // Remove starter "path"
            tmp_path = tmp_path.trim().to_string(); // Trim again
            tmp_path = tmp_path.replace("\"",""); // Remove quotes
            last_storage_path = tmp_path;
        }

        current_line_count +=1;
    }

    if xrd_line < 0 {
        println!("Xrd not found, exitting...");
        exit(1);
    }

    if cfg!(windows) {
        Ok(format!("{}\\steamapps\\common\\GUILTY GEAR Xrd -REVELATOR-",last_storage_path))
    } else {
        Ok(format!("{}/steamapps/common/GUILTY GEAR Xrd -REVELATOR-",last_storage_path))
    }
}

pub fn print_different_versions(current:&AppStruct, latest:&TagInfo) -> bool {
    // for convenience returns true if a new version is fouund.

    println!("Checking updates for app: {}",current.get_app_name());

    if current.tag_name == latest.tag_name && current.published_at == latest.published_at {
        println!("[✅ ] APP {} is up to date!",current.get_app_name());
        return false
    } else {
        println!("[⚠️ ] APP {} has a new version detected.",current.get_app_name());

        // Version
        println!("Version:\t'{}' -> '{}'",current.tag_name,latest.tag_name);
        // Published date
        println!("Published date: '{}' -> '{}'",current.published_at,latest.published_at);
        // Source URL
        println!("Source URL: '{}'",latest.html_url);
        // Print notes
        println!("Version notes:\n============\n{}\n============",latest.body.replace("\\n","\n").replace("\\r",""));
    }
    true
}

pub fn download_file_to_path(file_url: String, destination_dir: String){
    // Download overlay.zip
    let file_to_download = Download::new(&file_url);
    let destination_file_path = &format!("{}/{}", destination_dir, file_to_download.file_name.to_str().unwrap().to_string());

    // Check if file already exists
    let mut is_present:bool=Path::new(destination_file_path).exists();
    let mut is_dir:bool=Path::new(destination_file_path).is_dir();

    match (is_present,is_dir) {
        (true,false) => {
            // println!("A file with the name '{}' already exists, proceeding with the deletion.",destination_file_path);
            fs::remove_file(destination_file_path);
        }
        (true,true) => {
            // Error won't delete a folder
            // println!("The file '{}' cannot be downloaded due to a directory having the exact same name.",destination_file_path);
            exit(1);
        }
        _ => {}

    }

    // let mut is_dir:bool=Path::new(mod_folder).is_dir();

    // copy pasta
    // https://github.com/hunger/downloader
    let mut dl = Builder::default()
        .connect_timeout(Duration::from_secs(4))
        .download_folder(Path::new(&destination_dir))
        .parallel_requests(8)
        .build()
        .unwrap();

    let response = dl.download(&[file_to_download]).unwrap(); // other error handling

    response.iter().for_each(|v| match v {
        Ok(v) => {}
            // println!("Downloaded: {:?}", v),
        Err(e) => println!("Error: {:?}", e),
    });
}

pub fn unzip_file(zip_file_path: String, unzip_dir:String){
    // this was a copy pasta from somewhere

    let zipfile = File::open(&zip_file_path).unwrap();

    let mut archive = ZipArchive::new(zipfile).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = format!("{}/{}",unzip_dir,file.name());

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if file.is_dir() {
            // println!("File {} extracted to \"{}\"", i, outpath);
            fs::create_dir_all(&outpath).unwrap();
        } else {
            // println!("File {} extracted to \"{}\" ({} bytes)",i,outpath,file.size());
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    // println!("File '{}' extracted to '{}'",zip_file_path,unzip_dir);
}