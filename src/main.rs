fn main() {
    const STACK_SIZE: usize = 128 * 1024 * 1024;
    std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(|| {
            apery::usi::cmd_loop();
        })
        .unwrap()
        .join()
        .unwrap();
}
