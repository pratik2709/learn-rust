use image;
use num::complex::Complex;


fn get_color(i: u32, max_iters: u32) -> image::Rgb<u8> {
    if i > max_iters{
        return image::Rgb([255,255,255]);
    }
    if max_iters == 255 {
        let idx = i as u8;
        return image::Rgb([idx, idx, idx]);
    }
    let idx = (((i as f32)/(max_iters as f32))*255.0).round() as u8;
    return image::Rgb([idx, idx, idx]);
}

fn calculate_mandelbrot(max_iters: usize,
                        x_min: f64,
                        x_max: f64,
                        y_min: f64,
                        y_max: f64,
                        width: usize,
                        height: usize) -> Vec<Vec<usize>> {
    let mut img = image::RgbImage::new(width as u32, height as u32);
    let mut all_rows: Vec<Vec<usize>> = Vec::with_capacity(width);
    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let cx = x_min + (x_max - x_min) * (img_x as f64 / width as f64);
            let cy = y_min + (y_max - y_min) * (img_y as f64 / height as f64);
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
            let rgb = get_color(escaped_at as u32, max_iters as u32);
            img.put_pixel(img_x as u32,img_y as u32, rgb)
        }
        all_rows.push(row);
    }
    let fname = format!("mandelbrot.png");
    img.save_with_format(fname, image::ImageFormat::Png)
        .unwrap();
    all_rows
}

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        // find absolute value of the imaginary number
        // greater than 2 indicates it will continue growing infinitely
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    return max_iters;
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                2..=5 => '.',
                5..=10 => '•',
                11..=30 => '*',
                30..=100 => '+',
                100..=200 => 'x',
                200..=400 => '$',
                400..=700 => '#',
                _ => '%',
            };
            line.push(val);
        }
        println!("{}", line);
    }
}


fn main() {
    let mandelbrot = calculate_mandelbrot(
        1000, -2.0, 1.0, -1.0, 1.0, 100, 24);
    render_mandelbrot(mandelbrot);
}
