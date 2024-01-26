extern crate embed_resource;

fn main() {
    // Embed the application manifest
    embed_resource::compile("app.rc", embed_resource::NONE);
}
