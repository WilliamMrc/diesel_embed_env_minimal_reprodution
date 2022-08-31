#[macro_use]
extern crate diesel_migrations;

//embed_migrations!("migrations");
embed_migrations!(env!("MIGRATIONS_DIR"));

fn main() {
    println!("Hello, world! \n the migration dir is {}", env!("MIGRATIONS_DIR"));
}