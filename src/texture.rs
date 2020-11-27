//! Textures

/// Texture
#[repr(transparent)]
pub struct Texture(u32);

impl Texture {
    pub fn open<P>(path: P) -> Result<Self, crate::BoxError>
    where
        P: AsRef<std::path::Path>,
    {
        let i = image::io::Reader::open(path)?.decode()?;

        Self::from_image(i)
    }

    /// Load PNG from [std::io::Read]
    pub fn load<R: std::io::Read + std::io::Seek>(r: &mut R) -> Result<Self, crate::BoxError> {
        let i = image::io::Reader::new(std::io::BufReader::new(r))
            .with_guessed_format()?
            .decode()?;

        Self::from_image(i)
    }

    fn from_image(img: image::DynamicImage) -> Result<Self, crate::BoxError> {
        let i = img.flipv();
        use image::DynamicImage::*;
        use std::ffi::c_void;
        #[rustfmt::skip]
        let (i_fmt, e_fmt, s, dim, data) = match i {
            ImageLuma8(img)   => (gl::R8     as i32, gl::RED,  gl::UNSIGNED_BYTE,  img.dimensions(), img.into_raw().as_slice().as_ptr() as *const c_void),
            ImageLumaA8(img)  => (gl::RG8    as i32, gl::RG,   gl::UNSIGNED_BYTE,  img.dimensions(), img.into_raw().as_slice().as_ptr() as *const c_void),
            ImageRgb8(img)    => (gl::RGB8   as i32, gl::RGB,  gl::UNSIGNED_BYTE,  img.dimensions(), img.into_raw().as_slice().as_ptr() as *const c_void),
            ImageRgba8(img)   => (gl::RGBA8  as i32, gl::RGBA, gl::UNSIGNED_BYTE,  img.dimensions(), img.into_raw().as_slice().as_ptr() as *const c_void),
            ImageBgr8(img)    => (gl::RGB8   as i32, gl::BGR,  gl::UNSIGNED_BYTE,  img.dimensions(), img.into_raw().as_slice().as_ptr() as *const c_void),
            ImageBgra8(img)   => (gl::RGBA8  as i32, gl::BGRA, gl::UNSIGNED_BYTE,  img.dimensions(), img.into_raw().as_slice().as_ptr() as *const c_void),
            ImageLuma16(img)  => (gl::R16    as i32, gl::RED,  gl::UNSIGNED_SHORT, img.dimensions(), img.into_raw().as_slice().as_ptr() as *const c_void),
            ImageLumaA16(img) => (gl::RG16   as i32, gl::RG,   gl::UNSIGNED_SHORT, img.dimensions(), img.into_raw().as_slice().as_ptr() as *const c_void),
            ImageRgb16(img)   => (gl::RGB16  as i32, gl::RGB,  gl::UNSIGNED_SHORT, img.dimensions(), img.into_raw().as_slice().as_ptr() as *const c_void),
            ImageRgba16(img)  => (gl::RGBA16 as i32, gl::RGBA, gl::UNSIGNED_SHORT, img.dimensions(), img.into_raw().as_slice().as_ptr() as *const c_void),
        };

        unsafe {
            let mut id: u32 = 0;
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                i_fmt,
                dim.0 as i32,
                dim.1 as i32,
                0,
                e_fmt,
                gl::UNSIGNED_BYTE,
                data,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            Ok(Self(id))
        }
    }

    pub fn bind(&self, n: u32) {
        let mut value: i32 = 0;
        unsafe {
            gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut value);
        }
        if n < value as u32 {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + n);
                gl::BindTexture(gl::TEXTURE_2D, self.0);
            }
        }
    }
}
