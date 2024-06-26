use crate::args::aspect_ratio::AspectRatio;
use image::{imageops::FilterType, DynamicImage, GenericImageView};

pub fn load_and_resize(
    buffer: &[u8],
    size: u8,
    aspect_ratio: &AspectRatio,
    filter: &FilterType,
) -> Option<DynamicImage> {
    let size = size as u32;

    match image::load_from_memory(buffer) {
        Ok(picture) => match aspect_ratio {
            AspectRatio::Auto => {
                let (width, height) = picture.dimensions();
                let [nwidth, nheight] = if width > height {
                    [size * width / height, size]
                } else {
                    [size, size * height / width]
                };

                Some(picture.resize(nwidth, nheight, *filter))
            }
            AspectRatio::Stretch => Some(picture.resize_exact(size, size, *filter)),
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

                Some(
                    picture
                        .resize(nwidth, nheight, *filter)
                        .crop(x, y, size, size),
                )
            }
        },
        Err(_) => None,
    }
}
