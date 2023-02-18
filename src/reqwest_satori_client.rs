use crate::satori_client::SatoriClient;

use reqwest_cookie_store::CookieStoreMutex;
use std::sync::Arc;

pub struct ReqwestSatoriClient {
    base_url: reqwest::Url,
    domain: String,
    token_name: String,
    client: reqwest::blocking::Client,
    cookie_store: Arc<CookieStoreMutex>,
}

impl ReqwestSatoriClient {
    pub fn new(base_url: &str, token_name: &str) -> Self {
        let base_url = reqwest::Url::parse(base_url).unwrap();
        let domain = base_url.domain().unwrap().to_string();

        let cookie_store = reqwest_cookie_store::CookieStore::default();
        let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
        let cookie_store = std::sync::Arc::new(cookie_store);

        let client = reqwest::blocking::Client::builder()
            .cookie_provider(std::sync::Arc::clone(&cookie_store))
            .build()
            .unwrap();

        Self {
            base_url,
            domain,
            token_name: token_name.to_string(),
            client,
            cookie_store,
        }
    }


    pub fn get_token(&self) -> Option<String> {
        self.cookie_store
            .lock()
            .unwrap()
            .get(&self.domain, "/", &self.token_name)
            .map(|cookie| cookie.value().to_string())
    }

    pub fn set_token(&self, token: &str) {
        let cookie = cookie::Cookie::build(&self.token_name, token)
            .domain(&self.domain)
            .path("/")
            .secure(true)
            .http_only(true)
            .finish();
        self.cookie_store
            .lock()
            .unwrap()
            .insert_raw(&cookie, &self.base_url)
            .unwrap();
    }

    pub fn log_in(&self, login: &str, password: &str) -> reqwest::Result<()> {
        self.client
            .post(self.get_url("login"))
            .form(&[("login", login), ("password", password)])
            .send()
            .unwrap();
        Ok(())
    }

    pub fn get_url(&self, path: &str) -> reqwest::Url {
        self.base_url.join(path).unwrap()
    }

    pub fn do_get(&self, path: &str) -> reqwest::blocking::Response {
        self.client.get(self.get_url(path)).send().unwrap()
    }

    pub fn do_post(&self, path: &str, data: &[(&str, &str)]) -> reqwest::blocking::Response {
        self.client
            .post(self.get_url(path))
            .form(data)
            .send()
            .unwrap()
    }

    pub fn do_multipart_post(
        &self,
        path: &str,
        form: reqwest::blocking::multipart::Form,
    ) -> reqwest::blocking::Response {
        self.client
            .post(self.get_url(path))
            .multipart(form)
            .send()
            .unwrap()
    }
}

impl SatoriClient for ReqwestSatoriClient {
    fn get_token(&self) -> Option<String> {
        self.get_token()
    }

    fn set_token(&self, token: &str) {
        self.set_token(token)
    }

    fn get_url(&self, path: &str) -> String {
        self.get_url(path).to_string()
    }

    fn get(&self, path: &str) -> Option<String> {
        let response = self.do_get(path);
        if response.status().is_success() {
            Some(response.text().unwrap())
        } else {
            None
        }
    }

    fn post(&self, path: &str, data: &[(&str, &str)]) -> Option<String> {
        let response = self.do_post(path, data);
        if response.status().is_success() {
            Some(response.text().unwrap())
        } else {
            None
        }
    }

    fn submit_file(&self, path: &str, file_name: &str, file_path: &str) -> Option<String> {
        let form = reqwest::blocking::multipart::Form::new()
            .file(file_name.to_string(), file_path)
            .unwrap();
        let response = self.do_multipart_post(path, form);
        if response.status().is_success() {
            Some(response.text().unwrap())
        } else {
            None
        }
    }
}