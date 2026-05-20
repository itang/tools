//! downloader
//!

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Client, Proxy};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;
use std::pin::Pin;
use std::time::Duration;

const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 1000;

/// Downloader
#[derive(Debug, Clone)]
pub struct Downloader {
    client: Client,
}

impl Downloader {
    /// Create a new Downloader with proxy support
    pub fn new() -> Self {
        let client = build_http_client();
        Self { client }
    }

    /// Download file from URL with retry and resume support
    pub async fn download(
        &self,
        url: &str,
        output_path: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let output_path = self.resolve_output_path(url, output_path)?;
        let output_path_obj = Path::new(&output_path);

        // Check existing file size for resume
        let mut resume_from = if output_path_obj.exists() {
            let metadata = std::fs::metadata(output_path_obj)?;
            let size = metadata.len();
            if size > 0 {
                println!("Found partial file: {} ({} bytes), resuming download...", output_path, size);
            }
            size
        } else {
            0
        };

        let mut last_error: Option<Box<dyn std::error::Error>> = None;

        for attempt in 1..=MAX_RETRIES {
            match self.try_download_with_resume(url, &output_path, resume_from).await {
                Ok(downloaded) => {
                    // Update resume_from for next retry if needed
                    resume_from = downloaded;
                    if resume_from >= downloaded {
                        println!("\nDownload completed: {}", output_path);
                        return Ok(());
                    }
                }
                Err(e) => {
                    // Check if we made any progress
                    let current_size = if output_path_obj.exists() {
                        std::fs::metadata(output_path_obj).map(|m| m.len()).unwrap_or(resume_from)
                    } else {
                        resume_from
                    };

                    eprintln!("\nDownload attempt {}/{} failed: {}", attempt, MAX_RETRIES, e);

                    // If we made progress, update resume_from
                    if current_size > resume_from {
                        println!("Progress saved: {} bytes downloaded", current_size);
                        resume_from = current_size;
                    }

                    last_error = Some(e);

                    if attempt < MAX_RETRIES {
                        eprintln!("Resuming from byte {} in {} seconds...", resume_from, RETRY_DELAY_MS / 1000);
                        tokio::time::sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| "All download attempts failed".into()))
    }

    /// Try to download file with resume support
    /// Returns the number of bytes downloaded (including resumed portion)
    fn try_download_with_resume<'a>(
        &'a self,
        url: &'a str,
        output_path: &'a str,
        resume_from: u64,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<u64, Box<dyn std::error::Error>>> + Send + 'a>> {
        Box::pin(async move {
            self.try_download_with_resume_inner(url, output_path, resume_from).await
        })
    }

    /// Inner implementation to avoid recursive async issues
    async fn try_download_with_resume_inner(
        &self,
        url: &str,
        output_path: &str,
        resume_from: u64,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let output_path_obj = Path::new(output_path);

        // Build request with Range header if resuming
        let mut request_builder = self.client.get(url);
        if resume_from > 0 {
            request_builder = request_builder.header("Range", format!("bytes={}-", resume_from));
        }

        let response = request_builder.send().await?.error_for_status()?;

        // Check if server supports range requests
        let status = response.status();
        let is_partial = status == reqwest::StatusCode::PARTIAL_CONTENT;

        if resume_from > 0 && !is_partial {
            println!("Server does not support resume, restarting download from beginning");
            // Delete partial file and restart
            if output_path_obj.exists() {
                std::fs::remove_file(output_path_obj)?;
            }
            // Call without resume (using Box::pin to avoid infinite sized future)
            return self.try_download_with_resume(url, output_path, 0).await;
        }

        let total_size = if is_partial {
            // For 206 response, get total size from Content-Range header
            response
                .headers()
                .get("Content-Range")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| {
                    // Parse "bytes start-end/total" format
                    s.split('/').last().and_then(|total| total.parse::<u64>().ok())
                })
                .unwrap_or(0)
        } else {
            response.content_length().unwrap_or(0)
        };

        let progress_bar = if total_size > 0 {
            let pb = ProgressBar::new(total_size);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
                    )?
                    .progress_chars("#>-"),
            );
            pb.set_position(resume_from);
            pb
        } else {
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} [{elapsed_precise}] Downloading... {bytes}")?,
            );
            pb
        };

        // Open file for writing (create new or append to existing)
        let mut file = if resume_from > 0 && is_partial {
            let mut f = OpenOptions::new().write(true).open(output_path_obj)?;
            f.seek(SeekFrom::End(0))?;
            f
        } else {
            File::create(output_path_obj)?
        };

        let mut stream = response.bytes_stream();
        let mut downloaded = resume_from;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk)?;
            downloaded += chunk.len() as u64;
            progress_bar.set_position(downloaded);
        }

        progress_bar.finish_and_clear();
        Ok(downloaded)
    }

    /// Resolve output path from URL or user-provided path
    fn resolve_output_path(
        &self,
        url: &str,
        output_path: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(path) = output_path {
            return Ok(path.to_string());
        }

        let url_path = reqwest::Url::parse(url)?;
        let filename = url_path
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or("download");

        Ok(filename.to_string())
    }
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}

/// Build HTTP client with proxy support
fn build_http_client() -> Client {
    let mut client_builder = Client::builder()
        .timeout(Duration::from_secs(300))
        .connect_timeout(Duration::from_secs(30));

    if let Ok(http_proxy) = env::var("HTTP_PROXY") {
        match Proxy::http(&http_proxy) {
            Ok(proxy) => {
                client_builder = client_builder.proxy(proxy);
            }
            Err(e) => {
                eprintln!("Warning: Failed to parse HTTP_PROXY '{}': {}", http_proxy, e);
            }
        }
    }

    if let Ok(https_proxy) = env::var("HTTPS_PROXY") {
        match Proxy::https(&https_proxy) {
            Ok(proxy) => {
                client_builder = client_builder.proxy(proxy);
            }
            Err(e) => {
                eprintln!("Warning: Failed to parse HTTPS_PROXY '{}': {}", https_proxy, e);
            }
        }
    }

    if let Ok(all_proxy) = env::var("ALL_PROXY") {
        match Proxy::all(&all_proxy) {
            Ok(proxy) => {
                client_builder = client_builder.proxy(proxy);
            }
            Err(e) => {
                eprintln!("Warning: Failed to parse ALL_PROXY '{}': {}", all_proxy, e);
            }
        }
    }

    client_builder.build().expect("Failed to build HTTP client")
}
