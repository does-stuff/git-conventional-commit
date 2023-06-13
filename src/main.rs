pub mod clap_handler;
pub mod user_input_handler;
pub mod utils;

fn main() {
    let args = std::env::args();

    if args.len() > 1 {
        clap_handler::handle_with_clap();
    } else {
        user_input_handler::handle_with_user_input();
    }
}
