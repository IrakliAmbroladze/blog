pub struct Post {
    pub content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            content: String::from(""),
        }
    }
}
