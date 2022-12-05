// image - A module for handling Excel image files.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2022, John McNamara, jmcnamara@cpan.org

#![warn(missing_docs)]

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use crate::XlsxError;

#[derive(Clone, Debug)]
/// The Image struct is used to create an object to represent an image that can
/// be inserted into a worksheet.
///
/// ```rust
/// # // This code is available in examples/doc_image.rs
/// #
/// use rust_xlsxwriter::{Image, Workbook, XlsxError};
///
/// fn main() -> Result<(), XlsxError> {
///     // Create a new Excel file object.
///     let mut workbook = Workbook::new();
///
///     // Add a worksheet to the workbook.
///     let worksheet = workbook.add_worksheet();
///
///     // Create a new image object.
///     let image = Image::new("examples/rust_logo.png")?;
///
///     // Insert the image.
///     worksheet.insert_image(1, 2, &image)?;
///
///     // Save the file to disk.
///     workbook.save("image.xlsx")?;
///
///     Ok(())
/// }
/// ```
///
/// Output file:
///
/// <img src="https://rustxlsxwriter.github.io/images/image_intro.png">
///
pub struct Image {
    height: f64,
    width: f64,
    width_dpi: f64,
    height_dpi: f64,
    scale_width: f64,
    scale_height: f64,
    has_default_dpi: bool,
    pub(crate) x_offset: u32,
    pub(crate) y_offset: u32,
    pub(crate) image_type: XlsxImageType,
    pub(crate) alt_text: String,
    path: PathBuf,
}

impl Image {
    // -----------------------------------------------------------------------
    // Public (and crate public) methods.
    // -----------------------------------------------------------------------

    /// Create a new Image object from an image file.
    ///
    /// Create an Image object from a path to an image file. The image can then
    /// be inserted into a worksheet.
    ///
    /// The supported image formats are:
    ///
    /// - PNG
    /// - JPG
    /// - GIF: The image can be an animated gif in more resent versions of
    ///   Excel.
    /// - BMP: BMP images are only supported for backward compatibility. In
    ///   general it is best to avoid BMP images since they are not compressed.
    ///   If used, BMP images must be 24 bit, true color, bitmaps.
    ///
    /// EMF and WMF file formats will be supported in an upcoming version of the
    /// library.
    ///
    /// **NOTE on SVG files**: Excel doesn't directly support SVG files in the
    /// same way as other image file formats. It allows SVG to be inserted into
    /// a worksheet but converts them to, and displays them as, PNG files. It
    /// stores the original SVG image in the file so the original format can be
    /// retrieved. This removes the file size and resolution advantage of using
    /// SVG files. As such SVG files are not supported by `rust_xlsxwriter`
    /// since a conversion to the PNG format would be required and that format
    /// is already supported.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the image file to read e as a `&str` or as a
    ///   [`std::path`] Path or PathBuf instance.
    ///
    /// # Errors
    ///
    /// * [`XlsxError::UnknownImageType`] - Unknown image type. The supported
    ///   image formats are PNG, JPG, GIF and BMP.
    /// * [`XlsxError::ImageDimensionError`] - Image has 0 width or height, or
    ///   the dimensions couldn't be read.
    ///
    /// # Examples
    ///
    /// The following example demonstrates creating a new Image object and
    /// adding it to a worksheet.
    ///
    /// ```
    /// # // This code is available in examples/doc_image.rs
    /// #
    /// # use rust_xlsxwriter::{Image, Workbook, XlsxError};
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #
    /// #     // Add a worksheet to the workbook.
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    ///     // Create a new image object.
    ///     let image = Image::new("examples/rust_logo.png")?;
    ///
    ///     // Insert the image.
    ///     worksheet.insert_image(1, 2, &image)?;
    /// #
    /// #     // Save the file to disk.
    /// #     workbook.save("image.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img src="https://rustxlsxwriter.github.io/images/image_intro.png">
    ///
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Image, XlsxError> {
        let mut path_buf = PathBuf::new();
        path_buf.push(path);

        let mut image = Image {
            height: 0.0,
            width: 0.0,
            width_dpi: 96.0,
            height_dpi: 96.0,
            scale_width: 1.0,
            scale_height: 1.0,
            x_offset: 0,
            y_offset: 0,
            has_default_dpi: true,
            image_type: XlsxImageType::Unknown,
            alt_text: "".to_string(),
            path: path_buf,
        };

        Self::process_image(&mut image)?;

        // Check that we read a valid image.
        if let XlsxImageType::Unknown = image.image_type {
            return Err(XlsxError::UnknownImageType);
        }

        // Check that we read a the image dimensions.
        if image.width == 0.0 || image.height == 0.0 {
            return Err(XlsxError::ImageDimensionError);
        }

        Ok(image)
    }

    /// Set the height scale for the image.
    ///
    /// Set the height scale for the image relative to 1.0/100%. As with Excel
    /// this sets a logical scale for the image, it doesn't rescale the actual
    /// image. This allows the user to get back the original unscaled image.
    ///
    /// **Note for macOS Excel users**: the scale shown on Excel for macOS is
    /// different from the scale on Windows. This is an Excel issue and not a
    /// rust_xlsxwriter issue.
    ///
    /// # Arguments
    ///
    /// * `scale` - The scale ratio.
    ///
    /// # Examples
    ///
    /// This example shows how to create an image object and use it to insert
    /// the image into a worksheet. The image in this case is scaled.
    ///
    /// ```
    /// # // This code is available in examples/doc_image_set_scale_width.rs
    /// #
    /// # use rust_xlsxwriter::{Image, Workbook, XlsxError};
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #
    /// #     // Add a worksheet to the workbook.
    /// #     let worksheet = workbook.add_worksheet();
    ///
    ///     // Create a new image object.
    ///     let mut image = Image::new("examples/rust_logo.png")?;
    ///
    ///     // Set the image scale.
    ///     image.set_scale_height(0.75);
    ///     image.set_scale_width(0.75);
    ///
    ///     // Insert the image.
    ///     worksheet.insert_image(1, 2, &image)?;
    /// #
    /// #     // Save the file to disk.
    /// #     workbook.save("image.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img
    /// src="https://rustxlsxwriter.github.io/images/image_set_scale_width.png">
    ///
    pub fn set_scale_height(&mut self, scale: f64) -> &mut Image {
        self.scale_height = scale;
        self
    }

    /// Set the width scale for the image.
    ///
    /// Set the width scale for the image relative to 1.0/100%. See the
    /// [set_scale_height()](Image::set_scale_height) method for details.
    ///
    /// # Arguments
    ///
    /// * `scale` - The scale ratio.
    ///
    pub fn set_scale_width(&mut self, scale: f64) -> &mut Image {
        self.scale_width = scale;
        self
    }

    /// This will be documented in the next release when the "decorative"
    /// property is added.
    #[doc(hidden)]
    pub fn set_alt_text(&mut self, alt_text: &str) -> &mut Image {
        self.alt_text = alt_text.to_string();
        self
    }

    /// Get the width of the image used for the size calculations in Excel.
    ///
    /// # Examples
    ///
    /// This example shows how to get some of the properties of an Image that
    /// will be used in an Excel worksheet.
    ///
    /// ```
    /// # // This code is available in examples/doc_image_dimensions.rs
    /// #
    /// # use rust_xlsxwriter::{Image, XlsxError};
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     let image = Image::new("examples/rust_logo.png")?;
    /// #
    /// #     assert_eq!(106.0, image.width());
    /// #     assert_eq!(106.0, image.height());
    /// #     assert_eq!(96.0, image.width_dpi());
    /// #     assert_eq!(96.0, image.height_dpi());
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img src="https://rustxlsxwriter.github.io/images/image_dimensions.png">
    ///
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Get the height of the image used for the size calculations in Excel. See
    /// the example above.
    ///
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Get the width/horizontal DPI of the image used for the size calculations
    /// in Excel. See the example above.
    ///
    /// Excel assumes a default image DPI of 96.0 and scales all other DPIs
    /// relative to that.
    ///
    pub fn width_dpi(&self) -> f64 {
        self.width_dpi
    }

    /// Get the height/vertical DPI of the image used for the size calculations
    /// in Excel. See the example above.
    ///
    /// Excel assumes a default image DPI of 96.0 and scales all other DPIs
    /// relative to that.
    ///
    pub fn height_dpi(&self) -> f64 {
        self.height_dpi
    }

    // Get the scale width of the image for Excel size calculations.
    pub(crate) fn width_scaled(&self) -> f64 {
        // Scale to user scale.
        let width = (self.width as f64) * self.scale_width;

        // Scale for non 96dpi resolutions.
        width * 96.0 / self.width_dpi
    }

    // Get the scale height of the image for Excel size calculations.
    pub(crate) fn height_scaled(&self) -> f64 {
        // Scale to user scale.
        let height = (self.height as f64) * self.scale_height;

        // Scale for non 96dpi resolutions.
        height * 96.0 / self.height_dpi
    }

    // Get the image data as a u8 stream to add to the zipfile.
    pub(crate) fn data(&self) -> Vec<u8> {
        let file = File::open(self.path.clone()).unwrap();
        let mut reader = BufReader::new(file);
        let mut data: Vec<u8> = vec![];
        reader.read_to_end(&mut data).unwrap();
        data
    }

    // -----------------------------------------------------------------------
    // Internal methods.
    // -----------------------------------------------------------------------

    // Extract type and width and height information from an image file.
    fn process_image(&mut self) -> Result<(), XlsxError> {
        let file = File::open(self.path.clone())?;
        let mut reader = BufReader::new(file);
        let mut data: Vec<u8> = vec![];
        reader.read_to_end(&mut data)?;

        let png_marker = &data[1..4];
        let jpg_marker = unpack_u16_from_be_bytes(&data, 0);
        let bmp_marker = &data[0..2];
        let gif_marker = &data[0..4];

        if png_marker == "PNG".as_bytes() {
            self.process_png(&data);
        } else if jpg_marker == 0xFFD8 {
            self.process_jpg(&data);
        } else if bmp_marker == "BM".as_bytes() {
            self.process_bmp(&data);
        } else if gif_marker == "GIF8".as_bytes() {
            self.process_gif(&data);
        }

        Ok(())
    }

    // Extract width and height information from a PNG file.
    fn process_png(&mut self, data: &[u8]) {
        let mut offset: usize = 8;
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut width_dpi: f64 = 96.0;
        let mut height_dpi: f64 = 96.0;
        let data_length = data.len();

        // Search through the image data to read the height and width in the
        // IHDR element. Also read the DPI in the pHYs element, if present.
        while offset < data_length {
            let marker = &data[offset + 4..offset + 8];
            let length = unpack_u32_from_be_bytes(data, offset);

            // Read the image dimensions.
            if marker == "IHDR".as_bytes() {
                width = unpack_u32_from_be_bytes(data, offset + 8);
                height = unpack_u32_from_be_bytes(data, offset + 12);
            }

            // Read the image DPI values.
            if marker == "pHYs".as_bytes() {
                let units = &data[offset + 16];
                let x_density = unpack_u32_from_be_bytes(data, offset + 8);
                let y_density = unpack_u32_from_be_bytes(data, offset + 12);

                if *units == 1 {
                    width_dpi = x_density as f64 * 0.0254;
                    height_dpi = y_density as f64 * 0.0254;
                    self.has_default_dpi = false;
                }
            }

            if marker == "IEND".as_bytes() {
                break;
            }

            offset = offset + length as usize + 12;
        }

        self.width = width as f64;
        self.height = height as f64;
        self.width_dpi = width_dpi;
        self.height_dpi = height_dpi;
        self.image_type = XlsxImageType::Png;
    }

    // Extract width and height information from a PNG file.
    fn process_jpg(&mut self, data: &[u8]) {
        let mut offset: usize = 2;
        let mut height: u32 = 0;
        let mut width: u32 = 0;
        let mut width_dpi: f64 = 96.0;
        let mut height_dpi: f64 = 96.0;
        let data_length = data.len();

        // Search through the image data to read the height and width in the
        // IHDR element. Also read the DPI in the pHYs element, if present.
        while offset < data_length {
            let marker = unpack_u16_from_be_bytes(data, offset);
            let length = unpack_u16_from_be_bytes(data, offset + 2);

            // Read the height and width in the 0xFFCn elements (except C4, C8
            // and CC which aren't SOF markers).
            if (marker & 0xFFF0) == 0xFFC0
                && marker != 0xFFC4
                && marker != 0xFFC8
                && marker != 0xFFCC
            {
                height = unpack_u16_from_be_bytes(data, offset + 5) as u32;
                width = unpack_u16_from_be_bytes(data, offset + 7) as u32;
            }

            // Read the DPI in the 0xFFE0 element.
            if marker == 0xFFE0 {
                let units = &data[offset + 11];
                let x_density = unpack_u16_from_be_bytes(data, offset + 12);
                let y_density = unpack_u16_from_be_bytes(data, offset + 14);

                if *units == 1 {
                    width_dpi = x_density as f64;
                    height_dpi = y_density as f64;
                }

                if *units == 2 {
                    width_dpi = x_density as f64 * 2.54;
                    height_dpi = y_density as f64 * 2.54;
                    self.has_default_dpi = false;
                }

                // Workaround for incorrect dpi.
                if width_dpi == 0.0 || width_dpi == 1.0 {
                    width_dpi = 96.0
                }
                if height_dpi == 0.0 || height_dpi == 1.0 {
                    height_dpi = 96.0
                }
            }

            if marker == 0xFFDA {
                break;
            }

            offset = offset + length as usize + 2;
        }

        self.width = width as f64;
        self.height = height as f64;
        self.width_dpi = width_dpi;
        self.height_dpi = height_dpi;
        self.image_type = XlsxImageType::Jpg;
    }

    // Extract width and height information from a BMP file.
    fn process_bmp(&mut self, data: &[u8]) {
        let width_dpi: f64 = 96.0;
        let height_dpi: f64 = 96.0;

        let width = unpack_u32_from_le_bytes(data, 18);
        let height = unpack_u32_from_le_bytes(data, 22);

        self.width = width as f64;
        self.height = height as f64;
        self.width_dpi = width_dpi;
        self.height_dpi = height_dpi;
        self.image_type = XlsxImageType::Bmp;
    }

    // Extract width and height information from a GIF file.
    fn process_gif(&mut self, data: &[u8]) {
        let width = unpack_u16_from_le_bytes(data, 6) as u32;
        let height = unpack_u16_from_le_bytes(data, 8) as u32;

        self.width = width as f64;
        self.height = height as f64;
        self.width_dpi = 96.0;
        self.height_dpi = 96.0;
        self.image_type = XlsxImageType::Gif;
    }
}

// -----------------------------------------------------------------------
// Helper enums/structs/functions.
// -----------------------------------------------------------------------
#[derive(Clone, Debug)]
pub(crate) enum XlsxImageType {
    Unknown,
    Png,
    Jpg,
    Gif,
    Bmp,
}

impl XlsxImageType {
    pub(crate) fn extension(&self) -> String {
        match self {
            XlsxImageType::Unknown => "unknown".to_string(),
            XlsxImageType::Png => "png".to_string(),
            XlsxImageType::Jpg => "jpeg".to_string(),
            XlsxImageType::Gif => "gif".to_string(),
            XlsxImageType::Bmp => "bmp".to_string(),
        }
    }
}

// Some helper functions to extract 2 and 4 byte integers from image data.
fn unpack_u16_from_be_bytes(data: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes(data[offset..offset + 2].try_into().unwrap())
}

fn unpack_u16_from_le_bytes(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(data[offset..offset + 2].try_into().unwrap())
}

fn unpack_u32_from_be_bytes(data: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap())
}

fn unpack_u32_from_le_bytes(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap())
}

// -----------------------------------------------------------------------
// Tests.
// -----------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use crate::XlsxError;

    use super::Image;

    #[test]
    fn test_images() {
        let image_test_data = vec![
            // Name, width, height, width_dpi, height_dpi, type.
            ("black_150.jpg", 64, 64, 150.0, 150.0, "jpeg"),
            ("black_300.jpg", 64, 64, 300.0, 300.0, "jpeg"),
            ("black_300.png", 64, 64, 299.9994, 299.9994, "png"),
            ("black_300e.png", 64, 64, 299.9994, 299.9994, "png"),
            ("black_72.jpg", 64, 64, 72.0, 72.0, "jpeg"),
            ("black_72.png", 64, 64, 72.009, 72.009, "png"),
            ("black_72e.png", 64, 64, 72.009, 72.009, "png"),
            ("black_96.jpg", 64, 64, 96.0, 96.0, "jpeg"),
            ("black_96.png", 64, 64, 96.012, 96.012, "png"),
            ("blue.jpg", 23, 23, 96.0, 96.0, "jpeg"),
            ("blue.png", 23, 23, 96.0, 96.0, "png"),
            ("grey.jpg", 99, 69, 96.0, 96.0, "jpeg"),
            ("grey.png", 99, 69, 96.0, 96.0, "png"),
            ("happy.jpg", 423, 563, 96.0, 96.0, "jpeg"),
            ("issue32.png", 115, 115, 96.0, 96.0, "png"),
            ("logo.gif", 200, 80, 96.0, 96.0, "gif"),
            ("logo.jpg", 200, 80, 96.0, 96.0, "jpeg"),
            ("logo.png", 200, 80, 96.0, 96.0, "png"),
            ("mylogo.png", 215, 36, 95.9866, 95.9866, "png"),
            ("red.bmp", 32, 32, 96.0, 96.0, "bmp"),
            ("red.gif", 32, 32, 96.0, 96.0, "gif"),
            ("red.jpg", 32, 32, 96.0, 96.0, "jpeg"),
            ("red.png", 32, 32, 96.0, 96.0, "png"),
            ("red2.png", 32, 32, 96.0, 96.0, "png"),
            ("red_208.png", 208, 49, 96.0, 96.0, "png"),
            ("red_64x20.png", 64, 20, 96.0, 96.0, "png"),
            ("red_readonly.png", 32, 32, 96.0, 96.0, "png"),
            ("train.jpg", 640, 480, 96.0, 96.0, "jpeg"),
            ("watermark.png", 1778, 1003, 329.9968, 329.9968, "png"),
            ("yellow.jpg", 72, 72, 96.0, 96.0, "jpeg"),
            ("yellow.png", 72, 72, 96.0, 96.0, "png"),
            ("zero_dpi.jpg", 11, 16, 96.0, 96.0, "jpeg"),
            (
                "black_150.png",
                64,
                64,
                150.01239999999999,
                150.01239999999999,
                "png",
            ),
            (
                "black_150e.png",
                64,
                64,
                150.01239999999999,
                150.01239999999999,
                "png",
            ),
        ];

        for test_data in image_test_data {
            let (filename, width, height, width_dpi, height_dpi, image_type) = test_data;
            let filename = format!("tests/input/images/{filename}");

            let image = Image::new(&filename).unwrap();
            assert_eq!(width as f64, image.width());
            assert_eq!(height as f64, image.height());
            assert_eq!(width_dpi, image.width_dpi());
            assert_eq!(height_dpi, image.height_dpi());
            assert_eq!(image_type, image.image_type.extension());
        }
    }

    #[test]
    fn unknown_file_format() {
        let filename = format!("tests/input/images/unknown.img");

        let image = Image::new(&filename);
        assert!(matches!(image, Err(XlsxError::UnknownImageType)));
    }

    #[test]
    fn invalid_file_format() {
        let filename = format!("tests/input/images/no_dimensions.png");

        let image = Image::new(&filename);
        assert!(matches!(image, Err(XlsxError::ImageDimensionError)));
    }
}
