#![feature(try_blocks)]

#[derive(Default, Debug)]
struct Response {
    bookmarks: String,
    continue_after: String,
}

fn format_response(page: Result<String, String>) -> Result<Response, String> {
    try {
        Response {
            bookmarks: String::new(),
            continue_after: page?,
            ..Default::default()
        }
    }
}

pub fn main() {}
