use std::env::args;

use diesel_demo::get_post;

fn main() {
    let id = args()
        .nth(1)
        .expect("Get post needs a post_id")
        .parse::<i32>()
        .expect("Invalid ID");

    let post = get_post(id);

    // match post {
    //     Ok(post) => post,
    //     None => println!("Post not found"),
    // }

    println!("The wanted post is: {:?} ", post.unwrap().title,);
}
