use rascii_art::{
    render_to,
    RenderOptions,
};
                                                            
fn main() {
    let mut buffer = String::new();
                                                            
    render_to(
        r"/Users/kllarena/documents/github/myfetch/mental_illness.png",
        &mut buffer,
        &RenderOptions::new()
            .width(100)
            .colored(true)
            .charset(rascii_art::charsets::BLOCK)
    )
    .unwrap();

    println!("{}", buffer);
}
