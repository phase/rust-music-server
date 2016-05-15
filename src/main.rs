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
        let song = format!("music{}.mp3", &url);
        let md = metadata(song);
        match md {
            Ok(m) => {
                if m.is_file() { // Single Song
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
                else if m.is_dir() { // Playlist
                    res.send(b"Playlists unimplemented").unwrap();
                }
                else {
                   res.send(b"Song not found").unwrap();
                }
            },
            Err(_) => res.send(b"Song not found").unwrap()
        }

    }
}

fn main() {
    Server::http("127.0.0.1:666").unwrap().handle(handler).unwrap();
}