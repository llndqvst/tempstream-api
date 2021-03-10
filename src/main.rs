pub mod util;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use actix_web::{post, get, web, App, HttpServer, Result};
use actix_cors::Cors;
use serde_derive::{Serialize, Deserialize};
use deadpool_redis::{Config as RedisConfig, Pool};
use util::{redis_util, config_util};

#[derive(Serialize, Deserialize)]
struct StreamObj {
    success: i32,
    id: String,
    key: String,
    hls_url: String,
}

#[derive(Serialize, Deserialize)]
struct StreamRTMP {
    success: i32,
    id: String,
    key: String,
    rtmp_url: String,
}

#[derive(Serialize, Deserialize)]
struct Publish {
    action: String,
    stream: String,
    param: String,
}

#[get("/genstream")]
async fn genstream(pool: web::Data<Pool>,
                   config: web::Data<config_util::AppConfig>)
                   -> Result<web::Json<StreamRTMP>> {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let slice_id = &rand_string[..6];
    let slice_key = &rand_string[6..];

    redis_util::set_str(&pool, slice_id, slice_key).await;

    Ok(web::Json(StreamRTMP {
        success: 1,
        id: slice_id.to_string(),
        key: format!("{}?{}",slice_id.to_string(),slice_key.to_string()),
        rtmp_url: format!("{}", config.srs_rtmp),
    }))
}

#[get("/get_stream/{id}")]
async fn stream(pool: web::Data<Pool>,
                config: web::Data<config_util::AppConfig>,
                web::Path(id): web::Path<String>) -> Result<web::Json<StreamObj>> {
    let (key, status): (String, i32) = match redis_util::get_str(&pool, &id).await {
        Some(v) => (v, 0),
        None => (String::from("empty"), 1)
    };

    Ok(web::Json(StreamObj {
        success: status,
        id: id.clone(),
        key: key,
        hls_url: format!("{}/{}.m3u8", config.srs_web, id),
    }))
}

#[post("/verify")]
async fn verify(pool: web::Data<Pool>, publish: web::Json<Publish>) -> Result<String> {
    let key1: &str = &publish.param[1..];
    let id: &str = &publish.stream;

    let key2: String = match redis_util::get_str(&pool, &id).await {
        Some(v) => v,
        None => String::from("1")
    };
    let status = if key1 == key2 {
        "0"
    } else {
        "1"
    };

    Ok(String::from(status))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_config = config_util::get_app_config();
    let server_url = app_config.bind_address.clone();

    HttpServer::new(move|| {
        let redis_config = RedisConfig { url: Some(app_config.redis_url.clone()), pool: None };
        let redis_pool = redis_config.create_pool().unwrap();

        let cors = Cors::permissive();

        App::new()
            .data(redis_pool)
            .data(app_config.clone())
            .wrap(cors)
            .service(genstream)
            .service(stream)
            .service(verify)
    })
        .bind(server_url)?
        .run()
        .await
}
