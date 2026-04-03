use windows::Win32::Graphics::Gdi::{BITMAPINFO, BI_RGB};

#[derive(Default)]
pub struct MSImageBuffer {
    pub buffer: Vec<u8>, // [b, g, r, 0] = one pixel with colour of rgb
    pub width: i32,
    pub height: i32,
    pub bitmap_info: BITMAPINFO,
}

impl crate::traits::ImageBuffer for MSImageBuffer {
    fn init(&mut self, width: i32, height: i32) {
        // Info from: https://www.youtube.com/watch?v=Zp1SXzISgy4
        self.bitmap_info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFO>() as u32;
        self.bitmap_info.bmiHeader.biPlanes = 1;
        self.bitmap_info.bmiHeader.biBitCount = 32;
        self.bitmap_info.bmiHeader.biCompression = BI_RGB.0;

        self.resize_buffer(width, height);
    }

    fn resize_buffer(&mut self, width: i32, height: i32) {
        if width == self.width && height == self.height {
            return
        }

        self.bitmap_info.bmiHeader.biWidth = width as i32;
        self.bitmap_info.bmiHeader.biHeight = -(height as i32); // Has to be negative to create a top down image

        self.width = width;
        self.height = height;

        self.buffer.resize((4 * width * height) as usize, 0);
    }

    fn clear_buffer(&mut self) {
        self.buffer = vec![0; (4 * self.width * self.height) as usize];
    }

    fn set_buffer(&mut self, buffer: Vec<u8>) {
        let len = buffer.len();
        let mut buffer_resized: Vec<u8> = vec![0; len * 4/3];

        for i in 0..(len/3) {
            buffer_resized[4 * i]     = buffer[3 * i + 2]; // b
            buffer_resized[4 * i + 1] = buffer[3 * i + 1]; // g
            buffer_resized[4 * i + 2] = buffer[3 * i];     // r
            // padding
        }

        self.buffer = buffer_resized;
    }

    fn set_buffer_direct(&mut self, buffer: Vec<u8>) {
        self.buffer = buffer;
    }
}
