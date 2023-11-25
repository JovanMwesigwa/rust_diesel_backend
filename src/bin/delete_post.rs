use std::env::args;

use diesel_demo::delete_post;

fn main() {
    let id = args()
        .nth(1)
        .expect("Post id id needed to delete a record")
        .parse::<i32>()
        .expect("Invalid ID");

    delete_post(id);
}
