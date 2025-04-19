// this whole file is from chatgpt -- RTiOW has v little explanation on this section of the code and I don't know Rust well enough to translate it quickly, and I'm not interested enough in this section to spend the time on it.

// uses stb_image so that it can directly translate the tutorial code
// if you're using this in a proper raytracer, use the image crate

use std::env;
use std::ffi::CString;
use std::path::Path;
use std::ptr;

extern crate stb_image; // add `stb_image = "0.2"` to Cargo.toml

pub struct RTWImage {
    bytes_per_pixel: usize,      // always 3 (RGB)
    fdata: Vec<f32>,             // floating‐point pixels
    bdata: Vec<u8>,              // converted byte pixels
    width: usize,
    height: usize,
    bytes_per_scanline: usize,
}

impl RTWImage {
    /// Create an empty placeholder.
    pub fn new() -> Self {
        RTWImage {
            bytes_per_pixel: 3,
            fdata: Vec::new(),
            bdata: Vec::new(),
            width: 0,
            height: 0,
            bytes_per_scanline: 0,
        }
    }

    /// Try to load from filename, searching in RTW_IMAGES and up to 6 parent levels.
    pub fn new_from<P: AsRef<Path>>(filename: P) -> Result<Self, String> {
        let filename = filename.as_ref();
        let mut candidates = Vec::new();

        // 1) $RTW_IMAGES
        if let Ok(dir) = env::var("RTW_IMAGES") {
            candidates.push(Path::new(&dir).join(filename));
        }
        // 2) local paths
        candidates.push(filename.to_path_buf());
        candidates.push(Path::new("images").join(filename));
        // 3) up to six levels of ../images/
        for lvl in 1..=6 {
            let mut p = Path::new(&"../".repeat(lvl)).join("images").join(filename);
            candidates.push(p);
        }

        // Attempt loads
        for path in candidates {
            if let Ok(mut img) = RTWImage::load(&path) {
                return Ok(img);
            }
        }

        Err(format!("ERROR: Could not load image file '{}'.", filename.display()))
    }

    /// Load the file at `path`, returning Err on stbi failure.
    fn load(path: &Path) -> Result<Self, String> {
        // Prepare placeholders
        let mut img = RTWImage::new();
        let c_path = CString::new(path.to_string_lossy().as_ref())
            .map_err(|e| e.to_string())?;

        let mut w = 0;
        let mut h = 0;
        // Call into stb_image's float loader
        let raw_ptr = unsafe {
            stb_image::stb_image::stbi_loadf(
                c_path.as_ptr(),
                &mut w,
                &mut h,
                ptr::null_mut(),
                img.bytes_per_pixel as i32,
            )
        };
        if raw_ptr.is_null() {
            return Err("stbi_loadf failed".into());
        }

        // Build Vec<f32> from the returned pointer
        let count = (w as usize) * (h as usize) * img.bytes_per_pixel;
        let slice = unsafe { std::slice::from_raw_parts(raw_ptr, count) };
        img.fdata = slice.to_vec();

        // Free the raw buffer
        unsafe { stb_image::stb_image::stbi_image_free(raw_ptr as *mut _) };

        img.width = w as usize;
        img.height = h as usize;
        img.bytes_per_scanline = img.width * img.bytes_per_pixel;

        img.convert_to_bytes();
        Ok(img)
    }

    /// Convert the floating‐point data `[0.0..1.0]` into bytes `[0..255]`.
    fn convert_to_bytes(&mut self) {
        let total = self.width * self.height * self.bytes_per_pixel;
        self.bdata = Vec::with_capacity(total);
        for &v in &self.fdata {
            self.bdata.push(Self::float_to_byte(v));
        }
    }

    #[inline]
    fn float_to_byte(v: f32) -> u8 {
        if v <= 0.0       { 0 }
        else if v >= 1.0  { 255 }
        else              { (256.0 * v) as u8 }
    }

    /// Clamp `x` into `[low, high)`.
    #[inline]
    fn clamp(x: i32, low: i32, high: i32) -> i32 {
        if x < low       { low }
        else if x < high { x }
        else              { high - 1 }
    }

    /// Return the `(r,g,b)` bytes at `(x,y)` or magenta if no data.
    pub fn pixel_data(&self, x: i32, y: i32) -> [u8; 3] {
        static MAGENTA: [u8; 3] = [255, 0, 255];
        if self.bdata.is_empty() {
            return MAGENTA;
        }
        let xi = Self::clamp(x, 0, self.width as i32) as usize;
        let yi = Self::clamp(y, 0, self.height as i32) as usize;
        let offset = yi * self.bytes_per_scanline + xi * self.bytes_per_pixel;
        [
            self.bdata[offset],
            self.bdata[offset + 1],
            self.bdata[offset + 2],
        ]
    }

    pub fn width(&self)  -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
}

// `Drop` isn’t needed because `Vec` frees itself.

impl Clone for RTWImage {
    fn clone(&self) -> Self {
        Self {
            bytes_per_pixel: self.bytes_per_pixel.clone(),      // always 3 (RGB)
            fdata: self.fdata.clone(),             // floating‐point pixels
            bdata: self.bdata.clone(),              // converted byte pixels
            width: self.width.clone(),
            height: self.height.clone(),
            bytes_per_scanline: self.bytes_per_scanline.clone(),
        }
    }
}


/*
use crate::colour::Colour;

pub struct RTWImage {
    //private
    bytes_per_pixel: i32,
    fdata: Option<f32>,
    bdata: Option<f32>,
    image_width: Option<i32>,
    image_height: Option<i32>,
    bytes_per_scanline: i32,
}

impl RTWImage {

    pub fn new() -> Self {
        Self {
            bytes_per_pixel: 0,
            fdata: None,
            bdata: None,
            image_width: None,
            image_height: None,
            bytes_per_scanline: 0,
        }
    }

    pub fn new_from(image_filename: &str) -> Self {
        // get a handle on the image file
        let filename = image_filename;

        // use load function to load the image and initialise self
        
        return load(filename);
    }

    pub fn delete() {
        // how do deconstructors work in rust? They're not needed but what do you do instead? Just leave it?

    }

    pub fn load(filepath: &str) {

    }

    pub fn width(&self) -> Option<i32> {
            return self.image_width;
    }

    pub fn height(&self) -> Option<i32> {
        return self.image_height;
    }

    pub fn pixel_data(&self, x: i32, y: i32) -> Colour {
        // Return the address of the three RGB bytes of the pixel at x,y. If there is no image
        // data, returns magenta.

        let magenta = Colour::new_from(255.0, 0.0, 255.0);

        if !self.bdata.is_some() {
            return magenta;
        }
        else {
            let x2 = self.clamp(x, 0, self.image_width.expect("No Image Width!")) as f32;
            let y2 = self.clamp(y, 0, self.image_height.expect("No Image Height!")) as f32;
            return Colour::new_from(self.bdata.unwrap(), y2*self.bytes_per_scanline as f32, x2*self.bytes_per_pixel as f32);
        }
    }

    // private 
    fn clamp(&self, x: i32, low: i32, high: i32) -> i32 {
        if x < low {return low}
        if x < high {return x}
        return high - 1;
    }

    fn float_to_byte(value: f32) -> char {
        if value <= 0.0 {return 0 as char}
        else if 1.0 <= value {return 255 as char}
        else {return ((255*value as u8) + (value as u8)) as char}
    }

    fn convert_to_bytes() {
        /*
        
        // Convert the linear floating point pixel data to bytes, storing the resulting byte
        // data in the `bdata` member.

        int total_bytes = image_width * image_height * bytes_per_pixel;
        bdata = new unsigned char[total_bytes];

        // Iterate through all pixel components, converting from [0.0, 1.0] float values to
        // unsigned [0, 255] byte values.

        auto *bptr = bdata;
        auto *fptr = fdata;
        for (auto i=0; i < total_bytes; i++, fptr++, bptr++)
            *bptr = float_to_byte(*fptr);

         */
    }
}
    */