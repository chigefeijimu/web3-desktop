// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use rusqlite::{Connection, params, Result};
use serde::__private::de::IdentifierDeserializer;
use serde::Serialize;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

const DB_PATH: &str = "./web3.db";
const CREATE_TABLE_SQL: &str = "
CREATE TABLE IF NOT EXISTS web3_wallet (
    name TEXT,
    public_key TEXT NOT NULL PRIMARY KEY,
    private_key TEXT NOT NULL
);";
const RES_SUCCESS: &str = "创建钱包完成！";
const RES_ERR: &str = "创建钱包失败！";

#[tauri::command]
fn greet(name: &str) -> String {
    loop{
        let keypair: Keypair = Keypair::new();
        let public_key = keypair.pubkey().to_string();
        let private_key = keypair.to_base58_string();

        if public_key.starts_with(name) {
            let wallet = Wallet {
                public_key,
                private_key
            };
            return save_db(&wallet)
        }
    }
}

#[derive(Serialize)]
struct Wallet {
    public_key: String,
    private_key: String,
}

fn save_file(wallet: &Wallet) -> String {
    //创建本地文件
    let file_path = String::from("./wallet.json");
    println!("{}", file_path);
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("无法创建文件: {}", e);
            return RES_ERR.to_string();
        }
    };

    let json_data: String = match serde_json::to_string_pretty(&wallet) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("无法序列化JSON数据，原因：{}", e);
            return RES_ERR.to_string();
        }
    };
    //像文件中写入数据
    match file.write_all(json_data.as_bytes()) {
        Ok(_) => println!("创建钱包并写入文件成功！"),
        Err(e) => {
            eprintln!("写入序列化JSON数据失败，原因：{}", e);
            return RES_ERR.to_string();
        }
    };
    return RES_SUCCESS.to_string()
}

fn save_db(wallet: &Wallet) -> String {
    let insert_sql: &str = "INSERT INTO web3_wallet(name, public_key, private_key) VALUES(?1, ?2, ?3)";
    let conn: Connection = match Connection::open(DB_PATH) {
        Ok(connection) => connection,
        Err(e) => {
            eprintln!("数据库插入钱包数据失败， 原因：{}", e);
            return RES_ERR.to_string()
        }
    };
    match conn.execute(insert_sql, params!["", wallet.public_key, wallet.private_key]) {
        Ok(count) => count,
        Err(e) => {
            eprintln!("数据库插入钱包数据失败，原因：{}", e);
            return RES_ERR.to_string()
        }
    };
    return RES_SUCCESS.to_string()
}

fn init_db() -> Result<Arc<Mutex<Connection>>> {
    let conn: Connection = Connection::open(DB_PATH)?;
    conn.execute(CREATE_TABLE_SQL, params![])?;
    Ok(Arc::new(Mutex::new(conn)))
}

fn main() {
    tauri::Builder::default()
        .manage(init_db())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
