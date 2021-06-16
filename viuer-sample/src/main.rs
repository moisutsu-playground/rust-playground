use viuer::{print_from_file, Config};

fn main() {
    let conf = Config {
        absolute_offset: false,
        ..Default::default()
    };

    print_from_file("image/rust.jpg", &conf).expect("Image printing failed.");
}
