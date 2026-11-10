//! `dxgicommon.h`: structs pequenas usadas por DXGI e Direct3D.

use super::minwindef::UINT;
use crate::layout;

/// Fracao: taxa de atualizacao 60 Hz = `{ 60, 1 }`. `{ 0, 0 }` = qualquer uma.
#[repr(C)]
pub struct DXGI_RATIONAL {
    pub Numerator: UINT,
    pub Denominator: UINT,
}

layout!(DXGI_RATIONAL, size = 8, align = 4);

/// Multisampling (MSAA). `{ 1, 0 }` = sem MSAA.
#[repr(C)]
pub struct DXGI_SAMPLE_DESC {
    pub Count: UINT,
    pub Quality: UINT,
}

layout!(DXGI_SAMPLE_DESC, size = 8, align = 4);
