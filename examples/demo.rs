use lsp_doc::lsp_doc;

fn main() {
    say_hello();
}

#[lsp_doc("examples/hello.md")]
pub fn say_hello() {
    println!("Hello, world!");
}
