use core::fmt;
use std::sync::Arc;

use reqwest::{Client, StatusCode, Url};
use serde_json::Value;
use tokio::task::JoinHandle;

pub enum MirrorType {
    Search,
    Download,
}

#[derive(Debug, Clone)]
pub struct Mirror {
    pub host_url: Url,
    pub search_url: Option<Url>,
    pub search_url_fiction: Option<Url>,
    pub download_url: Option<Url>,
    pub download_url_fiction: Option<Url>,
    pub download_pattern: Option<String>,
    pub sync_url: Option<Url>,
    pub cover_pattern: Option<String>,
}

impl fmt::Display for Mirror {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.host_url)
    }
}

impl Mirror {
    pub async fn check_connection(&self, client: &Client) -> Result<(), StatusCode> {
        let response = client.get(self.host_url.as_str()).send().await;
        match response {
            Ok(res) => {
                let text = res.text().await.unwrap();
                if text.contains("Block Page") {
                    Err(StatusCode::FORBIDDEN)
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(e.status().unwrap()),
        }
    }
}

pub struct MirrorList {
    pub search_mirrors: Vec<Mirror>,
    pub download_mirrors: Vec<Mirror>,
}

impl MirrorList {
    pub fn parse_mirrors(json: &str) -> MirrorList {
        let mut search_mirrors: Vec<Mirror> = Vec::new();
        let mut download_mirrors: Vec<Mirror> = Vec::new();

        let map: Value = serde_json::from_str(json).unwrap();

        map.as_object().unwrap().iter().for_each(|(_k, v)| {
            let search_url = v
                .get("SearchUrl")
                .map(|v| Url::parse(v.as_str().unwrap()).unwrap());
            let search_url_fiction = v
                .get("FictionSearchUrl")
                .map(|v| Url::parse(v.as_str().unwrap()).unwrap());
            let host_url = v
                .get("Host")
                .map(|v| Url::parse(v.as_str().unwrap()).unwrap());
            let download_url = v
                .get("NonFictionDownloadUrl")
                .map(|v| Url::parse(&v.as_str().unwrap().replace("{md5}", "")).unwrap());
            let download_url_fiction = v
                .get("FictionDownloadUrl")
                .map(|v| Url::parse(&v.as_str().unwrap().replace("{md5}", "")).unwrap());
            let download_pattern = v
                .get("NonFictionDownloadUrl")
                .map(|v| v.as_str().unwrap().to_owned());
            let sync_url = v
                .get("NonFictionSynchronizationUrl")
                .map(|v| Url::parse(v.as_str().unwrap()).unwrap());
            let cover_pattern = v
                .get("NonFictionCoverUrl")
                .map(|v| String::from(v.as_str().unwrap()));
            if let Some(..) = host_url {
                if search_url.is_some() {
                    search_mirrors.push(Mirror {
                        host_url: host_url.unwrap(),
                        search_url,
                        search_url_fiction,
                        download_url,
                        download_url_fiction,
                        download_pattern,
                        sync_url,
                        cover_pattern,
                    })
                } else if download_url.is_some() {
                    download_mirrors.push(Mirror {
                        host_url: host_url.unwrap(),
                        search_url,
                        search_url_fiction,
                        download_url,
                        download_url_fiction,
                        download_pattern,
                        sync_url,
                        cover_pattern,
                    })
                }
            }
        });

        MirrorList {
            search_mirrors,
            download_mirrors,
        }
    }

    pub async fn get_working_mirrors(
        &self,
        mirror_type: MirrorType,
        client: Arc<Client>,
    ) -> Result<Vec<Mirror>, String> {
        let mut working_mirrors = Vec::new();
        let mut forbidden_mirrors = Vec::new();

        if let MirrorType::Search = mirror_type {
            for mirror in self.search_mirrors.iter() {
                match mirror.check_connection(&client).await {
                    Ok(_) => working_mirrors.push(mirror.clone()),
                    Err(e) => {
                        if e == StatusCode::FORBIDDEN {
                            forbidden_mirrors.push(mirror.clone());
                        }
                        continue;
                    }
                };
            }
        } else {
            for mirror in self.download_mirrors.iter() {
                match mirror.check_connection(&client).await {
                    Ok(_) => working_mirrors.push(mirror.clone()),
                    Err(e) => {
                        if e == StatusCode::FORBIDDEN {
                            forbidden_mirrors.push(mirror.clone());
                        }
                        continue;
                    }
                };
            }
        }

        if !forbidden_mirrors.is_empty() {
            let forbidden_urls: Vec<String> = forbidden_mirrors
                .iter()
                .map(|mirror| mirror.host_url.to_string())
                .collect();
            Err(format!(
                "The following mirrors were blocked: {}",
                forbidden_urls.join(", ")
            ))
        } else if working_mirrors.is_empty() {
            Err("Couldn't reach mirrors".to_string())
        } else {
            Ok(working_mirrors)
        }
    }

    pub async fn spawn_get_working_mirrors_tasks(
        self: Arc<Self>,
        client: &Arc<Client>,
    ) -> Vec<JoinHandle<Result<Vec<Mirror>, String>>> {
        let search_mirrors_handle = tokio::spawn({
            let self_clone = Arc::clone(&self);
            let client_clone = Arc::clone(&client);
            async move {
                self_clone
                    .get_working_mirrors(MirrorType::Search, client_clone)
                    .await
            }
        });

        let download_mirrors_handle = tokio::spawn({
            let self_clone = Arc::clone(&self);
            let client_clone = Arc::clone(&client);
            async move {
                self_clone
                    .get_working_mirrors(MirrorType::Download, client_clone)
                    .await
            }
        });

        vec![search_mirrors_handle, download_mirrors_handle]
    }
}
