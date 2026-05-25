mod content_build;

fn main() {
    if let Err(error) = content_build::build_content() {
        panic!("content build failed: {error}");
    }
}
