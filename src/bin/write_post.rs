use diesel_demo::{create_post, database_connection};

fn main() {
    let mut connection = database_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("Enter title: ");
    std::io::stdin().read_line(&mut title).unwrap();
    let trimmed_title = title.trim_end();

    println!("Enter body: ");
    std::io::stdin().read_line(&mut body).unwrap();
    let trimmed_body = body.trim_end();

    let post = create_post(&mut connection, trimmed_title, trimmed_body);

    println!("\nSaved draft {} with id {}", post.title, post.id);
}
