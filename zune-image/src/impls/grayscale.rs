use zune_core::colorspace::ColorSpace;
use zune_imageprocs::grayscale::rgb_to_grayscale;

use crate::errors::ImgOperationsErrors;
use crate::image::{Image, ImageChannels};
use crate::traits::OperationsTrait;

/// Convert RGB data to grayscale
///
/// This will convert any image that contains three
/// RGB channels(including RGB, RGBA,RGBX) into grayscale
///
/// Formula for RGB to grayscale conversion is given by
///
/// ```text
///Grayscale = 0.299R + 0.587G + 0.114B
/// ```
/// but it's implemented using fixed point integer mathematics and simd kernels
/// where applicable (see zune-imageprocs/grayscale)
pub struct RgbToGrayScale;

impl RgbToGrayScale
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> RgbToGrayScale
    {
        RgbToGrayScale {}
    }
}
impl OperationsTrait for RgbToGrayScale
{
    fn get_name(&self) -> &'static str
    {
        "RGB to Grayscale"
    }

    fn execute_simple(&self, image: &mut Image) -> Result<(), ImgOperationsErrors>
    {
        let im_colorspace = image.get_colorspace();

        // Support any colorspace with RGB data
        if im_colorspace != ColorSpace::RGB
            || im_colorspace != ColorSpace::RGBA
            || im_colorspace != ColorSpace::RGBX
        {
            return Err(ImgOperationsErrors::WrongColorspace(
                ColorSpace::RGB,
                image.get_colorspace(),
            ));
        }

        let (width, height) = image.get_dimensions();
        let size = width * height;

        let mut grayscale = vec![0; size];

        if let ImageChannels::ThreeChannels(rgb_data) = image.get_channel_ref()
        {
            rgb_to_grayscale((&rgb_data[0], &rgb_data[1], &rgb_data[2]), &mut grayscale);
        }
        else if let ImageChannels::FourChannels(rgba_data) = image.get_channel_ref()
        {
            // discard alpha channel
            rgb_to_grayscale(
                (&rgba_data[0], &rgba_data[1], &rgba_data[2]),
                &mut grayscale,
            );
        }
        // change image info to be grayscale
        image.set_image_channel(ImageChannels::OneChannel(grayscale));
        image.set_colorspace(ColorSpace::GrayScale);

        Ok(())
    }
}