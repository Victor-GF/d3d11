//! Tipos comuns a todas as versoes do Direct3D (D3D10, D3D11, D3D12) e ao
//! compilador de shaders: tipo de driver, feature level, topologia, blob.

#![allow(non_upper_case_globals)]

use super::basetsd::SIZE_T;
use super::guiddef::IID;
use super::minwindef::LPVOID;
use super::unknwn::IUnknownVtbl;
use super::winnt::LPCSTR;
use crate::layout;

// ---- D3D_DRIVER_TYPE ------------------------------------------------------

/// Enum do SDK.
pub type D3D_DRIVER_TYPE = i32;
pub const D3D_DRIVER_TYPE_UNKNOWN: D3D_DRIVER_TYPE = 0;
/// GPU de verdade.
pub const D3D_DRIVER_TYPE_HARDWARE: D3D_DRIVER_TYPE = 1;
pub const D3D_DRIVER_TYPE_REFERENCE: D3D_DRIVER_TYPE = 2;
pub const D3D_DRIVER_TYPE_NULL: D3D_DRIVER_TYPE = 3;
pub const D3D_DRIVER_TYPE_SOFTWARE: D3D_DRIVER_TYPE = 4;
/// Rasterizador em software da Microsoft (roda sem GPU).
pub const D3D_DRIVER_TYPE_WARP: D3D_DRIVER_TYPE = 5;

// ---- D3D_FEATURE_LEVEL ----------------------------------------------------

/// Enum do SDK. O valor codifica a versao: `0xb000` = 11.0, `0xb100` = 11.1.
pub type D3D_FEATURE_LEVEL = i32;
pub const D3D_FEATURE_LEVEL_1_0_GENERIC: D3D_FEATURE_LEVEL = 0x100;
pub const D3D_FEATURE_LEVEL_1_0_CORE: D3D_FEATURE_LEVEL = 0x1000;
pub const D3D_FEATURE_LEVEL_9_1: D3D_FEATURE_LEVEL = 0x9100;
pub const D3D_FEATURE_LEVEL_9_2: D3D_FEATURE_LEVEL = 0x9200;
pub const D3D_FEATURE_LEVEL_9_3: D3D_FEATURE_LEVEL = 0x9300;
pub const D3D_FEATURE_LEVEL_10_0: D3D_FEATURE_LEVEL = 0xa000;
pub const D3D_FEATURE_LEVEL_10_1: D3D_FEATURE_LEVEL = 0xa100;
pub const D3D_FEATURE_LEVEL_11_0: D3D_FEATURE_LEVEL = 0xb000;
pub const D3D_FEATURE_LEVEL_11_1: D3D_FEATURE_LEVEL = 0xb100;
pub const D3D_FEATURE_LEVEL_12_0: D3D_FEATURE_LEVEL = 0xc000;
pub const D3D_FEATURE_LEVEL_12_1: D3D_FEATURE_LEVEL = 0xc100;
pub const D3D_FEATURE_LEVEL_12_2: D3D_FEATURE_LEVEL = 0xc200;

// ---- D3D_PRIMITIVE_TOPOLOGY -----------------------------------------------

/// Enum do SDK; so as topologias basicas (sem `_ADJ` e sem patch lists).
/// Os nomes `D3D11_` sao membros do mesmo enum no header, com o mesmo valor.
pub type D3D_PRIMITIVE_TOPOLOGY = i32;
pub const D3D_PRIMITIVE_TOPOLOGY_UNDEFINED: D3D_PRIMITIVE_TOPOLOGY = 0;
pub const D3D_PRIMITIVE_TOPOLOGY_POINTLIST: D3D_PRIMITIVE_TOPOLOGY = 1;
pub const D3D_PRIMITIVE_TOPOLOGY_LINELIST: D3D_PRIMITIVE_TOPOLOGY = 2;
pub const D3D_PRIMITIVE_TOPOLOGY_LINESTRIP: D3D_PRIMITIVE_TOPOLOGY = 3;
pub const D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST: D3D_PRIMITIVE_TOPOLOGY = 4;
pub const D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP: D3D_PRIMITIVE_TOPOLOGY = 5;

pub const D3D11_PRIMITIVE_TOPOLOGY_UNDEFINED: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY_UNDEFINED;
pub const D3D11_PRIMITIVE_TOPOLOGY_POINTLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY_POINTLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_LINELIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY_LINELIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_LINESTRIP: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY_LINESTRIP;
pub const D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP;

// ---- D3D_SRV_DIMENSION ----------------------------------------------------

/// Enum do SDK: tipo de recurso visto por uma shader resource view. Os nomes
/// `D3D11_` sao membros do mesmo enum no header, com o mesmo valor (os
/// `D3D10_`/`D3D10_1_` ficaram de fora).
pub type D3D_SRV_DIMENSION = i32;
pub const D3D_SRV_DIMENSION_UNKNOWN: D3D_SRV_DIMENSION = 0;
pub const D3D_SRV_DIMENSION_BUFFER: D3D_SRV_DIMENSION = 1;
pub const D3D_SRV_DIMENSION_TEXTURE1D: D3D_SRV_DIMENSION = 2;
pub const D3D_SRV_DIMENSION_TEXTURE1DARRAY: D3D_SRV_DIMENSION = 3;
pub const D3D_SRV_DIMENSION_TEXTURE2D: D3D_SRV_DIMENSION = 4;
pub const D3D_SRV_DIMENSION_TEXTURE2DARRAY: D3D_SRV_DIMENSION = 5;
pub const D3D_SRV_DIMENSION_TEXTURE2DMS: D3D_SRV_DIMENSION = 6;
pub const D3D_SRV_DIMENSION_TEXTURE2DMSARRAY: D3D_SRV_DIMENSION = 7;
pub const D3D_SRV_DIMENSION_TEXTURE3D: D3D_SRV_DIMENSION = 8;
pub const D3D_SRV_DIMENSION_TEXTURECUBE: D3D_SRV_DIMENSION = 9;
pub const D3D_SRV_DIMENSION_TEXTURECUBEARRAY: D3D_SRV_DIMENSION = 10;
pub const D3D_SRV_DIMENSION_BUFFEREX: D3D_SRV_DIMENSION = 11;
pub const D3D11_SRV_DIMENSION_UNKNOWN: D3D_SRV_DIMENSION = 0;
pub const D3D11_SRV_DIMENSION_BUFFER: D3D_SRV_DIMENSION = 1;
pub const D3D11_SRV_DIMENSION_TEXTURE1D: D3D_SRV_DIMENSION = 2;
pub const D3D11_SRV_DIMENSION_TEXTURE1DARRAY: D3D_SRV_DIMENSION = 3;
pub const D3D11_SRV_DIMENSION_TEXTURE2D: D3D_SRV_DIMENSION = 4;
pub const D3D11_SRV_DIMENSION_TEXTURE2DARRAY: D3D_SRV_DIMENSION = 5;
pub const D3D11_SRV_DIMENSION_TEXTURE2DMS: D3D_SRV_DIMENSION = 6;
pub const D3D11_SRV_DIMENSION_TEXTURE2DMSARRAY: D3D_SRV_DIMENSION = 7;
pub const D3D11_SRV_DIMENSION_TEXTURE3D: D3D_SRV_DIMENSION = 8;
pub const D3D11_SRV_DIMENSION_TEXTURECUBE: D3D_SRV_DIMENSION = 9;
pub const D3D11_SRV_DIMENSION_TEXTURECUBEARRAY: D3D_SRV_DIMENSION = 10;
pub const D3D11_SRV_DIMENSION_BUFFEREX: D3D_SRV_DIMENSION = 11;

// ---- Shader macros / include ----------------------------------------------

/// `#define Name Definition` passado ao compilador. O array termina com um
/// item `{ null, null }`.
#[repr(C)]
pub struct D3D_SHADER_MACRO {
    pub Name: LPCSTR,
    pub Definition: LPCSTR,
}

layout!(D3D_SHADER_MACRO, size = 16, align = 8);

pub type LPD3D_SHADER_MACRO = *mut D3D_SHADER_MACRO;

/// Enum do SDK.
pub type D3D_INCLUDE_TYPE = i32;
pub const D3D_INCLUDE_LOCAL: D3D_INCLUDE_TYPE = 0;
pub const D3D_INCLUDE_SYSTEM: D3D_INCLUDE_TYPE = 1;

/// Callback de `#include` do compilador. Nao e COM de verdade: a vtable nao
/// comeca com `IUnknown`, so tem `Open` e `Close`.
#[repr(C)]
pub struct ID3DInclude {
    pub lpVtbl: *const ID3DIncludeVtbl,
}

#[repr(C)]
pub struct ID3DIncludeVtbl {
    pub Open: usize,
    pub Close: usize,
}

layout!(ID3DIncludeVtbl, size = 16, align = 8);

pub type LPD3DINCLUDE = *mut ID3DInclude;

// ---- ID3D10Blob / ID3DBlob ------------------------------------------------

/// `{8BA5FB08-5195-40E2-AC58-0D989C3A0102}`
pub const IID_ID3D10Blob: IID = IID {
    Data1: 0x8BA5_FB08,
    Data2: 0x5195,
    Data3: 0x40E2,
    Data4: [0xAC, 0x58, 0x0D, 0x98, 0x9C, 0x3A, 0x01, 0x02],
};

/// Buffer de bytes com contagem de referencia: bytecode de shader compilado,
/// mensagens de erro do compilador.
#[repr(C)]
pub struct ID3D10Blob {
    pub lpVtbl: *const ID3D10BlobVtbl,
}

#[repr(C)]
pub struct ID3D10BlobVtbl {
    pub base: IUnknownVtbl,
    pub GetBufferPointer: unsafe extern "system" fn(This: *mut ID3D10Blob) -> LPVOID,
    pub GetBufferSize: unsafe extern "system" fn(This: *mut ID3D10Blob) -> SIZE_T,
}

layout!(ID3D10BlobVtbl, size = 40, align = 8);

pub type LPD3D10BLOB = *mut ID3D10Blob;

pub type ID3DBlob = ID3D10Blob;
pub type LPD3DBLOB = *mut ID3DBlob;
