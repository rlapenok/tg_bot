use std::fs::{read_to_string, File};

use google_authenticator::GoogleAuthenticator;
use log::{info, warn};

//Структура MyAuthenticator, которая будет проводить проверку кода из приложения Google Authenticator
pub struct MyAuthenticator {
    auth: GoogleAuthenticator,
    key: Option<String>,
}

impl MyAuthenticator {
    //Создание или новой структуры или разрузка уже существующей
    pub fn new() -> Self {
        let mut auth = Self {
            auth: GoogleAuthenticator::new(),
            key: None,
        };
        match File::open("key.txt") {
            Ok(_) => {
                info!("key.txt exist");
                info!("Create new MyAuthenticator from key.txt");
                let key = read_to_string("key.txt").unwrap().trim().to_owned();
                if key.is_empty() {
                } else {
                    auth.key.replace(key.clone());
                }
                log::info!("{:?}", auth.key);
            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    info!("key.txt does not exist");
                }
                _ => {}
            },
        }
        auth
    }
    //Существует ли ключ в структуре, при ее создании
    pub fn exist_key(&self) -> bool {
        match &self.key {
            Some(_) => {
                info!("Key exist in key.txt");
                true
            }
            None => {
                warn!("Key does not exist in key.txt");
                false
            }
        }
    }
    //Создание ключа
    pub fn create_keys(&mut self) -> String {
        info!("Create key for Google Authenticator");
        let key = self.auth.create_secret(32);
        self.key.replace(key.clone());
        key
    }
    //Проверка введенного кода из приложения Google Authenticator
    pub fn verify_code(&self, code: &str) -> bool {
        match self
            .auth
            .verify_code(self.key.as_ref().unwrap(), code, 10, 0)
        {
            true => {
                log::info!("The entered code is correct");
                true
            }
            false => {
                log::error!("The entered code is incorrect");

                false
            }
        }
    }
}
