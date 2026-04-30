use blog::Post;

fn main() {
    let mut post = Post::new();
    post.content = String::from("hello");
    println!("{0}", post.content);
}
