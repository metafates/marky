use include_dir::{include_dir, Dir};

pub static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");
