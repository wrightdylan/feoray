use crate::Colour;
use image::{ImageBuffer, ImageResult, Rgb};
use std::ops::{Index, IndexMut};

// Max size is 18.44 x 18.44 exapixels
#[derive(Debug, PartialEq)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Colour>,
}

impl Canvas {
    /// Constructs a new blank cavas of any colour.
    /// Maximum size is 18.44 x 18.44 exapixels. That's plenty of space!
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let c = Canvas::new(3840, 2160, colour(0.3, 0.3, 0.3));
    /// 
    /// assert_eq!(c.width, 3840);
    /// assert_eq!(c.height, 2160);
    /// assert_eq!(c.pixels, vec![Colour::new(0.3, 0.3, 0.3); 3840 * 2160]);
    /// ```
    pub fn new(width: usize, height: usize, colour: Colour) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![colour; width * height]
        }
    }

    /// Returns the colour of the selected pixel
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let c = Canvas::new(10, 20, Colour::black());
    /// 
    /// assert_eq!(c.read_pix(2, 3), Colour::black());
    /// ```
    pub fn read_pix(&self, x: usize, y: usize) -> Colour {
        self[(x, y)]
    }

    /// Overwrites the colour of the selected pixel
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let c = Canvas::new(10, 20, Colour::black());
    /// c.write_pix(2, 3, Colour::red());
    /// 
    /// assert_eq!(c.read_pix(2, 3), Colour::red());
    /// ```
    pub fn write_pix(&mut self, x: usize, y: usize, colour: Colour ) {
        self[(x, y)] = colour
        
    }

    /// Exports the canvas to a recognisable image format.
    /// Uses the `image` crate, which does all the heavy lifting.
    /// Can export to many popular image formats, where format is automatically deduced from the path.
    /// 
    /// Default formats are: bmp, gif, ico, jpg, jpeg, pam, png, ppm, tiff, tga
    /// Formats that don't work: hdr, pbm, pgm, webp
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// use image::{ImageBuffer, ImageResult, Rgb};
    /// 
    /// let c = Canvas::new(10, 20, Colour::black());
    /// c.write_pix(2, 3, Colour::red());
    /// c.export("image.jpg").unwrap();
    /// ```
    pub fn export(&self, path: &str) -> ImageResult<()> {
        let mut img = ImageBuffer::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let colour = &self.read_pix(x as usize, y as usize);
            let (r, g, b) = colour.scale();
            *pixel = Rgb([r, g, b]);
        }

        img.save(path)
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = Colour;

    // Returns the colour of a pixel at location on canvas[(x, y)]
    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        let idx = (self.width * row) + col;
        &self.pixels[idx]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    // Changes the colour of a pixel at location on canvas[(x, y)]
    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Colour {
        let idx = (self.width * row) + col;
        &mut self.pixels[idx]
    }
}

pub fn canvas(width: usize, height: usize) -> Canvas {
    Canvas::new(width, height, Colour::black())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_black_canvas() {
        let c = canvas(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert_eq!(c.pixels, vec![Colour::new(0.0, 0.0, 0.0); 200]);
    }

    #[test]
    fn read_colour_at_pixel() {
        let c = canvas(10, 20);

        assert_eq!(c.read_pix(2, 3), Colour::black());
    }

    #[test]
    fn write_colour_at_pixel() {
        let mut c = canvas(10, 20);
        // Remember that the index is off by one
        c.write_pix(2, 3, Colour::red());
        println!("{:?}", c);

        assert_eq!(c.read_pix(2, 3), Colour::red());
    }

    /*#[test]
    fn write_blank_canvas() {
        let cnvs = canvas(5, 3);
        cnvs.export("blank_canvas.ppm").unwrap();
    }

    #[test]
    fn construct_pixel_data() {
        let mut cnvs = canvas(5, 3);
        let c1 = Colour::new(1.5, 0.0, 0.0);
        let c2 = Colour::new(0.0, 0.5, 0.0);
        let c3 = Colour::new(-0.5, 0.0, 1.0);

        cnvs.write_pix(0, 0, c1);
        cnvs.write_pix(2, 1, c2);
        cnvs.write_pix(4, 2, c3);

        cnvs.export("construct_pixel_data.ppm").unwrap();
    }

    #[test]
    fn split_long_lines() {
        let c = Colour::new(1.0, 0.8, 0.6);
        let cnvs = Canvas::new(10, 2, c);

        cnvs.export("long_lines.ppm").unwrap();
    }*/
}