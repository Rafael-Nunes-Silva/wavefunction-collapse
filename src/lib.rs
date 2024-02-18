use image::io::Reader;

pub fn run() {
    let flower = Reader::open("images/flower.png")
        .expect("Failed at reading file")
        .decode()
        .expect("Failed at decoding image");

    flower.save("result.png").expect("Failed at saving image");
}
