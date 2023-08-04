use std::collections::hash_map::DefaultHasher;
use axum::{extract::Path, routing::get, http::StatusCode, Router, Extension};
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use std::num::NonZeroUsize;
use std::sync::Arc;
use axum::http::{HeaderMap, HeaderValue};
use bytes::Bytes;
use image::ImageOutputFormat;
use lru::{LruCache};
use photon_rs::monochrome::sepia;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tracing::{info, instrument};

mod pb;
mod engine;

use pb::*;
use crate::engine::{Engine, Photon};

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

//Note1: Arc<T> 原子引用计数的智能指针，能保障线程安全
type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cache: Cache = Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(1024).unwrap())));

    let app = Router::new()
        .route("/image/:spec/:url", get(generate))
        .layer(ServiceBuilder::new().layer(AddExtensionLayer::new(cache)).into_inner());

    let addr = "127.0.0.1:8080".parse().unwrap();

    tracing::debug!("listening on {}",addr);

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
//NOTE1: |_| clojure
//NOTE2: map_err Maps a Result<T, E> to Result<T, F> by applying a function to a contained Err value, leaving an Ok value untouched.
// This function can be used to pass through a successful result while handling an error.
async fn generate(Path(Params { spec, url }): Path<Params>, Extension(cache): Extension<Cache>) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec.as_str().try_into().map_err(|_| StatusCode::BAD_REQUEST)?;
    let data = retrieve_image(&url, cache).await.map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut engine: Photon = data.try_into().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    engine.apply(&spec.specs);
    let image = engine.generate(ImageOutputFormat::Jpeg(85));
    info!("Finished process: image size {}",image.len());


    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("image/jpeg"));
    Ok((headers, image))
}

#[instrument(level = "info", skip(cache))]
async fn retrieve_image(url: &str, cache: Cache) -> Result<Bytes, Box<dyn std::error::Error>> {
    let mut hasher = DefaultHasher::default();
    url.hash(&mut hasher);
    let key = hasher.finish();
    let g = &mut cache.lock().await;
    let data = match g.get(&key) {
        None => {
            info!("Retrieve url");
            let resp = reqwest::get(url).await?;
            let data = resp.bytes().await?;
            g.put(key, data.clone());
            data
        }
        Some(v) => {
            info!("match cahe{}",key);
            v.to_owned()
        }
    };

    Ok(data)
}