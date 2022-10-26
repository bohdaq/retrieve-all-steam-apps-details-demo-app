use std::fs::{File, OpenOptions, read_to_string};
use std::path::Path;
use std::{fs, thread, time};
use std::io::{Read, Write};
use sha256::digest;

// How to use: 1. First step is to import crate functions.
use steam_webapi_rust_sdk::{get_app_list, get_app_details, get_cached_app_list};
use steam_webapi_rust_sdk::isteam_apps::get_app_list::SteamApp;
use steam_webapi_rust_sdk::util::get_cache_dir_path;

fn main() {
    println!("retrieve-all-steam-apps-details-demo-app");

   do_job()
}

fn do_job() {
    // How to use: 2. Getting app list from Steam store.


    let mut processed_app_id_list: Vec<i64> = vec![];

    println!("Getting list of already processed app ids. This may take a while...");
    let already_processed_app_id_list_path = [get_cache_dir_path(), "/".to_string(), "processed_app_id_list.json".to_string()].join("");
    let already_processed_app_id_list_path_sha_256 = [get_cache_dir_path(), "/".to_string(), "processed_app_id_list.json.sha256".to_string()].join("");
    let file_exists = Path::new(already_processed_app_id_list_path.as_str()).is_file();
    if file_exists {
        let serialized_string = read_to_string(&already_processed_app_id_list_path).unwrap();
        if serialized_string.len() > 0 {
            let boxed_processed_app_id_list = serde_json::from_str(serialized_string.as_str());
            if boxed_processed_app_id_list.is_ok() {
                processed_app_id_list = boxed_processed_app_id_list.unwrap();


                //Verification
                let list_as_string: String = format!("{:?}", &processed_app_id_list);
                let list_as_u8 : &[u8] = list_as_string.as_bytes();
                let sha_256 = digest(list_as_u8);
                println!("SHA256 deserialized list: {}", sha_256);

                let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .truncate(false)
                    .open(&already_processed_app_id_list_path_sha_256)
                    .unwrap();

                let mut sha256_from_file: String = "".to_string();
                let boxed_sha = file.read_to_string(&mut sha256_from_file);
                if boxed_sha.is_ok() {
                    println!("SHA256 from file: {}", sha256_from_file);
                }

                if sha_256 != sha256_from_file {
                    do_restore_from_backup();
                    //retry after backup restore
                    do_job();
                }
            } else {
                println!("unable to deserialize processed app list");
                do_restore_from_backup();
                do_job();
            }
        }
    } else {
        File::create(&already_processed_app_id_list_path).unwrap();
    }

    println!("Filtering already processed app details. This may take a while...");
    let mut iteration = 0;
    let mut app_list : Vec<SteamApp> = get_steam_app_list();
    let app_list_path_sha_256 = [get_cache_dir_path(), "/".to_string(), "ISteamApps-GetAppList-v2.json.sha256".to_string()].join("");
    let list_as_string: String = format!("{:?}", &app_list);
    let list_as_u8 : &[u8] = list_as_string.as_bytes();
    let digest = write_sha256(&app_list_path_sha_256, list_as_u8);
    println!("Written SHA256 for the app list: {}", digest);

    let app_list_size = app_list.len();
    let filtered_list: Vec<SteamApp> = app_list
        .into_iter()
        .filter(|steam_app| {
            iteration = iteration + 1;
            print!("\rFiltering already processed apps. Iteration {} of {}", iteration, app_list_size);
            !processed_app_id_list.contains(&steam_app.appid)

        })
        .collect();

    let filtered_list_len = filtered_list.len();

    let mut iteration_number = 1;
    for app in filtered_list {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&already_processed_app_id_list_path)
            .unwrap();
        let calculated_percentage = (100_f32 * iteration_number as f32) / filtered_list_len as f32;

        println!("\n\n Iteration number: {} \n App List size:    {}  {}%  After filtering: {}", iteration_number, app_list_size, calculated_percentage, filtered_list_len);
        retrieve_detailed_app_info(app.appid);
        iteration_number = iteration_number + 1;
        &processed_app_id_list.push(app.appid);

        let serialized_list = serde_json::to_string(&processed_app_id_list).unwrap();
        file.write_all(serialized_list.as_ref()).unwrap();

        let list_as_string: String = format!("{:?}", &processed_app_id_list);
        let list_as_u8 : &[u8] = list_as_string.as_bytes();
        let digest = write_sha256(&already_processed_app_id_list_path_sha_256, list_as_u8);
        println!("SHA256 after write for the list of already processed app ids: {}", digest);
    }
}

fn retrieve_detailed_app_info(app_id: i64) {
    // How to use: 3. Getting app details from Steam store.
    let boxed_result = get_app_details(app_id);
    if boxed_result.is_ok() {
        let app_details = boxed_result.unwrap();
        println!("result is ok for {} app id {}", app_details.name, app_details.app_id);

    } else {
        let error_message = boxed_result.err().unwrap();
        println!("{} {}", error_message, app_id);

        let is_not_steam_unsuccessful_response = error_message != "steampowered api returned failed response";
        let is_not_invalid_utf8_sequence = error_message != "invalid utf-8 sequence";
        let no_response_from_api = error_message == "no response from API";

        if (is_not_steam_unsuccessful_response && is_not_invalid_utf8_sequence) || no_response_from_api {
            println!("result is not ok for app id {}, retry in 1 min ", app_id);

            let one_minute = time::Duration::from_secs(60);
            thread::sleep(one_minute);

            retrieve_detailed_app_info(app_id);
        }
    }
}

fn do_backup() {
    let already_processed_app_id_list_path = [get_cache_dir_path(), "/".to_string(), "processed_app_id_list.json".to_string()].join("");
    let already_processed_app_id_list_path_sha_256 = [get_cache_dir_path(), "/".to_string(), "processed_app_id_list.json.sha256".to_string()].join("");

    let backup_already_processed_app_id_list_path = [get_cache_dir_path(), "/".to_string(), "backup_processed_app_id_list.json".to_string()].join("");
    let backup_already_processed_app_id_list_path_sha_256 = [get_cache_dir_path(), "/".to_string(), "backup_processed_app_id_list.json.sha256".to_string()].join("");

    let app_list_path = [get_cache_dir_path(), "/".to_string(), "ISteamApps-GetAppList-v2.json".to_string()].join("");
    let app_list_path_sha_256 = [get_cache_dir_path(), "/".to_string(), "ISteamApps-GetAppList-v2.json.sha256".to_string()].join("");

    let backup_app_list_path = [get_cache_dir_path(), "/".to_string(), "backup_ISteamApps-GetAppList-v2.json".to_string()].join("");
    let backup_app_list_path_sha_256 = [get_cache_dir_path(), "/".to_string(), "backup_ISteamApps-GetAppList-v2.json.sha256".to_string()].join("");


    let boxed_backup = fs::copy(&already_processed_app_id_list_path, &backup_already_processed_app_id_list_path);
    if boxed_backup.is_err() {
        println!("backup creation failed, exiting...");
        return;
    } else {
        println!("backup done.")
    }

    let boxed_backup_sha256 = fs::copy(&already_processed_app_id_list_path_sha_256, &backup_already_processed_app_id_list_path_sha_256);
    if boxed_backup_sha256.is_err() {
        println!("backup for sha256 creation failed, exiting...");
        return;
    } else {
        println!("backup sha256 done.")
    }

    let boxed_backup_app_list = fs::copy(&app_list_path, &backup_app_list_path);
    if boxed_backup_app_list.is_err() {
        println!("backup for app list creation failed, exiting...");
        return;
    } else {
        println!("backup for app list done.")
    }

    let boxed_backup_app_list_sha256 = fs::copy(&app_list_path_sha_256, &backup_app_list_path_sha_256);
    if boxed_backup_app_list_sha256.is_err() {
        println!("backup for app list sha256 creation failed, exiting...");
        return;
    } else {
        println!("backup for app list sha256 done.")
    }
}

fn do_restore_from_backup() {
    let already_processed_app_id_list_path = [get_cache_dir_path(), "/".to_string(), "processed_app_id_list.json".to_string()].join("");
    let already_processed_app_id_list_path_sha_256 = [get_cache_dir_path(), "/".to_string(), "processed_app_id_list.json.sha256".to_string()].join("");

    let backup_already_processed_app_id_list_path = [get_cache_dir_path(), "/".to_string(), "backup_processed_app_id_list.json".to_string()].join("");
    let backup_already_processed_app_id_list_path_sha_256 = [get_cache_dir_path(), "/".to_string(), "backup_processed_app_id_list.json.sha256".to_string()].join("");

    let app_list_path = [get_cache_dir_path(), "/".to_string(), "ISteamApps-GetAppList-v2.json".to_string()].join("");
    let app_list_path_sha_256 = [get_cache_dir_path(), "/".to_string(), "ISteamApps-GetAppList-v2.json.sha256".to_string()].join("");

    let backup_app_list_path = [get_cache_dir_path(), "/".to_string(), "backup_ISteamApps-GetAppList-v2.json".to_string()].join("");
    let backup_app_list_path_sha_256 = [get_cache_dir_path(), "/".to_string(), "backup_ISteamApps-GetAppList-v2.json.sha256".to_string()].join("");


    let boxed_backup_restore = fs::copy(&backup_already_processed_app_id_list_path, &already_processed_app_id_list_path);
    if boxed_backup_restore.is_err() {
        println!("backup restore for processed already apps failed, exiting...");
        return;
    }

    let boxed_backup_restore_sha256 = fs::copy(&backup_already_processed_app_id_list_path_sha_256, &already_processed_app_id_list_path_sha_256);
    if boxed_backup_restore_sha256.is_err() {
        println!("backup sha256 restore for processed already apps failed, exiting...");
        return;
    }

    let boxed_backup_restore_app_list = fs::copy(&backup_app_list_path, &app_list_path);
    if boxed_backup_restore_app_list.is_err() {
        println!("backup applist restore failed, exiting...");
        return;
    }

    let boxed_backup_restore_app_list_sha256 = fs::copy(&backup_app_list_path_sha_256, &app_list_path_sha_256);
    if boxed_backup_restore_app_list_sha256.is_err() {
        println!("backup applist sha256 restore failed, exiting...");
        return;
    }
}

fn write_sha256(path: &String, data: &[u8]) -> String {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
        .unwrap();

    let sha256_from_list = digest(data);
    file.write_all(sha256_from_list.as_ref()).unwrap();
    sha256_from_list
}

fn get_steam_app_list() -> Vec<SteamApp> {
    let mut app_list : Vec<SteamApp>;
    let boxed_cached_app_list = get_cached_app_list();
    if boxed_cached_app_list.is_ok() {
        app_list = boxed_cached_app_list.unwrap();
    } else {
        app_list = get_app_list().unwrap();
    }

    app_list
}

fn setup_encryption() {
    let boxed_passphrase = get_or_create_passphrase();
}

fn get_or_create_passphrase() -> Result<String, String> {
    let passphrase_path = ".passphrase";

    let does_passphrase_exist = does_file_exist(passphrase_path);
    return if does_passphrase_exist {
        let passphrase = read_file(passphrase_path);
        Ok(passphrase)
    } else {
        create_file(passphrase_path);
        Ok("passphrase".to_string())
    }

}

fn create_file(path: &str)  {
    OpenOptions::new()
        .read(false)
        .write(false)
        .create(true)
        .truncate(false)
        .open(path)
        .unwrap();
}

fn does_file_exist(path: &str) -> bool {
    let file_exists = Path::new(path).is_file();
    file_exists
}

fn read_file(path: &str) -> String {
    let mut file_contents : String = "".to_string();

    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .truncate(false)
        .open(path)
        .unwrap();

    let boxed_read = file.read_to_string(&mut file_contents);
    if boxed_read.is_err() {
        println!("unable to read from file: {}", boxed_read.err().unwrap());
    }

    file_contents
}