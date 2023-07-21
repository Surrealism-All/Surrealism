use surrealism::{DefaultInitService, InitService};

fn main() {
    let mut service = DefaultInitService::new();
    service.init();
    // println!("{:?}", service);
}
