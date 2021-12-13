fn main() {
    println!("Hello, world!");
}

fn calculate_mandelbrot(max_iters: usize,
                        x_min:f64,
                        x_max:f64,
                        y_min:f64,
                        y_max:f64,
                        width: usize,
                        height: usize) -> Vec<Vec<usize>>{
    let mut all_rows: Vec<Vec<usize>> = Vec::with_capacity(width);
    for img_y in 0 .. height {
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0 .. width{
            let cx = x_min + (x_max - x_min) * (img_x as f64 / width as f64);
            let cy = y_min + (y_max - y_min) * (img_y as f64 / height as f64);
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at)
        }
        all_rows.push(row);
    }
    all_rows
}

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    todo!()
}

