use sealed::sealed;

use crate::sl;

use super::raw::{self, ImageInternalFormat, ImageType};

#[sealed]
pub trait ImageData {
    #[doc(hidden)]
    fn raw(&self) -> &raw::ImageData;
}

#[sealed]
pub trait ImageFormat {
    type Sample: sl::Sample;
    type Data<'a>: ImageData;
}

pub struct RgbaData<'a>(raw::ImageData<'a>);

#[sealed]
impl<'a> ImageData for RgbaData<'a> {
    fn raw(&self) -> &raw::ImageData {
        &self.0
    }
}

impl<'a> RgbaData<'a> {
    pub fn from_u8(dimensions: (u32, u32), data: &'a [u8]) -> Self {
        RgbaData(raw::ImageData {
            dimensions,
            ty: ImageType::U8,
            internal_format: ImageInternalFormat::RgbaU8,
            data: Some(data),
        })
    }

    pub fn from_u8_srgb(dimensions: (u32, u32), data: &'a [u8]) -> Self {
        RgbaData(raw::ImageData {
            dimensions,
            ty: ImageType::U8,
            internal_format: ImageInternalFormat::SrgbU8AlphaU8,
            data: Some(data),
        })
    }

    pub fn from_i8_snorm(dimensions: (u32, u32), data: &'a [i8]) -> Self {
        RgbaData(raw::ImageData {
            dimensions,
            ty: ImageType::I8,
            internal_format: ImageInternalFormat::RgbaI8Snorm,
            data: Some(bytemuck::cast_slice(data)),
        })
    }

    pub fn from_f32(dimensions: (u32, u32), data: &'a [f32]) -> Self {
        RgbaData(raw::ImageData {
            dimensions,
            ty: ImageType::F32,
            internal_format: ImageInternalFormat::RgbaF32,
            data: Some(bytemuck::cast_slice(data)),
        })
    }

    pub fn zeroed_u8(dimensions: (u32, u32)) -> Self {
        RgbaData(raw::ImageData {
            dimensions,
            ty: ImageType::U8,
            internal_format: ImageInternalFormat::RgbaU8,
            data: None,
        })
    }

    pub fn zeroed_u8_srgb(dimensions: (u32, u32)) -> Self {
        RgbaData(raw::ImageData {
            dimensions,
            ty: ImageType::U8,
            internal_format: ImageInternalFormat::SrgbU8AlphaU8,
            data: None,
        })
    }

    pub fn zeroed_i8_snorm(dimensions: (u32, u32)) -> Self {
        RgbaData(raw::ImageData {
            dimensions,
            ty: ImageType::I8,
            internal_format: ImageInternalFormat::RgbaI8Snorm,
            data: None,
        })
    }

    pub fn zeroed_f32(dimensions: (u32, u32)) -> Self {
        RgbaData(raw::ImageData {
            dimensions,
            ty: ImageType::F32,
            internal_format: ImageInternalFormat::RgbaF32,
            data: None,
        })
    }
}

pub struct RgbaFormat;

#[sealed]
impl ImageFormat for RgbaFormat {
    type Sample = sl::Vec4<f32>;
    type Data<'a> = RgbaData<'a>;
}

// TODO:
// - RgbaIntFormat
// - RgbaUnsignedIntFormat
// - RgbFormat
// - RgbIntFormat
// - RgbUnsignedIntFormat
// - RgFormat
// - RgIntFormat
// - RgUnsignedIntFormat
// - RedFormat
// - RedIntFormat
// - RedUnsignedIntFormat
// - DepthFormat
// - DepthStencilFormat

/*

RGBA - UNSIGNED_BYTE - RGBA8, SRGB8_ALPHA8
RGBA - BYTE          - RGBA8_SNORM
RGBA - FLOAT         - RGBA32F, RGBA16F

RGB  - UNSIGNED_BYTE - RGB8, SRGB8
RGB  - BYTE          - RGB8_SNORM
RGB  - FLOAT         - RGB32, RGB16F

RG   - UNSIGNED_BYTE - RG8
RG   - BYTE          - RG8_SNORM
RG   - FLOAT         - RG32F, RG16F

RED  - UNSIGNED_BYTE - R8
RED  - BYTE          - R8_SNORM
RED  - FLOAT         - R32F, R16F

RGBA_INTEGER - UNSIGNED_BYTE - RGBA8UI
RGBA_INTEGER - BYTE          - RGBA8I
...

RGB_INTEGER  - UNSIGNED_BYTE  - RGB8UI
RGB_INTEGER  - BYTE           - RGB8I
...

RG_INTEGER   - UNSIGNED_BYTE  - RG8UI
RG_INTEGER   - BYTE           - RG 8I
...

RED_INTEGER  - UNSIGNED_BYTE  - R8UI
RED_INTEGER  - BYTE           - R8I
...

DEPTH_COMPONENT - UNSIGNED_SHORT - DEPTH_COMPONENT16
DEPTH_COMPONENT - UNSIGNED_INT   - DEPTH_COMPONENT24
DEPTH_COMPONENT - FLOAT          - DEPTH_COMPONENT32F

DEPTH_STENCIL   - UNSIGNED_INT_24_8 - DEPTH24_STENCIL8

*/