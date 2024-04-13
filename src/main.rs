use image::GenericImageView;
use std::env;

fn get_str_ascii(intent: u8) -> &'static str {
    let index = (intent / 4) as usize;
    let ascii: [&str; 64] = [
        " ", ".", "'", "`", "^", ",", ":", ";", "I", "l", "!", "i", ">", "<", "~", "+",
        "_", "-", "?", "[", "{", "1", "(", "|", "/", "t", "f", "j", "r", "x", "n", "u",
        "v", "c", "z", "X", "Y", "U", "J", "C", "L", "Q", "0", "O", "Z", "m", "w", "q",
        "p", "d", "k", "b", "h", "a", "o", "*", "#", "M", "W", "&", "8", "%", "B", "@"
    ];
    ascii[index]
}

fn get_image(dir: &str, scale: usize) {
    let img = image::open(dir).unwrap();
    println!("{:?}", img.dimensions());

    let (width, height) = img.dimensions();
    for y in (0..height).step_by(scale*2)  {
        for x in (0..width).step_by(scale) {
            let pixel = img.get_pixel(x, y);
            let mut intent = ((pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32)/3 ) as u8;
            if pixel[3] == 0 {
                intent = 0;
            }
            print!("{}", get_str_ascii(intent));
        }
        println!("");
    }
}

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <image_path> <scale>", args[0]);
        return;
    }
    let image_path = &args[1];
    let scale: usize = args[2].parse().expect("Scale must be an integer");
    
    get_image(image_path, scale);
}
