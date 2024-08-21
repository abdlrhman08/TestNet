use axum::response::Html;
use std::{fs::File, io::Read};

pub(crate) fn html_response(filepath: &str) -> Html<String> {
    let html_file = File::open(format!("dist/{}", filepath));
    let mut html_string = String::new();
    let _ = match html_file {
        Err(e) => {
            println!("Error opening HTML file, {}", e);
            return Html("Not Found".to_string());
        }
        Ok(mut f) => f.read_to_string(&mut html_string),
    };

    Html(html_string)
}
