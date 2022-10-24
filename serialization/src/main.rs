use model::AdModel;

mod model;
mod util;

fn main() {
    let res =
        util::from_file("/Users/weixuan/code/rustcode/rustexample/serialization/src/input.json");

    util::to_json();
    util::to_obj();

    println!("res {:?}", res);
}
