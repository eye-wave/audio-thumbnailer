use crate::config::{AspectRatio, CoverSettings};
use crate::Result;
use image::{DynamicImage, GenericImageView};

pub fn load_and_resize(buffer: &[u8], options: &CoverSettings) -> Result<DynamicImage> {
    let size = options.size;
    let aspect_ratio = &options.aspect_ratio;
    let filter = options.interpolation.to_filter_type();

    let picture = image::load_from_memory(buffer)?;
    match aspect_ratio {
        AspectRatio::Auto => {
            let (width, height) = picture.dimensions();
            let [nwidth, nheight] = if width > height {
                [size * width / height, size]
            } else {
                [size, size * height / width]
            };

            Ok(picture.resize(nwidth, nheight, filter))
        }
        AspectRatio::Stretch => Ok(picture.resize_exact(size, size, filter)),
        AspectRatio::Crop => {
            let (width, height) = picture.dimensions();
            let mut x = 0;
            let mut y = 0;
            let [nwidth, nheight] = if width > height {
                let nwidth = size * width / height;
                x = (nwidth - size) / 2;

                [nwidth, size]
            } else {
                let nheight = size * height / width;
                y = (nheight - size) / 2;

                [size, nheight]
            };

            Ok(picture
                .resize(nwidth, nheight, filter)
                .crop(x, y, size, size))
        }
    }
}
