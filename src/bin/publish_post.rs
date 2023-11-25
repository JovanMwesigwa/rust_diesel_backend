use std::env::args;

use diesel_demo::update_post;

fn main() {
    let id = args()
        .nth(1)
        .expect("publish post requires an id")
        .parse::<i32>()
        .expect("Invalid id");

    update_post(id);
}
