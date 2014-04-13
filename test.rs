struct Stream {
    text: ~str,
    ptr: int
}
fn main() {

    let mut stream = Stream{ text : ~"test text", ptr: 0};

    for c in stream.text.chars() {
        println!("{}", c);
        stream.ptr += 1;

    }
}
