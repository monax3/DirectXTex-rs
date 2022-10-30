use core::fmt;

use crate::{
    ScratchImage,
    TexMetadata,
    CMSE_FLAGS,
    CNMAP_FLAGS,
    CP_FLAGS,
    DDS_FLAGS,
    TEX_COMPRESS_FLAGS,
    TEX_DIMENSION,
    TEX_FILTER_FLAGS,
    TEX_PMALPHA_FLAGS,
    TGA_FLAGS,
    WIC_FLAGS,
};

impl Eq for TexMetadata {}

impl PartialEq for TexMetadata {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.height == other.height
            && self.depth == other.depth
            && self.arraySize == other.arraySize
            && self.mipLevels == other.mipLevels
            && self.miscFlags == other.miscFlags
            && self.miscFlags2 == other.miscFlags2
            && self.format == other.format
            && self.dimension == other.dimension
    }
}

impl fmt::Debug for ScratchImage {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScratchImage")
            .field("m_nimages", &self.m_nimages)
            .field("m_size", &self.m_size)
            .field("m_metadata", &self.m_metadata)
            .field("m_image", &self.m_image)
            .field("m_memory", &self.m_memory)
            .finish()
    }
}

impl fmt::Debug for TexMetadata {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TexMetadata")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("depth", &self.depth)
            .field("arraySize", &self.arraySize)
            .field("mipLevels", &self.mipLevels)
            .field("miscFlags", &self.miscFlags)
            .field("miscFlags2", &self.miscFlags2)
            .field("format", &self.format)
            .field("dimension", &self.dimension)
            .finish()
    }
}

impl fmt::Debug for TEX_DIMENSION {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TEX_DIMENSION::TEX_DIMENSION_TEXTURE1D => f.write_str("TEX_DIMENSION_TEXTURE1D"),
            TEX_DIMENSION::TEX_DIMENSION_TEXTURE2D => f.write_str("TEX_DIMENSION_TEXTURE2D"),
            TEX_DIMENSION::TEX_DIMENSION_TEXTURE3D => f.write_str("TEX_DIMENSION_TEXTURE3D"),
            unknown => write!(f, "TEX_DIMENSION(invalid {})", unknown.0),
        }
    }
}

impl Default for TEX_FILTER_FLAGS {
    #[inline]
    fn default() -> Self { TEX_FILTER_FLAGS::TEX_FILTER_DEFAULT }
}

impl Default for TEX_COMPRESS_FLAGS {
    #[inline]
    fn default() -> Self { TEX_COMPRESS_FLAGS::TEX_COMPRESS_DEFAULT }
}

impl Default for TEX_PMALPHA_FLAGS {
    #[inline]
    fn default() -> Self { TEX_PMALPHA_FLAGS::TEX_PMALPHA_DEFAULT }
}

impl Default for CNMAP_FLAGS {
    #[inline]
    fn default() -> Self { CNMAP_FLAGS::CNMAP_DEFAULT }
}

impl Default for CMSE_FLAGS {
    #[inline]
    fn default() -> Self { CMSE_FLAGS::CMSE_DEFAULT }
}

impl Default for DDS_FLAGS {
    #[inline]
    fn default() -> Self { DDS_FLAGS::DDS_FLAGS_NONE }
}

impl Default for CP_FLAGS {
    #[inline]
    fn default() -> Self { CP_FLAGS::CP_FLAGS_NONE }
}

impl Default for TGA_FLAGS {
    #[inline]
    fn default() -> Self { TGA_FLAGS::TGA_FLAGS_NONE }
}

impl Default for WIC_FLAGS {
    #[inline]
    fn default() -> Self { WIC_FLAGS::WIC_FLAGS_NONE }
}
