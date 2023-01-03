// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(subcommand_required = true)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Blur {
        #[arg(value_name = "INFILE")]
        infile: String,

        #[arg(value_name = "OUTFILE")]
        outfile: String,

        #[arg(short, long, value_name = "VALUE", default_value_t = 2.0)]
        sigma: f32,
    },

    Brighten {
        #[arg(value_name = "INFILE")]
        infile: String,

        #[arg(value_name = "OUTFILE")]
        outfile: String,

        #[arg(value_name = "BRIGHTNESS")]
        brightness: i32,
    },

    Crop {
        #[arg(value_name = "INFILE")]
        infile: String,

        #[arg(value_name = "OUTFILE")]
        outfile: String,

        x: u32,
        y: u32,
        w: u32,
        h: u32,
    },

    Rotate {
        #[arg(value_name = "INFILE")]
        infile: String,

        #[arg(value_name = "OUTFILE")]
        outfile: String,

        #[arg(value_name = "ANGLE", value_parser(["90", "180", "270"]))]
        angle: String,
    },

    Invert {
        #[arg(value_name = "INFILE")]
        infile: String,

        #[arg(value_name = "OUTFILE")]
        outfile: String,
    },

    Grayscale {
        #[arg(value_name = "INFILE")]
        infile: String,

        #[arg(value_name = "OUTFILE")]
        outfile: String,
    },

    Fractal {
        #[arg(value_name = "OUTFILE")]
        outfile: String,
    },

    Generate {
        #[arg(value_name = "OUTFILE")]
        outfile: String,

        r: u8,
        g: u8,
        b: u8,
    },
}

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let cli = CLI::parse();
    match cli.command {
        Commands::Blur { infile, outfile, sigma, } => {
            blur(infile, outfile, sigma);
        }
        Commands::Brighten { infile, outfile, brightness, } => {
            brighten(infile, outfile, brightness);
        }
        Commands::Crop { infile, outfile, x, y, w, h, } => {
            crop(infile, outfile, x, y, w, h);
        }
        Commands::Rotate { infile, outfile, angle, } => {
            rotate(infile, outfile, &angle);
        }
        Commands::Invert { infile, outfile } => {
            invert(infile, outfile);
        }
        Commands::Grayscale { infile, outfile } => {
            grayscale(infile, outfile);
        }
        Commands::Fractal { outfile } => {
            fractal(outfile);
        }
        Commands::Generate { outfile , r, g, b} => {
            generate(outfile, r, g, b);
        }
    };
}

fn blur(infile: String, outfile: String, sigma: f32) {
    let img_in = image::open(infile).expect("Failed to open INFILE.");
    let img_out = img_in.blur(sigma);
    img_out.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, brightness: i32) {
    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.

    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
    let img_in = image::open(infile).expect("Failed to open INFILE.");
    let img_out = img_in.brighten(brightness);
    img_out.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(infile: String, outfile: String, x: u32, y: u32, w: u32, h: u32) {
    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.

    // Challenge: parse the four values from the command-line and pass them
    // through to this function.
    let img_in = image::open(infile).expect("Failed to open INFILE.");
    let img_out = img_in.crop_imm(x, y, w, h);
    img_out.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate(infile: String, outfile: String, angle: &str) {
    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!

    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.
    let img_in = image::open(infile).expect("Failed to open INFILE.");
    let img_out = match angle {
        "90" => img_in.rotate90(),
        "180" => img_in.rotate180(),
        "270" => img_in.rotate270(),
        _ => panic!(),
    };
    img_out.save(outfile).expect("Failed writing OUTFILE.");
}

fn invert(infile: String, outfile: String) {
    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.
    let img_in = image::open(infile).expect("Failed to open INFILE.");
    let mut img_out = img_in.clone();
    img_out.invert();
    img_out.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: String, outfile: String) {
    // .grayscale() takes no arguments. It returns a new image.
    let img_in = image::open(infile).expect("Failed to open INFILE.");
    let img_out = img_in.grayscale();
    img_out.save(outfile).expect("Failed writing OUTFILE.");
}

fn generate(outfile: String, r: u8, g: u8, b: u8) {
    // Create an ImageBuffer -- see fractal() for an example

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example

    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!
    let width = 800;
    let height = 800;

    let mut img_buf = image::ImageBuffer::new(width, height);

    // Iterate over the coordinates and pixels of the image
    for (_, _, pixel) in img_buf.enumerate_pixels_mut() {
        *pixel = image::Rgb([r, g, b]);
    }

    img_buf.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
