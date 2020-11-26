//! Textures

/// Texture
pub struct Texture {
	/// PNG info
	info: png::OutputInfo,
	/// PNG data
	data: Vec<u8>
}

impl Texture {
	// The region that has been written be checked afterwards
	// by calling info after a successful call and inspecting the frame_control data.
	// This requirement may be lifted in a later version of png.
	//
	// All samples are in big endian byte order where this matters.
	/// Load PNG from [std::io::Read]
	pub fn load<R: std::io::Read>(r: &mut R) -> Result<Self, crate::BoxError> {
		let (info, mut reader) = png::Decoder::new(r).read_info()?;
		if Self::check_gl_sized_internal_format(&info.color_type, &info.bit_depth) == 0 {
			return Err(png::DecodingError::Other("Unsupported format".into()).into());
		}
		let mut data = vec![0; info.buffer_size()];
		reader.next_frame(&mut data)?;
		Ok(Self { info, data })
	}

    // TODO make this a struct later
    pub fn to_gl(self) -> u32 {
        unsafe {
            let mut id: u32 = 0;
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, self.gl_sized_internal_format(), self.size().0 as i32, self.size().1 as i32, 0, self.gl_format(), self.gl_type(), self.data.as_slice().as_ptr() as *const _);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            id
        }
    }

	/// Get OpenGL format as [i32]
	pub fn gl_sized_internal_format(&self) -> i32 {
		Self::check_gl_sized_internal_format(&self.info.color_type, &self.info.bit_depth)
	}

    pub fn gl_type(&self) -> u32 {
        gl::UNSIGNED_BYTE
    }

    pub fn gl_format(&self) -> u32 {
        use png::ColorType;
        match self.info.color_type {
            ColorType::Grayscale => gl::RED,
            ColorType::RGB       => gl::RGB,
            ColorType::GrayscaleAlpha => gl::RG,
            ColorType::RGBA      => gl::RGBA,
            _ => 0
        }
    }

	/// Get OpenGL format as [i32]
	#[rustfmt::skip]
	fn check_gl_sized_internal_format(color_type: &png::ColorType, bit_depth: &png::BitDepth) -> i32 {
		use png::{BitDepth, ColorType};
		match (&color_type, &bit_depth) {
			(ColorType::Grayscale,      BitDepth::Eight  ) => { gl::R8    as i32 }
			(ColorType::RGB,            BitDepth::Eight  ) => { gl::RGB8  as i32 }
			(ColorType::GrayscaleAlpha, BitDepth::Eight  ) => { gl::RG8   as i32 }
			(ColorType::RGBA,           BitDepth::Eight  ) => { gl::RGBA8 as i32 }
            _ => 0
		}
	}

	/// Get size of image
	pub fn size(&self) -> (u32, u32) {
		(self.info.width, self.info.height)
	}

	/// Get pitch of image
	pub fn pitch(&self) -> usize {
		self.info.line_size
	}

	/// Get image data
	pub fn data(&self) -> &[u8] {
		&self.data
	}
}
