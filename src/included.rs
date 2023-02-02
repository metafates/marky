use include_dir::{include_dir, Dir};

pub static VENDOR_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/vendor");
pub static TEMPLATES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates");
