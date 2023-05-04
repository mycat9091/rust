fn main() {
    greet_world();
}

fn greet_world() {
    let c = "你好，世界";
    let e = "hello,world";
    let regions = [c, e];

    for region in regions.iter() {
        println!("{}", &region);
    }
}
