use avatar_rs::Icon;

fn main() {
    let icon = Icon::new(12345);
    icon.save("test.png").unwrap();
}
