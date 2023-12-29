#[macro_use] extern crate rocket;
// use rocket::form::Form;



use std::path::Path;
// use rocket::fs::FileName; 
// use std::fs::read_to_string;
use rocket::fs::{relative, TempFile, FileServer};
// use rocket::response::content::RawHtml;
use rocket::form::Form;
use rocket::response::Redirect;
#[derive(FromForm)]
struct UploadUser<'f>{
    
    maquina: String,
    input_file: TempFile<'f>,

}




#[post("/input", format = "multipart/form-data", data = "<form_input>")]
async fn form(mut  form_input : Form<UploadUser<'_>>) -> Redirect {
    let mut path: String = "/home/murylo1050/teste/fileSite/".to_string();
    path.push_str(form_input.input_file.name().expect("deu merda no arquivo")); 
    path.push_str(".txt");
    
    let maquina = &form_input.maquina;
    println!("Raw name {}", maquina);
    form_input.input_file.move_copy_to(Path::new(&path)).await.ok();
   
    Redirect::to(uri!("/"))
}
#[get("/")]
fn index() -> Redirect/* RawHtml<String> */{
     Redirect::to(uri!("/index.html"))
}

#[launch]
fn rocket()-> _{
    rocket::build().mount("/", routes![index,form]).mount("/", FileServer::from(relative!("static")))}
