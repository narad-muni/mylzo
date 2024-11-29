use mylzo::decompress;

fn main() {
    let out = decompress(&[], &mut []);

    println!("{out:?}");
}
