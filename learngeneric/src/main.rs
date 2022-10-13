mod inter;
mod ops;
mod point;
use inter::test_trait;
use point::test_point;

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug")
    }
    tracing_subscriber::fmt::init();
    test_point();

    test_trait();
}
