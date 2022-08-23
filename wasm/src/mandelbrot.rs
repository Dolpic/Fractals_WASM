use num::complex::Complex;

pub struct Color{
    r : u8,
    g : u8,
    b : u8
}

pub struct Mandelbrot{
    width      : usize,
    height     : usize,
    pos_x      : f64,
    pos_y      : f64,
    zoom       : u128,
    iterations : u32,
    colors     : Vec<u8>
}

impl Mandelbrot{
    pub fn new(width:usize, height:usize, pos_x:f64, pos_y:f64, zoom:u128, iterations:u32, colors:Vec<u8>) -> Mandelbrot{
        Mandelbrot{width, height, pos_x, pos_y, zoom, iterations, colors}
    }

    pub fn compute(&self) -> Vec<u8>{
        let mut result = Vec::with_capacity(self.width * self.height);

        let in_color  = Color{r:self.colors[0],g:self.colors[1],b:self.colors[2]};
        let out_color = Color{r:self.colors[3],g:self.colors[4],b:self.colors[5]};
        let highlight = Color{r:self.colors[6],g:self.colors[7],b:self.colors[8]};

        let start_x = -(self.width as f64) /(2*self.zoom)as f64 + self.pos_x;
        let start_y = -(self.height as f64)/(2*self.zoom)as f64 + self.pos_y;

        let incr = 1.0 / self.zoom as f64;

        let norm_limit = 2.0;
        let nb_iterations = self.iterations;

        let mut cur_y = start_y;
        for _y in 0..self.height{
            let mut cur_x = start_x;
            for _x in 0..self.width{

                let mut z = Complex::new(0.0, 0.0 );
                let c = Complex::new(cur_x, cur_y);

                let mut i = 0;
                while i < nb_iterations && z.norm_sqr() < norm_limit{
                    z = z*z + c;
                    i += 1;
                }

                if z.norm_sqr() >= norm_limit {
                    let ratio = i as f32 / nb_iterations as f32;
                    result.push( ((highlight.r as f32 * ratio + out_color.r as f32 * (1.0-ratio))/2.0) as u8);
                    result.push( ((highlight.g as f32 * ratio + out_color.g as f32 * (1.0-ratio))/2.0) as u8);
                    result.push( ((highlight.b as f32 * ratio + out_color.b as f32 * (1.0-ratio))/2.0) as u8);
                    result.push(255);
                }else {
                    result.push(in_color.r);
                    result.push(in_color.g);
                    result.push(in_color.b);
                    result.push(255);
                }

                cur_x += incr;
            }
            cur_y += incr;
        }
        
        return result;
    }
}