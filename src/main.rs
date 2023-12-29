#[macro_use] extern crate rocket;
// use rocket::form::Form;


use std::path::Path;
// use rocket::fs::FileName; 
use std::fs::read_to_string;
use rocket::fs::{relative, TempFile};
use rocket::response::content::RawHtml;
use rocket::form::Form;
// use rocket::response::Redirect;
#[post("/submit", format = "multipart/form-data", data = "<file>")]
async fn form(mut file : Form<TempFile<'_>>) -> std::io::Result<()>{
    let path = Path::new("~");
    let mut pathbuf = path.join("teste");
    pathbuf.push("fileSite");
    pathbuf.push(file.name().expect("deu merda no arquivo"));
    pathbuf.set_extension("txt");
    // println!("\n\n\n\n\n Path: {}", pathbuf.display());

    file.copy_to(pathbuf).await.ok();
    // Redirect::to("/");
    Ok(())
                                                                                                                                                                                                                                                                                                  
}
#[get("/")]
fn index() -> RawHtml<String>{

    RawHtml(read_to_string(relative!("src/static/index.html")).expect("transa")) 
}

#[launch]
fn rocket()-> _{
    rocket::build().mount("/", routes![index,form])}
