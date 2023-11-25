use diesel::OptionalExtension;
use diesel::{
    Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use dotenvy::dotenv;
use models::Posts;

pub mod models;
pub mod schema;

use std::env;

use crate::models::NewPost;

pub fn database_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE URL was not set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to database {}", database_url))
}

// This gets all the available published posts
pub fn show_posts() {
    use schema::posts::dsl::*;

    let connection = &mut database_connection();
    let results = posts
        .filter(published.eq(true))
        .select(Posts::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}

// This creates a new post to the database
pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> Posts {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    conn.transaction(|connection| {
        diesel::insert_into(posts::table)
            .values(&new_post)
            .execute(connection)?;

        posts::table
            .order(posts::id.desc())
            .select(Posts::as_select())
            .first(connection)
    })
    .expect("Error while saving post")
}

// This updates an existing post
pub fn update_post(post_id: i32) {
    use schema::posts::dsl::{posts, published};

    let mut connection = database_connection();

    connection
        .transaction(|conn| {
            // let post = posts.find(post_id).select(Posts::as_select()).first(conn)?;

            diesel::update(posts.find(post_id))
                .set(published.eq(true))
                .execute(conn)?;

            Ok(())
        })
        .unwrap_or_else(|_: diesel::result::Error| panic!("Unable to find post"));
}

// This fetches a single post
pub fn get_post(post_id: i32) -> Option<Posts> {
    use schema::posts::dsl::posts;

    let mut connection = database_connection();

    let post = posts
        .find(post_id)
        .select(Posts::as_select())
        .first(&mut connection)
        .optional();

    match post {
        Ok(post) => post,
        Err(_) => None,
    }
}

// Delete a user post
pub fn delete_post(post_id: i32) {
    use crate::schema::posts::id;
    use schema::posts::dsl::posts;

    let mut connection = database_connection();

    let _ = diesel::delete(posts.filter(id.eq(post_id))).execute(&mut connection);

    println!("Post deleted");
}
