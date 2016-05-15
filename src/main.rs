extern crate hyper;
extern crate liquid;

use std::io::prelude::*;
use std::fs::{File, metadata};
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Server, Response, Request};
use hyper::uri::RequestUri;
use liquid::{Renderable, Context, Value};


fn handler(req: Request, mut res: Response) {
    if let RequestUri::AbsolutePath(url) = req.uri {
        println!("GET {}", &url);
        let song = format!("music{}.mp3", &url);
        let md = metadata(song.clone());
        match md {
            Ok(m) => {
                if m.is_file() { // Single Song
                    println!("{} is a song", song.clone());
                    // Read file contents into string
                    let mut file = File::open("static/song.html").unwrap();
                    let mut html = String::new();
                    file.read_to_string(&mut html).unwrap();

                    // Create template
                    let template = liquid::parse(&html, Default::default()).unwrap();
                    let mut context = Context::new();
                    context.set_val("song", Value::Str(url[1..].to_string()));

                    // Render template and send it
                    let body = template.render(&mut context).unwrap().unwrap();
                    res.headers_mut().set(ContentLength(body.len() as u64));
                    res.headers_mut().set(ContentType::html());
                    let mut res = res.start().unwrap();
                    res.write_all(body.as_bytes()).unwrap();
                }
            },
            Err(_) => {
                let file_name = url[1..].to_string();
                println!("{} isn't a song", &file_name);
                let url_md = metadata(&file_name);
                match url_md {
                    Ok(m) => {
                        print!("Metadata exists for {}. ", &file_name);
                        if m.is_file() {
                            // Put file bytes into buffer
                            let mut file = File::open(&file_name).unwrap();
                            let mut buffer = Vec::new();
                            file.read_to_end(&mut buffer).unwrap();

                            // Send file
                            println!("{} is a file of {} bytes", &file_name, buffer.len());
                            res.headers_mut().set(ContentLength(buffer.len() as u64));
                            let mut res = res.start().unwrap();
                            for byte in &buffer {
                                res.write(&[*byte]).unwrap();
                            }
                            res.flush().unwrap();
                        } else if m.is_dir() {
                            println!("{} is a directory!", &file_name);
                        }
                    },
                    Err(_) => {}
                }
            }
        }

    }
}

fn main() {
    Server::http("127.0.0.1:666").unwrap().handle(handler).unwrap();
}