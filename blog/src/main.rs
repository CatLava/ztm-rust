use blog::Post;

// Post will ahve a state that goes through various states before publish
// this is verified with assert_eq!
fn main() {
    let mut post = Post::new();

    post.add_text("I ate salad for lunch");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate salad for lunch", post.content());
}
