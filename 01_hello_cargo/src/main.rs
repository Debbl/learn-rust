fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "Hello, world!";
    let regions = [southern_germany, chinese, english];
    for r in regions.iter() {
        println!("{}", &r);
    }
}
fn main() {
    greet_world();
}
