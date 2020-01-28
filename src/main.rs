fn main() {
    std::thread::Builder::new()
        .stack_size(apery::stack_size::STACK_SIZE)
        .spawn(|| {
            apery::usi::cmd_loop();
        })
        .unwrap()
        .join()
        .unwrap();
}
