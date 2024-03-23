mod integrate;

fn main() {
    integrate::pipe();

    std::thread::sleep(std::time::Duration::from_secs(1000000000000000));
}
