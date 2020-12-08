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

    fn into_v8(v: Vec<u16>) -> Vec<u8> {
        // Use into_raw_parts when it's stabilized
        let mut me = std::mem::ManuallyDrop::new(v);
        unsafe { Vec::from_raw_parts(me.as_mut_ptr() as *mut u8, me.len(), me.capacity()) }
    }

    fn from_image(img: image::DynamicImage) -> Result<Self, crate::BoxError> {
        let i = img.flipv();
        use image::DynamicImage::*;
        use std::ffi::c_void;
        #[rustfmt::skip]
        let (i_fmt, e_fmt, s, dim, data) = match i {
            ImageLuma8(img)   => (gl::R8     as i32, gl::RED,  gl::UNSIGNED_BYTE,  img.dimensions(), img.into_raw()),
            ImageLumaA8(img)  => (gl::RG8    as i32, gl::RG,   gl::UNSIGNED_BYTE,  img.dimensions(), img.into_raw()),
            ImageRgb8(img)    => (gl::RGB8   as i32, gl::RGB,  gl::UNSIGNED_BYTE,  img.dimensions(), img.into_raw()),
            ImageRgba8(img)   => (gl::RGBA8  as i32, gl::RGBA, gl::UNSIGNED_BYTE,  img.dimensions(), img.into_raw()),
            ImageBgr8(img)    => (gl::RGB8   as i32, gl::BGR,  gl::UNSIGNED_BYTE,  img.dimensions(), img.into_raw()),
            ImageBgra8(img)   => (gl::RGBA8  as i32, gl::BGRA, gl::UNSIGNED_BYTE,  img.dimensions(), img.into_raw()),
            ImageLuma16(img)  => (gl::R16    as i32, gl::RED,  gl::UNSIGNED_SHORT, img.dimensions(), Self::into_v8(img.into_raw())),
            ImageLumaA16(img) => (gl::RG16   as i32, gl::RG,   gl::UNSIGNED_SHORT, img.dimensions(), Self::into_v8(img.into_raw())),
            ImageRgb16(img)   => (gl::RGB16  as i32, gl::RGB,  gl::UNSIGNED_SHORT, img.dimensions(), Self::into_v8(img.into_raw())),
            ImageRgba16(img)  => (gl::RGBA16 as i32, gl::RGBA, gl::UNSIGNED_SHORT, img.dimensions(), Self::into_v8(img.into_raw())),
        };

        unsafe {
            let mut id: u32 = 0;
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                i_fmt,
                dim.0 as i32,
                dim.1 as i32,
                0,
                e_fmt,
                gl::UNSIGNED_BYTE,
                data.as_slice().as_ptr() as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            let mut aniso = 0.0;
            gl::GetFloatv(0x84FF, &mut aniso); // Preferrably use the enums in the future
            gl::TexParameterf(gl::TEXTURE_2D, 0x84FE, aniso); // It's anisotropy extension

            gl::BindTexture(gl::TEXTURE_2D, 0);
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

// TODO impl Drop

#[repr(transparent)]
pub struct Cubemap(u32);

impl Cubemap {
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

    fn into_v8(v: Vec<u16>) -> Vec<u8> {
        // Use into_raw_parts when it's stabilized
        let mut me = std::mem::ManuallyDrop::new(v);
        unsafe { Vec::from_raw_parts(me.as_mut_ptr() as *mut u8, me.len(), me.capacity()) }
    }

    fn from_image(img: image::DynamicImage) -> Result<Self, crate::BoxError> {
        let i = img;
        use image::DynamicImage::*;
        use std::ffi::c_void;
        #[rustfmt::skip]
        let (i_fmt, e_fmt, s, dim, data) = match i {
            ImageLuma8(img)   => (gl::R8     as i32, gl::RED,  gl::UNSIGNED_BYTE,  img.dimensions(), img.into_raw()),
            ImageLumaA8(img)  => (gl::RG8    as i32, gl::RG,   gl::UNSIGNED_BYTE,  img.dimensions(), img.into_raw()),
            ImageRgb8(img)    => (gl::RGB8   as i32, gl::RGB,  gl::UNSIGNED_BYTE,  img.dimensions(), img.into_raw()),
            ImageRgba8(img)   => (gl::RGBA8  as i32, gl::RGBA, gl::UNSIGNED_BYTE,  img.dimensions(), img.into_raw()),
            ImageBgr8(img)    => (gl::RGB8   as i32, gl::BGR,  gl::UNSIGNED_BYTE,  img.dimensions(), img.into_raw()),
            ImageBgra8(img)   => (gl::RGBA8  as i32, gl::BGRA, gl::UNSIGNED_BYTE,  img.dimensions(), img.into_raw()),
            ImageLuma16(img)  => (gl::R16    as i32, gl::RED,  gl::UNSIGNED_SHORT, img.dimensions(), Self::into_v8(img.into_raw())),
            ImageLumaA16(img) => (gl::RG16   as i32, gl::RG,   gl::UNSIGNED_SHORT, img.dimensions(), Self::into_v8(img.into_raw())),
            ImageRgb16(img)   => (gl::RGB16  as i32, gl::RGB,  gl::UNSIGNED_SHORT, img.dimensions(), Self::into_v8(img.into_raw())),
            ImageRgba16(img)  => (gl::RGBA16 as i32, gl::RGBA, gl::UNSIGNED_SHORT, img.dimensions(), Self::into_v8(img.into_raw())),
        };

        unsafe {
            let mut id: u32 = 0;
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, id);
            let base = data.as_slice().as_ptr() as *const c_void;
            let offset: isize = (data.len() as isize) / 6;
            for n in 0isize..6isize {
                gl::TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + (n as u32),
                    0,
                    i_fmt,
                    dim.0 as i32,
                    dim.1 as i32 / 6,
                    0,
                    e_fmt,
                    gl::UNSIGNED_BYTE,
                    base.offset(n * offset),
                );
            }
            //gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_R,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32, // No mipmap yet
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
            );
            let mut aniso = 0.0;
            gl::GetFloatv(0x84FF, &mut aniso); // Preferrably use the enums in the future
            gl::TexParameterf(gl::TEXTURE_CUBE_MAP, 0x84FE, aniso); // It's anisotropy extension

            gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
            Ok(Self(id))
        }
    }

    pub fn bind(&self, n: u32) {
        let mut value: i32 = 0;
        unsafe {
            // Not sure if this applies to cubemaps
            gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut value);
        }
        if n < value as u32 {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + n);
                gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.0);
            }
        }
    }
}

// TODO impl Drop
