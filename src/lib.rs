mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;
use std::env;
//use std::fs::File;
use serde::{Deserialize, Serialize};
use serde_json::{Value};

//use rusqlite::{params, Connection};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
//    println!("hello-wasm3");
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn test1(msg: &str) {
    console::log_1(&JsValue::from_str( msg ));
}

#[derive(Debug)]
pub struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

/*
pub fn get_content(filename: &str ) -> String{
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return contents;
}
#[wasm_bindgen]
pub fn wasm_get_json(filename: &str) {
    console::log_1(&JsValue::from_str( filename ));
    let json = get_content( &filename );
}
*/
#[wasm_bindgen]
pub fn wasm_task_disp(id_name: &str, json: &str) -> Result<(), JsValue>{
    let mut s_elm : String = String::new();
    let deserialized: Vec<TaskItem> = serde_json::from_str(json).unwrap();
    for row in &deserialized {
        console::log_1(&JsValue::from_str( &row.title ));
//        console::log_1(&JsValue::from_str( &s ));
    }

    Ok(())
}
#[derive(Serialize, Deserialize, Debug)]
struct TaskItem {
    id: i64,
    title: String,
    content: String,
}

#[wasm_bindgen]
pub fn wasm_object_array(id_name: &str, val: &JsValue) -> JsValue {
    let deserialized: Vec<TaskItem> = val.into_serde().unwrap();
    let mut s_elm : String = String::new();
    for row in &deserialized {
//        console::log_1(&JsValue::from_str( &row.title ));
        let s = format!("
        <div class='div_post_row_wrap'>
            <a href='/tasks/show/{}'>
                <h3 class='h3_title'>{}</h3>
            </a>        
            <p class='p_title mb-0'><span>ID :{}</span>
            </p>
            <hr class='hr_ex1 mt-1 mb-1'>    
        </div>" , &row.id, &row.title , &row.id );
        s_elm.push_str( &s );    
    }    

    JsValue::from_str( &s_elm )
}
#[wasm_bindgen]
pub fn wasm_test_array(id_name: &str, val: &JsValue) -> JsValue {
    let deserialized: Vec<TaskItem> = val.into_serde().unwrap();
    let mut s_elm : String = String::new();
    for row in &deserialized {
//        console::log_1(&JsValue::from_str( &row.title ));
        let s = format!("
        <div class='div_post_row_wrap'>
            <p class='p_title mb-0'>{} , <span>ID :{}</span>
            </p>
            <hr class='hr_ex1 mt-1 mb-1'>    
        </div>" , &row.title , &row.id );
        s_elm.push_str( &s );    
    }    

    JsValue::from_str( &s_elm )
}
#[wasm_bindgen]
pub fn wasm_test_convert(id_name: &str, val: &JsValue) -> JsValue {
    let deserialized: Vec<TaskItem> = val.into_serde().unwrap();
    let mut s_elm : String = String::new();
    
    JsValue::from_str( &s_elm )
}