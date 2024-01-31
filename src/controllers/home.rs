use actix_web::HttpResponse;
use actix_web::{get, web};
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
pub struct NavbarContent {
    pub home: String,
    pub blog: String,
    pub about: String, 
    pub services: String, 
    pub works: String,
    pub contact: String,
}


#[derive(TemplateOnce)]
#[template(path = "index.html")]
pub struct HomeView {
   pub static_content: Value,
   pub language: String,
   pub navbar_content: NavbarContent
}

pub fn read_static_file() -> String {
    let json = fs::read_to_string("src/language.json").unwrap();
    json
}

#[get("/{lang}")]
async fn index(path: web::Path<(String)>) -> HttpResponse {
    let (lang) = path.into_inner();
    let language: String;
    if lang == "en" {
        language = String::from("english");
    }else{
        language = String::from("indonesia");
    }
    println!("{} ", language);
    let json_content = read_static_file();
    let static_content: Value = serde_json::from_str(json_content.clone().as_str()).unwrap();
    // println!("static {:?} ", static_content);
    println!("{:?} ", static_content["hire"][&language]);
    let navbar_content = static_content["navbar"][&language].clone();
    let view = HomeView {
        static_content: static_content,
        language ,
        navbar_content: serde_json::from_value(navbar_content).unwrap()
    };
    HttpResponse::Ok().body(view.render_once().unwrap())
}
