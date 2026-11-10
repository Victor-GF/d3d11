//! `d3d11.h`: device, device context, recursos, views, shaders e states.
//!
//! Ordem dos metodos das vtables, valores dos enums e IIDs foram extraidos
//! do header do SDK (10.0.26100.0). Tamanhos das structs conferidos contra
//! `sizeof` em C.
//!
//! Metodos ainda nao usados ficam como `usize` (so ocupam a posicao). Para
//! usar um, troque o `usize` pela assinatura do header.
//!
//! COM nao tem heranca em Rust: para passar um `*mut ID3D11Texture2D` onde
//! se espera `*mut ID3D11Resource`, faca o cast (`tex as *mut ID3D11Resource`).
//! E valido porque a vtable do filho comeca com a do pai.

#![allow(non_upper_case_globals)]

use super::basetsd::SIZE_T;
use super::d3dcommon::{D3D_DRIVER_TYPE, D3D_FEATURE_LEVEL, D3D_PRIMITIVE_TOPOLOGY, D3D_SRV_DIMENSION};
use super::dxgi::{IDXGIAdapter, IDXGISwapChain, DXGI_SWAP_CHAIN_DESC};
use super::dxgicommon::DXGI_SAMPLE_DESC;
use super::dxgiformat::DXGI_FORMAT;
use super::guiddef::IID;
use super::minwindef::{BOOL, FLOAT, HMODULE, INT, UINT};
use super::unknwn::IUnknownVtbl;
use super::winnt::{HRESULT, LPCSTR};
use core::ffi::c_void;
use crate::layout;

pub type UINT8 = u8;

pub const D3D11_SDK_VERSION: UINT = 7;

/// `AlignedByteOffset`: logo depois do elemento anterior.
pub const D3D11_APPEND_ALIGNED_ELEMENT: UINT = 0xFFFF_FFFF;

/// `typedef D3D_PRIMITIVE_TOPOLOGY D3D11_PRIMITIVE_TOPOLOGY;` (valores em d3dcommon).
pub type D3D11_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY;
/// `typedef D3D_SRV_DIMENSION D3D11_SRV_DIMENSION;` (valores em d3dcommon).
pub type D3D11_SRV_DIMENSION = D3D_SRV_DIMENSION;

// ---- Enums ------------------------------------------------------------------

/// Enum do SDK, usado como flags no parametro `Flags` (`UINT`).
pub type D3D11_CREATE_DEVICE_FLAG = i32;
pub const D3D11_CREATE_DEVICE_SINGLETHREADED: D3D11_CREATE_DEVICE_FLAG = 0x1;
pub const D3D11_CREATE_DEVICE_DEBUG: D3D11_CREATE_DEVICE_FLAG = 0x2;
pub const D3D11_CREATE_DEVICE_SWITCH_TO_REF: D3D11_CREATE_DEVICE_FLAG = 0x4;
pub const D3D11_CREATE_DEVICE_PREVENT_INTERNAL_THREADING_OPTIMIZATIONS: D3D11_CREATE_DEVICE_FLAG = 0x8;
pub const D3D11_CREATE_DEVICE_BGRA_SUPPORT: D3D11_CREATE_DEVICE_FLAG = 0x20;
pub const D3D11_CREATE_DEVICE_DEBUGGABLE: D3D11_CREATE_DEVICE_FLAG = 0x40;
pub const D3D11_CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY: D3D11_CREATE_DEVICE_FLAG = 0x80;
pub const D3D11_CREATE_DEVICE_DISABLE_GPU_TIMEOUT: D3D11_CREATE_DEVICE_FLAG = 0x100;
pub const D3D11_CREATE_DEVICE_VIDEO_SUPPORT: D3D11_CREATE_DEVICE_FLAG = 0x800;

/// Enum do SDK: quem le/escreve o recurso (GPU, CPU) e com que frequencia.
pub type D3D11_USAGE = i32;
pub const D3D11_USAGE_DEFAULT: D3D11_USAGE = 0;
pub const D3D11_USAGE_IMMUTABLE: D3D11_USAGE = 1;
pub const D3D11_USAGE_DYNAMIC: D3D11_USAGE = 2;
pub const D3D11_USAGE_STAGING: D3D11_USAGE = 3;

/// Enum do SDK, usado como flags em `BindFlags` (`UINT`).
pub type D3D11_BIND_FLAG = i32;
pub const D3D11_BIND_VERTEX_BUFFER: D3D11_BIND_FLAG = 0x1;
pub const D3D11_BIND_INDEX_BUFFER: D3D11_BIND_FLAG = 0x2;
pub const D3D11_BIND_CONSTANT_BUFFER: D3D11_BIND_FLAG = 0x4;
pub const D3D11_BIND_SHADER_RESOURCE: D3D11_BIND_FLAG = 0x8;
pub const D3D11_BIND_STREAM_OUTPUT: D3D11_BIND_FLAG = 0x10;
pub const D3D11_BIND_RENDER_TARGET: D3D11_BIND_FLAG = 0x20;
pub const D3D11_BIND_DEPTH_STENCIL: D3D11_BIND_FLAG = 0x40;
pub const D3D11_BIND_UNORDERED_ACCESS: D3D11_BIND_FLAG = 0x80;
pub const D3D11_BIND_DECODER: D3D11_BIND_FLAG = 0x200;
pub const D3D11_BIND_VIDEO_ENCODER: D3D11_BIND_FLAG = 0x400;

/// Enum do SDK, usado como flags em `CPUAccessFlags` (`UINT`).
pub type D3D11_CPU_ACCESS_FLAG = i32;
pub const D3D11_CPU_ACCESS_WRITE: D3D11_CPU_ACCESS_FLAG = 0x10000;
pub const D3D11_CPU_ACCESS_READ: D3D11_CPU_ACCESS_FLAG = 0x20000;

/// Enum do SDK.
pub type D3D11_MAP = i32;
pub const D3D11_MAP_READ: D3D11_MAP = 1;
pub const D3D11_MAP_WRITE: D3D11_MAP = 2;
pub const D3D11_MAP_READ_WRITE: D3D11_MAP = 3;
pub const D3D11_MAP_WRITE_DISCARD: D3D11_MAP = 4;
pub const D3D11_MAP_WRITE_NO_OVERWRITE: D3D11_MAP = 5;

/// Enum do SDK, usado como flags em `ClearFlags` (`UINT`).
pub type D3D11_CLEAR_FLAG = i32;
pub const D3D11_CLEAR_DEPTH: D3D11_CLEAR_FLAG = 0x1;
pub const D3D11_CLEAR_STENCIL: D3D11_CLEAR_FLAG = 0x2;

/// Enum do SDK.
pub type D3D11_INPUT_CLASSIFICATION = i32;
pub const D3D11_INPUT_PER_VERTEX_DATA: D3D11_INPUT_CLASSIFICATION = 0;
pub const D3D11_INPUT_PER_INSTANCE_DATA: D3D11_INPUT_CLASSIFICATION = 1;

/// Enum do SDK.
pub type D3D11_FILL_MODE = i32;
pub const D3D11_FILL_WIREFRAME: D3D11_FILL_MODE = 2;
pub const D3D11_FILL_SOLID: D3D11_FILL_MODE = 3;

/// Enum do SDK.
pub type D3D11_CULL_MODE = i32;
pub const D3D11_CULL_NONE: D3D11_CULL_MODE = 1;
pub const D3D11_CULL_FRONT: D3D11_CULL_MODE = 2;
pub const D3D11_CULL_BACK: D3D11_CULL_MODE = 3;

/// Enum do SDK.
pub type D3D11_COMPARISON_FUNC = i32;
pub const D3D11_COMPARISON_NEVER: D3D11_COMPARISON_FUNC = 1;
pub const D3D11_COMPARISON_LESS: D3D11_COMPARISON_FUNC = 2;
pub const D3D11_COMPARISON_EQUAL: D3D11_COMPARISON_FUNC = 3;
pub const D3D11_COMPARISON_LESS_EQUAL: D3D11_COMPARISON_FUNC = 4;
pub const D3D11_COMPARISON_GREATER: D3D11_COMPARISON_FUNC = 5;
pub const D3D11_COMPARISON_NOT_EQUAL: D3D11_COMPARISON_FUNC = 6;
pub const D3D11_COMPARISON_GREATER_EQUAL: D3D11_COMPARISON_FUNC = 7;
pub const D3D11_COMPARISON_ALWAYS: D3D11_COMPARISON_FUNC = 8;

/// Enum do SDK.
pub type D3D11_DEPTH_WRITE_MASK = i32;
pub const D3D11_DEPTH_WRITE_MASK_ZERO: D3D11_DEPTH_WRITE_MASK = 0;
pub const D3D11_DEPTH_WRITE_MASK_ALL: D3D11_DEPTH_WRITE_MASK = 1;

/// Enum do SDK.
pub type D3D11_STENCIL_OP = i32;
pub const D3D11_STENCIL_OP_KEEP: D3D11_STENCIL_OP = 1;
pub const D3D11_STENCIL_OP_ZERO: D3D11_STENCIL_OP = 2;
pub const D3D11_STENCIL_OP_REPLACE: D3D11_STENCIL_OP = 3;
pub const D3D11_STENCIL_OP_INCR_SAT: D3D11_STENCIL_OP = 4;
pub const D3D11_STENCIL_OP_DECR_SAT: D3D11_STENCIL_OP = 5;
pub const D3D11_STENCIL_OP_INVERT: D3D11_STENCIL_OP = 6;
pub const D3D11_STENCIL_OP_INCR: D3D11_STENCIL_OP = 7;
pub const D3D11_STENCIL_OP_DECR: D3D11_STENCIL_OP = 8;

/// Enum do SDK.
pub type D3D11_FILTER = i32;
pub const D3D11_FILTER_MIN_MAG_MIP_POINT: D3D11_FILTER = 0x0;
pub const D3D11_FILTER_MIN_MAG_POINT_MIP_LINEAR: D3D11_FILTER = 0x1;
pub const D3D11_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D11_FILTER = 0x4;
pub const D3D11_FILTER_MIN_POINT_MAG_MIP_LINEAR: D3D11_FILTER = 0x5;
pub const D3D11_FILTER_MIN_LINEAR_MAG_MIP_POINT: D3D11_FILTER = 0x10;
pub const D3D11_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D11_FILTER = 0x11;
pub const D3D11_FILTER_MIN_MAG_LINEAR_MIP_POINT: D3D11_FILTER = 0x14;
pub const D3D11_FILTER_MIN_MAG_MIP_LINEAR: D3D11_FILTER = 0x15;
pub const D3D11_FILTER_ANISOTROPIC: D3D11_FILTER = 0x55;
pub const D3D11_FILTER_COMPARISON_MIN_MAG_MIP_POINT: D3D11_FILTER = 0x80;
pub const D3D11_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR: D3D11_FILTER = 0x81;
pub const D3D11_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D11_FILTER = 0x84;
pub const D3D11_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR: D3D11_FILTER = 0x85;
pub const D3D11_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT: D3D11_FILTER = 0x90;
pub const D3D11_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D11_FILTER = 0x91;
pub const D3D11_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT: D3D11_FILTER = 0x94;
pub const D3D11_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR: D3D11_FILTER = 0x95;
pub const D3D11_FILTER_COMPARISON_ANISOTROPIC: D3D11_FILTER = 0xD5;
pub const D3D11_FILTER_MINIMUM_MIN_MAG_MIP_POINT: D3D11_FILTER = 0x100;
pub const D3D11_FILTER_MINIMUM_MIN_MAG_POINT_MIP_LINEAR: D3D11_FILTER = 0x101;
pub const D3D11_FILTER_MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D11_FILTER = 0x104;
pub const D3D11_FILTER_MINIMUM_MIN_POINT_MAG_MIP_LINEAR: D3D11_FILTER = 0x105;
pub const D3D11_FILTER_MINIMUM_MIN_LINEAR_MAG_MIP_POINT: D3D11_FILTER = 0x110;
pub const D3D11_FILTER_MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D11_FILTER = 0x111;
pub const D3D11_FILTER_MINIMUM_MIN_MAG_LINEAR_MIP_POINT: D3D11_FILTER = 0x114;
pub const D3D11_FILTER_MINIMUM_MIN_MAG_MIP_LINEAR: D3D11_FILTER = 0x115;
pub const D3D11_FILTER_MINIMUM_ANISOTROPIC: D3D11_FILTER = 0x155;
pub const D3D11_FILTER_MAXIMUM_MIN_MAG_MIP_POINT: D3D11_FILTER = 0x180;
pub const D3D11_FILTER_MAXIMUM_MIN_MAG_POINT_MIP_LINEAR: D3D11_FILTER = 0x181;
pub const D3D11_FILTER_MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D11_FILTER = 0x184;
pub const D3D11_FILTER_MAXIMUM_MIN_POINT_MAG_MIP_LINEAR: D3D11_FILTER = 0x185;
pub const D3D11_FILTER_MAXIMUM_MIN_LINEAR_MAG_MIP_POINT: D3D11_FILTER = 0x190;
pub const D3D11_FILTER_MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D11_FILTER = 0x191;
pub const D3D11_FILTER_MAXIMUM_MIN_MAG_LINEAR_MIP_POINT: D3D11_FILTER = 0x194;
pub const D3D11_FILTER_MAXIMUM_MIN_MAG_MIP_LINEAR: D3D11_FILTER = 0x195;
pub const D3D11_FILTER_MAXIMUM_ANISOTROPIC: D3D11_FILTER = 0x1D5;

/// Enum do SDK.
pub type D3D11_TEXTURE_ADDRESS_MODE = i32;
pub const D3D11_TEXTURE_ADDRESS_WRAP: D3D11_TEXTURE_ADDRESS_MODE = 1;
pub const D3D11_TEXTURE_ADDRESS_MIRROR: D3D11_TEXTURE_ADDRESS_MODE = 2;
pub const D3D11_TEXTURE_ADDRESS_CLAMP: D3D11_TEXTURE_ADDRESS_MODE = 3;
pub const D3D11_TEXTURE_ADDRESS_BORDER: D3D11_TEXTURE_ADDRESS_MODE = 4;
pub const D3D11_TEXTURE_ADDRESS_MIRROR_ONCE: D3D11_TEXTURE_ADDRESS_MODE = 5;

/// Enum do SDK.
pub type D3D11_RTV_DIMENSION = i32;
pub const D3D11_RTV_DIMENSION_UNKNOWN: D3D11_RTV_DIMENSION = 0;
pub const D3D11_RTV_DIMENSION_BUFFER: D3D11_RTV_DIMENSION = 1;
pub const D3D11_RTV_DIMENSION_TEXTURE1D: D3D11_RTV_DIMENSION = 2;
pub const D3D11_RTV_DIMENSION_TEXTURE1DARRAY: D3D11_RTV_DIMENSION = 3;
pub const D3D11_RTV_DIMENSION_TEXTURE2D: D3D11_RTV_DIMENSION = 4;
pub const D3D11_RTV_DIMENSION_TEXTURE2DARRAY: D3D11_RTV_DIMENSION = 5;
pub const D3D11_RTV_DIMENSION_TEXTURE2DMS: D3D11_RTV_DIMENSION = 6;
pub const D3D11_RTV_DIMENSION_TEXTURE2DMSARRAY: D3D11_RTV_DIMENSION = 7;
pub const D3D11_RTV_DIMENSION_TEXTURE3D: D3D11_RTV_DIMENSION = 8;

/// Enum do SDK.
pub type D3D11_DSV_DIMENSION = i32;
pub const D3D11_DSV_DIMENSION_UNKNOWN: D3D11_DSV_DIMENSION = 0;
pub const D3D11_DSV_DIMENSION_TEXTURE1D: D3D11_DSV_DIMENSION = 1;
pub const D3D11_DSV_DIMENSION_TEXTURE1DARRAY: D3D11_DSV_DIMENSION = 2;
pub const D3D11_DSV_DIMENSION_TEXTURE2D: D3D11_DSV_DIMENSION = 3;
pub const D3D11_DSV_DIMENSION_TEXTURE2DARRAY: D3D11_DSV_DIMENSION = 4;
pub const D3D11_DSV_DIMENSION_TEXTURE2DMS: D3D11_DSV_DIMENSION = 5;
pub const D3D11_DSV_DIMENSION_TEXTURE2DMSARRAY: D3D11_DSV_DIMENSION = 6;

// ---- Structs ----------------------------------------------------------------

#[repr(C)]
pub struct D3D11_VIEWPORT {
    pub TopLeftX: FLOAT,
    pub TopLeftY: FLOAT,
    pub Width: FLOAT,
    pub Height: FLOAT,
    pub MinDepth: FLOAT,
    pub MaxDepth: FLOAT,
}

layout!(D3D11_VIEWPORT, size = 24, align = 4);

#[repr(C)]
pub struct D3D11_BUFFER_DESC {
    pub ByteWidth: UINT,
    pub Usage: D3D11_USAGE,
    pub BindFlags: UINT,
    pub CPUAccessFlags: UINT,
    pub MiscFlags: UINT,
    pub StructureByteStride: UINT,
}

layout!(D3D11_BUFFER_DESC, size = 24, align = 4);

#[repr(C)]
pub struct D3D11_TEXTURE2D_DESC {
    pub Width: UINT,
    pub Height: UINT,
    pub MipLevels: UINT,
    pub ArraySize: UINT,
    pub Format: DXGI_FORMAT,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub Usage: D3D11_USAGE,
    pub BindFlags: UINT,
    pub CPUAccessFlags: UINT,
    pub MiscFlags: UINT,
}

layout!(D3D11_TEXTURE2D_DESC, size = 44, align = 4);

/// Dados iniciais de um recurso. `SysMemPitch`: bytes por linha (texturas).
#[repr(C)]
pub struct D3D11_SUBRESOURCE_DATA {
    pub pSysMem: *const c_void,
    pub SysMemPitch: UINT,
    pub SysMemSlicePitch: UINT,
}

layout!(D3D11_SUBRESOURCE_DATA, size = 16, align = 8);

/// O que `Map` devolve: ponteiro para escrever direto na memoria do recurso.
#[repr(C)]
pub struct D3D11_MAPPED_SUBRESOURCE {
    pub pData: *mut c_void,
    pub RowPitch: UINT,
    pub DepthPitch: UINT,
}

layout!(D3D11_MAPPED_SUBRESOURCE, size = 16, align = 8);

/// Um atributo de vertice; liga um campo do vertex buffer a uma semantica do
/// shader (`SemanticName` = "POS", "NOR"...).
#[repr(C)]
pub struct D3D11_INPUT_ELEMENT_DESC {
    pub SemanticName: LPCSTR,
    pub SemanticIndex: UINT,
    pub Format: DXGI_FORMAT,
    pub InputSlot: UINT,
    pub AlignedByteOffset: UINT,
    pub InputSlotClass: D3D11_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: UINT,
}

layout!(D3D11_INPUT_ELEMENT_DESC, size = 32, align = 8);

#[repr(C)]
pub struct D3D11_RASTERIZER_DESC {
    pub FillMode: D3D11_FILL_MODE,
    pub CullMode: D3D11_CULL_MODE,
    pub FrontCounterClockwise: BOOL,
    pub DepthBias: INT,
    pub DepthBiasClamp: FLOAT,
    pub SlopeScaledDepthBias: FLOAT,
    pub DepthClipEnable: BOOL,
    pub ScissorEnable: BOOL,
    pub MultisampleEnable: BOOL,
    pub AntialiasedLineEnable: BOOL,
}

layout!(D3D11_RASTERIZER_DESC, size = 40, align = 4);

#[repr(C)]
pub struct D3D11_SAMPLER_DESC {
    pub Filter: D3D11_FILTER,
    pub AddressU: D3D11_TEXTURE_ADDRESS_MODE,
    pub AddressV: D3D11_TEXTURE_ADDRESS_MODE,
    pub AddressW: D3D11_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: FLOAT,
    pub MaxAnisotropy: UINT,
    pub ComparisonFunc: D3D11_COMPARISON_FUNC,
    pub BorderColor: [FLOAT; 4],
    pub MinLOD: FLOAT,
    pub MaxLOD: FLOAT,
}

layout!(D3D11_SAMPLER_DESC, size = 52, align = 4);

#[repr(C)]
pub struct D3D11_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: D3D11_STENCIL_OP,
    pub StencilDepthFailOp: D3D11_STENCIL_OP,
    pub StencilPassOp: D3D11_STENCIL_OP,
    pub StencilFunc: D3D11_COMPARISON_FUNC,
}

layout!(D3D11_DEPTH_STENCILOP_DESC, size = 16, align = 4);

#[repr(C)]
pub struct D3D11_DEPTH_STENCIL_DESC {
    pub DepthEnable: BOOL,
    pub DepthWriteMask: D3D11_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D11_COMPARISON_FUNC,
    pub StencilEnable: BOOL,
    pub StencilReadMask: UINT8,
    pub StencilWriteMask: UINT8,
    pub FrontFace: D3D11_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D11_DEPTH_STENCILOP_DESC,
}

layout!(D3D11_DEPTH_STENCIL_DESC, size = 52, align = 4);

// ---- View descs -------------------------------------------------------------
//
// No SDK, depois de `Format`/`ViewDimension` vem uma union com um membro por
// tipo de recurso (Buffer, Texture1D, Texture2D, Texture2DArray, ...). Aqui
// fica so `Texture2D` seguido de padding ate o tamanho do maior membro: mesmo
// layout, sem precisar de `unsafe` para acessar o campo.

#[repr(C)]
pub struct D3D11_TEX2D_RTV {
    pub MipSlice: UINT,
}

layout!(D3D11_TEX2D_RTV, size = 4, align = 4);

#[repr(C)]
pub struct D3D11_RENDER_TARGET_VIEW_DESC {
    pub Format: DXGI_FORMAT,
    pub ViewDimension: D3D11_RTV_DIMENSION,
    pub Texture2D: D3D11_TEX2D_RTV,
    /// Resto da union (maior membro: `D3D11_TEX2D_ARRAY_RTV`, 12 bytes).
    pub _union: [UINT; 2],
}

layout!(D3D11_RENDER_TARGET_VIEW_DESC, size = 20, align = 4);

#[repr(C)]
pub struct D3D11_TEX2D_DSV {
    pub MipSlice: UINT,
}

layout!(D3D11_TEX2D_DSV, size = 4, align = 4);

#[repr(C)]
pub struct D3D11_DEPTH_STENCIL_VIEW_DESC {
    pub Format: DXGI_FORMAT,
    pub ViewDimension: D3D11_DSV_DIMENSION,
    pub Flags: UINT,
    pub Texture2D: D3D11_TEX2D_DSV,
    /// Resto da union (maior membro: `D3D11_TEX2D_ARRAY_DSV`, 12 bytes).
    pub _union: [UINT; 2],
}

layout!(D3D11_DEPTH_STENCIL_VIEW_DESC, size = 24, align = 4);

#[repr(C)]
pub struct D3D11_TEX2D_SRV {
    pub MostDetailedMip: UINT,
    pub MipLevels: UINT,
}

layout!(D3D11_TEX2D_SRV, size = 8, align = 4);

#[repr(C)]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC {
    pub Format: DXGI_FORMAT,
    pub ViewDimension: D3D11_SRV_DIMENSION,
    pub Texture2D: D3D11_TEX2D_SRV,
    /// Resto da union (maior membro: `D3D11_TEXCUBE_ARRAY_SRV`, 16 bytes).
    pub _union: [UINT; 2],
}

layout!(D3D11_SHADER_RESOURCE_VIEW_DESC, size = 24, align = 4);

// ---- Interfaces -------------------------------------------------------------

/// Base de tudo que pertence a um device.
/// `{1841E5C8-16B0-489B-BCC8-44CFB0D5DEAE}`
pub const IID_ID3D11DeviceChild: IID = IID {
    Data1: 0x1841_E5C8,
    Data2: 0x16B0,
    Data3: 0x489B,
    Data4: [0xBC, 0xC8, 0x44, 0xCF, 0xB0, 0xD5, 0xDE, 0xAE],
};

#[repr(C)]
pub struct ID3D11DeviceChild {
    pub lpVtbl: *const ID3D11DeviceChildVtbl,
}

#[repr(C)]
pub struct ID3D11DeviceChildVtbl {
    pub base: IUnknownVtbl,
    pub GetDevice: usize,
    pub GetPrivateData: usize,
    pub SetPrivateData: usize,
    pub SetPrivateDataInterface: usize,
}

layout!(ID3D11DeviceChildVtbl, size = 56, align = 8);

/// Base de buffers e texturas (memoria na GPU).
/// `{DC8E63F3-D12B-4952-B47B-5E45026A862D}`
pub const IID_ID3D11Resource: IID = IID {
    Data1: 0xDC8E_63F3,
    Data2: 0xD12B,
    Data3: 0x4952,
    Data4: [0xB4, 0x7B, 0x5E, 0x45, 0x02, 0x6A, 0x86, 0x2D],
};

#[repr(C)]
pub struct ID3D11Resource {
    pub lpVtbl: *const ID3D11ResourceVtbl,
}

#[repr(C)]
pub struct ID3D11ResourceVtbl {
    pub base: ID3D11DeviceChildVtbl,
    pub GetType: usize,
    pub SetEvictionPriority: usize,
    pub GetEvictionPriority: usize,
}

layout!(ID3D11ResourceVtbl, size = 80, align = 8);

/// `{48570B85-D1EE-4FCD-A250-EB350722B037}`
pub const IID_ID3D11Buffer: IID = IID {
    Data1: 0x4857_0B85,
    Data2: 0xD1EE,
    Data3: 0x4FCD,
    Data4: [0xA2, 0x50, 0xEB, 0x35, 0x07, 0x22, 0xB0, 0x37],
};

#[repr(C)]
pub struct ID3D11Buffer {
    pub lpVtbl: *const ID3D11BufferVtbl,
}

#[repr(C)]
pub struct ID3D11BufferVtbl {
    pub base: ID3D11ResourceVtbl,
    pub GetDesc: unsafe extern "system" fn(This: *mut ID3D11Buffer, pDesc: *mut D3D11_BUFFER_DESC),
}

layout!(ID3D11BufferVtbl, size = 88, align = 8);

/// `{6F15AAF2-D208-4E89-9AB4-489535D34F9C}`
pub const IID_ID3D11Texture2D: IID = IID {
    Data1: 0x6F15_AAF2,
    Data2: 0xD208,
    Data3: 0x4E89,
    Data4: [0x9A, 0xB4, 0x48, 0x95, 0x35, 0xD3, 0x4F, 0x9C],
};

#[repr(C)]
pub struct ID3D11Texture2D {
    pub lpVtbl: *const ID3D11Texture2DVtbl,
}

#[repr(C)]
pub struct ID3D11Texture2DVtbl {
    pub base: ID3D11ResourceVtbl,
    pub GetDesc: unsafe extern "system" fn(This: *mut ID3D11Texture2D, pDesc: *mut D3D11_TEXTURE2D_DESC),
}

layout!(ID3D11Texture2DVtbl, size = 88, align = 8);

/// Base das views: como um estagio do pipeline enxerga um recurso.
/// `{839D1216-BB2E-412B-B7F4-A9DBEBE08ED1}`
pub const IID_ID3D11View: IID = IID {
    Data1: 0x839D_1216,
    Data2: 0xBB2E,
    Data3: 0x412B,
    Data4: [0xB7, 0xF4, 0xA9, 0xDB, 0xEB, 0xE0, 0x8E, 0xD1],
};

#[repr(C)]
pub struct ID3D11View {
    pub lpVtbl: *const ID3D11ViewVtbl,
}

#[repr(C)]
pub struct ID3D11ViewVtbl {
    pub base: ID3D11DeviceChildVtbl,
    pub GetResource: usize,
}

layout!(ID3D11ViewVtbl, size = 64, align = 8);

/// `{DFDBA067-0B8D-4865-875B-D7B4516CC164}`
pub const IID_ID3D11RenderTargetView: IID = IID {
    Data1: 0xDFDB_A067,
    Data2: 0x0B8D,
    Data3: 0x4865,
    Data4: [0x87, 0x5B, 0xD7, 0xB4, 0x51, 0x6C, 0xC1, 0x64],
};

#[repr(C)]
pub struct ID3D11RenderTargetView {
    pub lpVtbl: *const ID3D11RenderTargetViewVtbl,
}

#[repr(C)]
pub struct ID3D11RenderTargetViewVtbl {
    pub base: ID3D11ViewVtbl,
    pub GetDesc: usize,
}

layout!(ID3D11RenderTargetViewVtbl, size = 72, align = 8);

/// `{9FDAC92A-1876-48C3-AFAD-25B94F84A9B6}`
pub const IID_ID3D11DepthStencilView: IID = IID {
    Data1: 0x9FDA_C92A,
    Data2: 0x1876,
    Data3: 0x48C3,
    Data4: [0xAF, 0xAD, 0x25, 0xB9, 0x4F, 0x84, 0xA9, 0xB6],
};

#[repr(C)]
pub struct ID3D11DepthStencilView {
    pub lpVtbl: *const ID3D11DepthStencilViewVtbl,
}

#[repr(C)]
pub struct ID3D11DepthStencilViewVtbl {
    pub base: ID3D11ViewVtbl,
    pub GetDesc: usize,
}

layout!(ID3D11DepthStencilViewVtbl, size = 72, align = 8);

/// `{B0E06FE0-8192-4E1A-B1CA-36D7414710B2}`
pub const IID_ID3D11ShaderResourceView: IID = IID {
    Data1: 0xB0E0_6FE0,
    Data2: 0x8192,
    Data3: 0x4E1A,
    Data4: [0xB1, 0xCA, 0x36, 0xD7, 0x41, 0x47, 0x10, 0xB2],
};

#[repr(C)]
pub struct ID3D11ShaderResourceView {
    pub lpVtbl: *const ID3D11ShaderResourceViewVtbl,
}

#[repr(C)]
pub struct ID3D11ShaderResourceViewVtbl {
    pub base: ID3D11ViewVtbl,
    pub GetDesc: usize,
}

layout!(ID3D11ShaderResourceViewVtbl, size = 72, align = 8);

/// `{3B301D64-D678-4289-8897-22F8928B72F3}`
pub const IID_ID3D11VertexShader: IID = IID {
    Data1: 0x3B30_1D64,
    Data2: 0xD678,
    Data3: 0x4289,
    Data4: [0x88, 0x97, 0x22, 0xF8, 0x92, 0x8B, 0x72, 0xF3],
};

#[repr(C)]
pub struct ID3D11VertexShader {
    pub lpVtbl: *const ID3D11VertexShaderVtbl,
}

#[repr(C)]
pub struct ID3D11VertexShaderVtbl {
    pub base: ID3D11DeviceChildVtbl,
}

layout!(ID3D11VertexShaderVtbl, size = 56, align = 8);

/// `{EA82E40D-51DC-4F33-93D4-DB7C9125AE8C}`
pub const IID_ID3D11PixelShader: IID = IID {
    Data1: 0xEA82_E40D,
    Data2: 0x51DC,
    Data3: 0x4F33,
    Data4: [0x93, 0xD4, 0xDB, 0x7C, 0x91, 0x25, 0xAE, 0x8C],
};

#[repr(C)]
pub struct ID3D11PixelShader {
    pub lpVtbl: *const ID3D11PixelShaderVtbl,
}

#[repr(C)]
pub struct ID3D11PixelShaderVtbl {
    pub base: ID3D11DeviceChildVtbl,
}

layout!(ID3D11PixelShaderVtbl, size = 56, align = 8);

/// `{E4819DDC-4CF0-4025-BD26-5DE82A3E07B7}`
pub const IID_ID3D11InputLayout: IID = IID {
    Data1: 0xE481_9DDC,
    Data2: 0x4CF0,
    Data3: 0x4025,
    Data4: [0xBD, 0x26, 0x5D, 0xE8, 0x2A, 0x3E, 0x07, 0xB7],
};

#[repr(C)]
pub struct ID3D11InputLayout {
    pub lpVtbl: *const ID3D11InputLayoutVtbl,
}

#[repr(C)]
pub struct ID3D11InputLayoutVtbl {
    pub base: ID3D11DeviceChildVtbl,
}

layout!(ID3D11InputLayoutVtbl, size = 56, align = 8);

/// `{75B68FAA-347D-4159-8F45-A0640F01CD9A}`
pub const IID_ID3D11BlendState: IID = IID {
    Data1: 0x75B6_8FAA,
    Data2: 0x347D,
    Data3: 0x4159,
    Data4: [0x8F, 0x45, 0xA0, 0x64, 0x0F, 0x01, 0xCD, 0x9A],
};

#[repr(C)]
pub struct ID3D11BlendState {
    pub lpVtbl: *const ID3D11BlendStateVtbl,
}

#[repr(C)]
pub struct ID3D11BlendStateVtbl {
    pub base: ID3D11DeviceChildVtbl,
    pub GetDesc: usize,
}

layout!(ID3D11BlendStateVtbl, size = 64, align = 8);

/// `{03823EFB-8D8F-4E1C-9AA2-F64BB2CBFDF1}`
pub const IID_ID3D11DepthStencilState: IID = IID {
    Data1: 0x0382_3EFB,
    Data2: 0x8D8F,
    Data3: 0x4E1C,
    Data4: [0x9A, 0xA2, 0xF6, 0x4B, 0xB2, 0xCB, 0xFD, 0xF1],
};

#[repr(C)]
pub struct ID3D11DepthStencilState {
    pub lpVtbl: *const ID3D11DepthStencilStateVtbl,
}

#[repr(C)]
pub struct ID3D11DepthStencilStateVtbl {
    pub base: ID3D11DeviceChildVtbl,
    pub GetDesc: usize,
}

layout!(ID3D11DepthStencilStateVtbl, size = 64, align = 8);

/// `{9BB4AB81-AB1A-4D8F-B506-FC04200B6EE7}`
pub const IID_ID3D11RasterizerState: IID = IID {
    Data1: 0x9BB4_AB81,
    Data2: 0xAB1A,
    Data3: 0x4D8F,
    Data4: [0xB5, 0x06, 0xFC, 0x04, 0x20, 0x0B, 0x6E, 0xE7],
};

#[repr(C)]
pub struct ID3D11RasterizerState {
    pub lpVtbl: *const ID3D11RasterizerStateVtbl,
}

#[repr(C)]
pub struct ID3D11RasterizerStateVtbl {
    pub base: ID3D11DeviceChildVtbl,
    pub GetDesc: usize,
}

layout!(ID3D11RasterizerStateVtbl, size = 64, align = 8);

/// `{DA6FEA51-564C-4487-9810-F0D0F9B4E3A5}`
pub const IID_ID3D11SamplerState: IID = IID {
    Data1: 0xDA6F_EA51,
    Data2: 0x564C,
    Data3: 0x4487,
    Data4: [0x98, 0x10, 0xF0, 0xD0, 0xF9, 0xB4, 0xE3, 0xA5],
};

#[repr(C)]
pub struct ID3D11SamplerState {
    pub lpVtbl: *const ID3D11SamplerStateVtbl,
}

#[repr(C)]
pub struct ID3D11SamplerStateVtbl {
    pub base: ID3D11DeviceChildVtbl,
    pub GetDesc: usize,
}

layout!(ID3D11SamplerStateVtbl, size = 64, align = 8);

/// Dynamic shader linkage; aqui so para tipar parametros (passe null).
/// `{A6CD7FAA-B0B7-4A2F-9436-8662A65797CB}`
pub const IID_ID3D11ClassInstance: IID = IID {
    Data1: 0xA6CD_7FAA,
    Data2: 0xB0B7,
    Data3: 0x4A2F,
    Data4: [0x94, 0x36, 0x86, 0x62, 0xA6, 0x57, 0x97, 0xCB],
};

#[repr(C)]
pub struct ID3D11ClassInstance {
    pub lpVtbl: *const ID3D11ClassInstanceVtbl,
}

#[repr(C)]
pub struct ID3D11ClassInstanceVtbl {
    pub base: ID3D11DeviceChildVtbl,
    pub GetClassLinkage: usize,
    pub GetDesc: usize,
    pub GetInstanceName: usize,
    pub GetTypeName: usize,
}

layout!(ID3D11ClassInstanceVtbl, size = 88, align = 8);

/// Dynamic shader linkage; aqui so para tipar parametros (passe null).
/// `{DDF57CBA-9543-46E4-A12B-F207A0FE7FED}`
pub const IID_ID3D11ClassLinkage: IID = IID {
    Data1: 0xDDF5_7CBA,
    Data2: 0x9543,
    Data3: 0x46E4,
    Data4: [0xA1, 0x2B, 0xF2, 0x07, 0xA0, 0xFE, 0x7F, 0xED],
};

#[repr(C)]
pub struct ID3D11ClassLinkage {
    pub lpVtbl: *const ID3D11ClassLinkageVtbl,
}

#[repr(C)]
pub struct ID3D11ClassLinkageVtbl {
    pub base: ID3D11DeviceChildVtbl,
    pub GetClassInstance: usize,
    pub CreateClassInstance: usize,
}

layout!(ID3D11ClassLinkageVtbl, size = 72, align = 8);

/// Cria recursos, views, shaders e states. Nao desenha.
/// `{DB6F6DDB-AC77-4E88-8253-819DF9BBF140}`
pub const IID_ID3D11Device: IID = IID {
    Data1: 0xDB6F_6DDB,
    Data2: 0xAC77,
    Data3: 0x4E88,
    Data4: [0x82, 0x53, 0x81, 0x9D, 0xF9, 0xBB, 0xF1, 0x40],
};

#[repr(C)]
pub struct ID3D11Device {
    pub lpVtbl: *const ID3D11DeviceVtbl,
}

#[repr(C)]
pub struct ID3D11DeviceVtbl {
    pub base: IUnknownVtbl,
    pub CreateBuffer: unsafe extern "system" fn(This: *mut ID3D11Device,
        pDesc: *const D3D11_BUFFER_DESC,
        pInitialData: *const D3D11_SUBRESOURCE_DATA,
        ppBuffer: *mut *mut ID3D11Buffer,
    ) -> HRESULT,
    pub CreateTexture1D: usize,
    pub CreateTexture2D: unsafe extern "system" fn(This: *mut ID3D11Device,
        pDesc: *const D3D11_TEXTURE2D_DESC,
        pInitialData: *const D3D11_SUBRESOURCE_DATA,
        ppTexture2D: *mut *mut ID3D11Texture2D,
    ) -> HRESULT,
    pub CreateTexture3D: usize,
    pub CreateShaderResourceView: unsafe extern "system" fn(This: *mut ID3D11Device,
        pResource: *mut ID3D11Resource,
        pDesc: *const D3D11_SHADER_RESOURCE_VIEW_DESC,
        ppSRView: *mut *mut ID3D11ShaderResourceView,
    ) -> HRESULT,
    pub CreateUnorderedAccessView: usize,
    pub CreateRenderTargetView: unsafe extern "system" fn(This: *mut ID3D11Device,
        pResource: *mut ID3D11Resource,
        pDesc: *const D3D11_RENDER_TARGET_VIEW_DESC,
        ppRTView: *mut *mut ID3D11RenderTargetView,
    ) -> HRESULT,
    pub CreateDepthStencilView: unsafe extern "system" fn(This: *mut ID3D11Device,
        pResource: *mut ID3D11Resource,
        pDesc: *const D3D11_DEPTH_STENCIL_VIEW_DESC,
        ppDepthStencilView: *mut *mut ID3D11DepthStencilView,
    ) -> HRESULT,
    pub CreateInputLayout: unsafe extern "system" fn(This: *mut ID3D11Device,
        pInputElementDescs: *const D3D11_INPUT_ELEMENT_DESC,
        NumElements: UINT,
        pShaderBytecodeWithInputSignature: *const c_void,
        BytecodeLength: SIZE_T,
        ppInputLayout: *mut *mut ID3D11InputLayout,
    ) -> HRESULT,
    pub CreateVertexShader: unsafe extern "system" fn(This: *mut ID3D11Device,
        pShaderBytecode: *const c_void,
        BytecodeLength: SIZE_T,
        pClassLinkage: *mut ID3D11ClassLinkage,
        ppVertexShader: *mut *mut ID3D11VertexShader,
    ) -> HRESULT,
    pub CreateGeometryShader: usize,
    pub CreateGeometryShaderWithStreamOutput: usize,
    pub CreatePixelShader: unsafe extern "system" fn(This: *mut ID3D11Device,
        pShaderBytecode: *const c_void,
        BytecodeLength: SIZE_T,
        pClassLinkage: *mut ID3D11ClassLinkage,
        ppPixelShader: *mut *mut ID3D11PixelShader,
    ) -> HRESULT,
    pub CreateHullShader: usize,
    pub CreateDomainShader: usize,
    pub CreateComputeShader: usize,
    pub CreateClassLinkage: usize,
    pub CreateBlendState: usize,
    pub CreateDepthStencilState: unsafe extern "system" fn(This: *mut ID3D11Device,
        pDepthStencilDesc: *const D3D11_DEPTH_STENCIL_DESC,
        ppDepthStencilState: *mut *mut ID3D11DepthStencilState,
    ) -> HRESULT,
    pub CreateRasterizerState: unsafe extern "system" fn(This: *mut ID3D11Device,
        pRasterizerDesc: *const D3D11_RASTERIZER_DESC,
        ppRasterizerState: *mut *mut ID3D11RasterizerState,
    ) -> HRESULT,
    pub CreateSamplerState: unsafe extern "system" fn(This: *mut ID3D11Device,
        pSamplerDesc: *const D3D11_SAMPLER_DESC,
        ppSamplerState: *mut *mut ID3D11SamplerState,
    ) -> HRESULT,
    pub CreateQuery: usize,
    pub CreatePredicate: usize,
    pub CreateCounter: usize,
    pub CreateDeferredContext: usize,
    pub OpenSharedResource: usize,
    pub CheckFormatSupport: usize,
    pub CheckMultisampleQualityLevels: usize,
    pub CheckCounterInfo: usize,
    pub CheckCounter: usize,
    pub CheckFeatureSupport: usize,
    pub GetPrivateData: usize,
    pub SetPrivateData: usize,
    pub SetPrivateDataInterface: usize,
    pub GetFeatureLevel: usize,
    pub GetCreationFlags: usize,
    pub GetDeviceRemovedReason: usize,
    pub GetImmediateContext: unsafe extern "system" fn(This: *mut ID3D11Device, ppImmediateContext: *mut *mut ID3D11DeviceContext),
    pub SetExceptionMode: usize,
    pub GetExceptionMode: usize,
}

layout!(ID3D11DeviceVtbl, size = 344, align = 8);

/// Configura o pipeline e emite comandos de desenho.
/// `{C0BFA96C-E089-44FB-8EAF-26F8796190DA}`
pub const IID_ID3D11DeviceContext: IID = IID {
    Data1: 0xC0BF_A96C,
    Data2: 0xE089,
    Data3: 0x44FB,
    Data4: [0x8E, 0xAF, 0x26, 0xF8, 0x79, 0x61, 0x90, 0xDA],
};

#[repr(C)]
pub struct ID3D11DeviceContext {
    pub lpVtbl: *const ID3D11DeviceContextVtbl,
}

#[repr(C)]
pub struct ID3D11DeviceContextVtbl {
    pub base: ID3D11DeviceChildVtbl,
    pub VSSetConstantBuffers: unsafe extern "system" fn(This: *mut ID3D11DeviceContext,
        StartSlot: UINT,
        NumBuffers: UINT,
        ppConstantBuffers: *const *mut ID3D11Buffer,
    ),
    pub PSSetShaderResources: unsafe extern "system" fn(This: *mut ID3D11DeviceContext,
        StartSlot: UINT,
        NumViews: UINT,
        ppShaderResourceViews: *const *mut ID3D11ShaderResourceView,
    ),
    pub PSSetShader: unsafe extern "system" fn(This: *mut ID3D11DeviceContext,
        pPixelShader: *mut ID3D11PixelShader,
        ppClassInstances: *const *mut ID3D11ClassInstance,
        NumClassInstances: UINT,
    ),
    pub PSSetSamplers: unsafe extern "system" fn(This: *mut ID3D11DeviceContext,
        StartSlot: UINT,
        NumSamplers: UINT,
        ppSamplers: *const *mut ID3D11SamplerState,
    ),
    pub VSSetShader: unsafe extern "system" fn(This: *mut ID3D11DeviceContext,
        pVertexShader: *mut ID3D11VertexShader,
        ppClassInstances: *const *mut ID3D11ClassInstance,
        NumClassInstances: UINT,
    ),
    pub DrawIndexed: unsafe extern "system" fn(This: *mut ID3D11DeviceContext,
        IndexCount: UINT,
        StartIndexLocation: UINT,
        BaseVertexLocation: INT,
    ),
    pub Draw: unsafe extern "system" fn(This: *mut ID3D11DeviceContext, VertexCount: UINT, StartVertexLocation: UINT),
    pub Map: unsafe extern "system" fn(This: *mut ID3D11DeviceContext,
        pResource: *mut ID3D11Resource,
        Subresource: UINT,
        MapType: D3D11_MAP,
        MapFlags: UINT,
        pMappedResource: *mut D3D11_MAPPED_SUBRESOURCE,
    ) -> HRESULT,
    pub Unmap: unsafe extern "system" fn(This: *mut ID3D11DeviceContext, pResource: *mut ID3D11Resource, Subresource: UINT),
    pub PSSetConstantBuffers: usize,
    pub IASetInputLayout: unsafe extern "system" fn(This: *mut ID3D11DeviceContext, pInputLayout: *mut ID3D11InputLayout),
    pub IASetVertexBuffers: unsafe extern "system" fn(This: *mut ID3D11DeviceContext,
        StartSlot: UINT,
        NumBuffers: UINT,
        ppVertexBuffers: *const *mut ID3D11Buffer,
        pStrides: *const UINT,
        pOffsets: *const UINT,
    ),
    pub IASetIndexBuffer: unsafe extern "system" fn(This: *mut ID3D11DeviceContext,
        pIndexBuffer: *mut ID3D11Buffer,
        Format: DXGI_FORMAT,
        Offset: UINT,
    ),
    pub DrawIndexedInstanced: usize,
    pub DrawInstanced: usize,
    pub GSSetConstantBuffers: usize,
    pub GSSetShader: usize,
    pub IASetPrimitiveTopology: unsafe extern "system" fn(This: *mut ID3D11DeviceContext, Topology: D3D11_PRIMITIVE_TOPOLOGY),
    pub VSSetShaderResources: usize,
    pub VSSetSamplers: usize,
    pub Begin: usize,
    pub End: usize,
    pub GetData: usize,
    pub SetPredication: usize,
    pub GSSetShaderResources: usize,
    pub GSSetSamplers: usize,
    pub OMSetRenderTargets: unsafe extern "system" fn(This: *mut ID3D11DeviceContext,
        NumViews: UINT,
        ppRenderTargetViews: *const *mut ID3D11RenderTargetView,
        pDepthStencilView: *mut ID3D11DepthStencilView,
    ),
    pub OMSetRenderTargetsAndUnorderedAccessViews: usize,
    pub OMSetBlendState: unsafe extern "system" fn(This: *mut ID3D11DeviceContext,
        pBlendState: *mut ID3D11BlendState,
        BlendFactor: *const [FLOAT; 4],
        SampleMask: UINT,
    ),
    pub OMSetDepthStencilState: unsafe extern "system" fn(This: *mut ID3D11DeviceContext,
        pDepthStencilState: *mut ID3D11DepthStencilState,
        StencilRef: UINT,
    ),
    pub SOSetTargets: usize,
    pub DrawAuto: usize,
    pub DrawIndexedInstancedIndirect: usize,
    pub DrawInstancedIndirect: usize,
    pub Dispatch: usize,
    pub DispatchIndirect: usize,
    pub RSSetState: unsafe extern "system" fn(This: *mut ID3D11DeviceContext, pRasterizerState: *mut ID3D11RasterizerState),
    pub RSSetViewports: unsafe extern "system" fn(This: *mut ID3D11DeviceContext,
        NumViewports: UINT,
        pViewports: *const D3D11_VIEWPORT,
    ),
    pub RSSetScissorRects: usize,
    pub CopySubresourceRegion: usize,
    pub CopyResource: usize,
    pub UpdateSubresource: usize,
    pub CopyStructureCount: usize,
    pub ClearRenderTargetView: unsafe extern "system" fn(This: *mut ID3D11DeviceContext,
        pRenderTargetView: *mut ID3D11RenderTargetView,
        ColorRGBA: *const [FLOAT; 4],
    ),
    pub ClearUnorderedAccessViewUint: usize,
    pub ClearUnorderedAccessViewFloat: usize,
    pub ClearDepthStencilView: unsafe extern "system" fn(This: *mut ID3D11DeviceContext,
        pDepthStencilView: *mut ID3D11DepthStencilView,
        ClearFlags: UINT,
        Depth: FLOAT,
        Stencil: UINT8,
    ),
    pub GenerateMips: usize,
    pub SetResourceMinLOD: usize,
    pub GetResourceMinLOD: usize,
    pub ResolveSubresource: usize,
    pub ExecuteCommandList: usize,
    pub HSSetShaderResources: usize,
    pub HSSetShader: usize,
    pub HSSetSamplers: usize,
    pub HSSetConstantBuffers: usize,
    pub DSSetShaderResources: usize,
    pub DSSetShader: usize,
    pub DSSetSamplers: usize,
    pub DSSetConstantBuffers: usize,
    pub CSSetShaderResources: usize,
    pub CSSetUnorderedAccessViews: usize,
    pub CSSetShader: usize,
    pub CSSetSamplers: usize,
    pub CSSetConstantBuffers: usize,
    pub VSGetConstantBuffers: usize,
    pub PSGetShaderResources: usize,
    pub PSGetShader: usize,
    pub PSGetSamplers: usize,
    pub VSGetShader: usize,
    pub PSGetConstantBuffers: usize,
    pub IAGetInputLayout: usize,
    pub IAGetVertexBuffers: usize,
    pub IAGetIndexBuffer: usize,
    pub GSGetConstantBuffers: usize,
    pub GSGetShader: usize,
    pub IAGetPrimitiveTopology: usize,
    pub VSGetShaderResources: usize,
    pub VSGetSamplers: usize,
    pub GetPredication: usize,
    pub GSGetShaderResources: usize,
    pub GSGetSamplers: usize,
    pub OMGetRenderTargets: usize,
    pub OMGetRenderTargetsAndUnorderedAccessViews: usize,
    pub OMGetBlendState: usize,
    pub OMGetDepthStencilState: usize,
    pub SOGetTargets: usize,
    pub RSGetState: usize,
    pub RSGetViewports: usize,
    pub RSGetScissorRects: usize,
    pub HSGetShaderResources: usize,
    pub HSGetShader: usize,
    pub HSGetSamplers: usize,
    pub HSGetConstantBuffers: usize,
    pub DSGetShaderResources: usize,
    pub DSGetShader: usize,
    pub DSGetSamplers: usize,
    pub DSGetConstantBuffers: usize,
    pub CSGetShaderResources: usize,
    pub CSGetUnorderedAccessViews: usize,
    pub CSGetShader: usize,
    pub CSGetSamplers: usize,
    pub CSGetConstantBuffers: usize,
    pub ClearState: usize,
    pub Flush: usize,
    pub GetType: usize,
    pub GetContextFlags: usize,
    pub FinishCommandList: usize,
}

layout!(ID3D11DeviceContextVtbl, size = 920, align = 8);

// ---- Funcoes ----------------------------------------------------------------

#[link(name = "d3d11", kind = "raw-dylib")]
unsafe extern "system" {
    pub fn D3D11CreateDevice(
        pAdapter: *mut IDXGIAdapter,
        DriverType: D3D_DRIVER_TYPE,
        Software: HMODULE,
        Flags: UINT,
        pFeatureLevels: *const D3D_FEATURE_LEVEL,
        FeatureLevels: UINT,
        SDKVersion: UINT,
        ppDevice: *mut *mut ID3D11Device,
        pFeatureLevel: *mut D3D_FEATURE_LEVEL,
        ppImmediateContext: *mut *mut ID3D11DeviceContext,
    ) -> HRESULT;
    pub fn D3D11CreateDeviceAndSwapChain(
        pAdapter: *mut IDXGIAdapter,
        DriverType: D3D_DRIVER_TYPE,
        Software: HMODULE,
        Flags: UINT,
        pFeatureLevels: *const D3D_FEATURE_LEVEL,
        FeatureLevels: UINT,
        SDKVersion: UINT,
        pSwapChainDesc: *const DXGI_SWAP_CHAIN_DESC,
        ppSwapChain: *mut *mut IDXGISwapChain,
        ppDevice: *mut *mut ID3D11Device,
        pFeatureLevel: *mut D3D_FEATURE_LEVEL,
        ppImmediateContext: *mut *mut ID3D11DeviceContext,
    ) -> HRESULT;
}
