//! `dxgitype.h`: descricao de modo de video (resolucao, formato, refresh).

use super::dxgicommon::DXGI_RATIONAL;
use super::dxgiformat::DXGI_FORMAT;
use super::minwindef::UINT;
use crate::layout;

/// Enum do SDK.
pub type DXGI_MODE_SCANLINE_ORDER = i32;
pub const DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED: DXGI_MODE_SCANLINE_ORDER = 0;
pub const DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE: DXGI_MODE_SCANLINE_ORDER = 1;
pub const DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST: DXGI_MODE_SCANLINE_ORDER = 2;
pub const DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST: DXGI_MODE_SCANLINE_ORDER = 3;

/// Enum do SDK.
pub type DXGI_MODE_SCALING = i32;
pub const DXGI_MODE_SCALING_UNSPECIFIED: DXGI_MODE_SCALING = 0;
pub const DXGI_MODE_SCALING_CENTERED: DXGI_MODE_SCALING = 1;
pub const DXGI_MODE_SCALING_STRETCHED: DXGI_MODE_SCALING = 2;

/// `Width`/`Height` = 0: o DXGI usa o tamanho da area cliente da janela.
#[repr(C)]
pub struct DXGI_MODE_DESC {
    pub Width: UINT,
    pub Height: UINT,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
}

layout!(DXGI_MODE_DESC, size = 28, align = 4);
