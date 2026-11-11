//! `dxgi.h`: swap chain (os buffers que vao para a tela) e a hierarquia de
//! interfaces DXGI ate `IDXGISwapChain`.

#![allow(non_upper_case_globals)]

use super::dxgicommon::DXGI_SAMPLE_DESC;
use super::dxgitype::DXGI_MODE_DESC;
use super::guiddef::{IID, REFIID};
use super::minwindef::{BOOL, UINT};
use super::unknwn::IUnknownVtbl;
use super::windef::HWND;
use super::winnt::HRESULT;
use crate::layout;
use core::ffi::c_void;

// ---- DXGI_USAGE -----------------------------------------------------------

/// Flags: para que os buffers da swap chain (ou um recurso) serao usados.
pub type DXGI_USAGE = UINT;
pub const DXGI_USAGE_SHADER_INPUT: DXGI_USAGE = 0x0000_0010;
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: DXGI_USAGE = 0x0000_0020;
pub const DXGI_USAGE_BACK_BUFFER: DXGI_USAGE = 0x0000_0040;
pub const DXGI_USAGE_SHARED: DXGI_USAGE = 0x0000_0080;
pub const DXGI_USAGE_READ_ONLY: DXGI_USAGE = 0x0000_0100;
pub const DXGI_USAGE_DISCARD_ON_PRESENT: DXGI_USAGE = 0x0000_0200;
pub const DXGI_USAGE_UNORDERED_ACCESS: DXGI_USAGE = 0x0000_0400;

// ---- DXGI_SWAP_EFFECT -----------------------------------------------------

/// Enum do SDK. Os modelos `FLIP_*` sao os modernos (Windows 8+/10+) e
/// exigem `BufferCount >= 2` e sem MSAA na swap chain.
pub type DXGI_SWAP_EFFECT = i32;
pub const DXGI_SWAP_EFFECT_DISCARD: DXGI_SWAP_EFFECT = 0;
pub const DXGI_SWAP_EFFECT_SEQUENTIAL: DXGI_SWAP_EFFECT = 1;
pub const DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL: DXGI_SWAP_EFFECT = 3;
pub const DXGI_SWAP_EFFECT_FLIP_DISCARD: DXGI_SWAP_EFFECT = 4;

// ---- DXGI_SWAP_CHAIN_FLAG -------------------------------------------------

/// Enum do SDK, usado como flags em `DXGI_SWAP_CHAIN_DESC::Flags` (`UINT`).
pub type DXGI_SWAP_CHAIN_FLAG = i32;
pub const DXGI_SWAP_CHAIN_FLAG_NONPREROTATED: DXGI_SWAP_CHAIN_FLAG = 1;
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH: DXGI_SWAP_CHAIN_FLAG = 2;
pub const DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE: DXGI_SWAP_CHAIN_FLAG = 4;
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT: DXGI_SWAP_CHAIN_FLAG = 8;
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER: DXGI_SWAP_CHAIN_FLAG = 16;
pub const DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY: DXGI_SWAP_CHAIN_FLAG = 32;
pub const DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT: DXGI_SWAP_CHAIN_FLAG = 64;
pub const DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER: DXGI_SWAP_CHAIN_FLAG = 128;
pub const DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO: DXGI_SWAP_CHAIN_FLAG = 256;
pub const DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO: DXGI_SWAP_CHAIN_FLAG = 512;
pub const DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED: DXGI_SWAP_CHAIN_FLAG = 1024;
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING: DXGI_SWAP_CHAIN_FLAG = 2048;
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS: DXGI_SWAP_CHAIN_FLAG = 4096;

// ---- DXGI_SWAP_CHAIN_DESC -------------------------------------------------

/// Ha 4 bytes de padding depois de `BufferCount`, para `OutputWindow`
/// (ponteiro) ficar alinhado a 8; o `#[repr(C)]` reproduz isso sozinho.
#[repr(C)]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: DXGI_MODE_DESC,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: UINT,
    pub OutputWindow: HWND,
    pub Windowed: BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: UINT,
}

layout!(DXGI_SWAP_CHAIN_DESC, size = 72, align = 8);

// ---- Interfaces -----------------------------------------------------------
//
// Hierarquia: IUnknown -> IDXGIObject -> IDXGIDeviceSubObject -> IDXGISwapChain.
// Cada vtable comeca com a vtable da interface pai (`base`).

#[repr(C)]
pub struct IDXGIObjectVtbl {
    pub base: IUnknownVtbl,
    pub SetPrivateData: usize,
    pub SetPrivateDataInterface: usize,
    pub GetPrivateData: usize,
    pub GetParent: usize,
}

layout!(IDXGIObjectVtbl, size = 56, align = 8);

#[repr(C)]
pub struct IDXGIDeviceSubObjectVtbl {
    pub base: IDXGIObjectVtbl,
    pub GetDevice: usize,
}

layout!(IDXGIDeviceSubObjectVtbl, size = 64, align = 8);

/// Uma placa de video. `null` em `D3D11CreateDevice*` = adaptador padrao.
#[repr(C)]
pub struct IDXGIAdapter {
    pub lpVtbl: *const IDXGIAdapterVtbl,
}

#[repr(C)]
pub struct IDXGIAdapterVtbl {
    pub base: IDXGIObjectVtbl,
    pub EnumOutputs: usize,
    pub GetDesc: usize,
    pub CheckInterfaceSupport: usize,
}

layout!(IDXGIAdapterVtbl, size = 80, align = 8);

/// `{310D36A0-D2E7-4C0A-AA04-6A9D23B8886A}`
pub const IID_IDXGISwapChain: IID = IID {
    Data1: 0x310D_36A0,
    Data2: 0xD2E7,
    Data3: 0x4C0A,
    Data4: [0xAA, 0x04, 0x6A, 0x9D, 0x23, 0xB8, 0x88, 0x6A],
};

#[repr(C)]
pub struct IDXGISwapChain {
    pub lpVtbl: *const IDXGISwapChainVtbl,
}

impl IDXGISwapChain {
    pub unsafe fn GetDesc(&self, pDesc: *mut DXGI_SWAP_CHAIN_DESC) -> HRESULT {
        unsafe { ((*self.lpVtbl).GetDesc)(self as *const _ as *mut _, pDesc) }
    }

    pub unsafe fn GetBuffer(
        &self,
        Buffer: UINT,
        riid: REFIID,
        ppSurface: *mut *mut c_void,
    ) -> HRESULT {
        unsafe {
            ((*self.lpVtbl).GetBuffer)(self as *const _ as *mut _, Buffer, riid, ppSurface)
        }
    }
}

#[repr(C)]
pub struct IDXGISwapChainVtbl {
    pub base: IDXGIDeviceSubObjectVtbl,
    /// `SyncInterval` 1 = espera o vsync; 0 = apresenta ja.
    pub Present: unsafe extern "system" fn(
        This: *mut IDXGISwapChain,
        SyncInterval: UINT,
        Flags: UINT,
    ) -> HRESULT,
    /// Buffer 0 = back buffer; `riid` costuma ser `IID_ID3D11Texture2D`.
    pub GetBuffer: unsafe extern "system" fn(
        This: *mut IDXGISwapChain,
        Buffer: UINT,
        riid: REFIID,
        ppSurface: *mut *mut c_void,
    ) -> HRESULT,
    pub SetFullscreenState: usize,
    pub GetFullscreenState: usize,
    pub GetDesc: unsafe extern "system" fn(
        This: *mut IDXGISwapChain,
        pDesc: *mut DXGI_SWAP_CHAIN_DESC,
    ) -> HRESULT,
    pub ResizeBuffers: usize,
    pub ResizeTarget: usize,
    pub GetContainingOutput: usize,
    pub GetFrameStatistics: usize,
    pub GetLastPresentCount: usize,
}

layout!(IDXGISwapChainVtbl, size = 144, align = 8);
