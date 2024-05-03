use winres::WindowsResource;

fn main() {
    let mut res = WindowsResource::new();

    res.set_icon("assets/logo.ico");
    res.compile().unwrap();
}
