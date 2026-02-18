use avatar_rs::Avatar;

fn main() {
    let icon = Avatar::new(12345);
    icon.save("test.png").unwrap();
}
