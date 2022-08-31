# diesel_embed_env_minimal_reproduction
A minimal reproduction of an issue with diesel embed_migrations macro
https://github.com/diesel-rs/diesel/discussions/3295

`embed_migrations!(env!("MIGRATIONS_DIR"));` can't build and I am looking for an alternative to give a build environment variable `env!()` to the macro `embed_migrations!()`

You can comment this line and use `embed_migrations!("migrations");` instead.

the program only print the `MIGRATIONS_DIR` environment variable