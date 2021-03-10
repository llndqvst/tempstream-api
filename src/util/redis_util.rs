use actix_web::web;
use serde::{Deserialize,Serialize};
use deadpool_redis::{cmd, Pool};
use std::env;

pub fn redis_uri() -> String {
    match env::var("REDIS_URL") {
        Ok(s) if !s.is_empty() => s,
        _ => String::from(String::from("redis://127.0.0.1:6379"))
    }
}

pub async fn get_str(redis: &web::Data<Pool>, name: &str) -> Option<String> {
    let mut r = redis.get().await.unwrap();
    let v = cmd("GET").arg(&[name]).query_async::<String>(&mut r).await;
    match v{
        Ok(s)=>{ Some(s)}
        Err(_e)=>{None}
    }
}

pub async fn set_str(redis: &web::Data<Pool>, name: &str, value: &str) {
    let mut r = redis.get().await.unwrap();
    cmd("SET").arg(&[name, value]).execute_async(&mut r).await.unwrap();
}

#[derive(Deserialize,Serialize)]
pub struct Success{
    code:i32,
    msg:String,
}

//Make a return. Although http code is recommended for the code parameter, some front ends prefer to take it from the return value
//, so we have to take care of it
pub fn msg_response(code:i32,msg:&str)->Success{
    Success{
        code:code,
        msg:String::from(msg)
    }
}
