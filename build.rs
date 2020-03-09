#[async_std::main]
async fn main() {
    match feather_packet_parser::compile_def("packets/", "src/packets/").await {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e);
        }
    }
}
