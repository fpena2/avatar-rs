use avatar_rs::Icon;

fn main() {
    let icon = Icon::new(1253);
    icon.save("test.png").unwrap();
}
