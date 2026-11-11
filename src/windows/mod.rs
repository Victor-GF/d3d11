//! Bindings Win32 escritos a mao.
//!
//! Apenas o que o backend usa, declarado direto contra as DLLs do sistema.
//! Sem `windows-sys`, sem `winapi`: cada simbolo aqui e uma decisao explicita.
//!
//! Regras deste modulo:
//! - Um arquivo por header do Windows SDK, com o mesmo nome. Cada simbolo
//!   fica no arquivo do header que o declara.
//! - Linkagem `raw-dylib`: o rustc gera a import table sozinho, sem depender
//!   das import libs (.a/.lib) do toolchain.
//! - Toda struct `#[repr(C)]` tem seu tamanho e alinhamento travados com
//!   `layout!`, usando os valores x64 do SDK.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

/// Falha na compilacao se o tamanho/alinhamento da struct divergir do SDK.
/// `#[macro_export]` coloca a macro na raiz do crate: `use crate::layout;`.
/// (Antes vinha do `forge_core`, que nao e dependencia deste projeto.)
#[macro_export]
macro_rules! layout {
    ($t:ty, size = $size:expr, align = $align:expr) => {
        const _: () = {
            assert!(::core::mem::size_of::<$t>() == $size, concat!("size_of<", stringify!($t), "> diverge do esperado"));
            assert!(::core::mem::align_of::<$t>() == $align, concat!("align_of<", stringify!($t), "> diverge do esperado"));
        };
    };
}

/// O `L"texto"` do C: string UTF-16 terminada em zero, montada em tempo de
/// compilacao. Devolve um `LPCWSTR` (`*const u16`) valido pelo programa todo.
/// So aceita ASCII (o suficiente para nomes de arquivo, classes de janela etc.);
/// um caractere fora disso e erro de compilacao.
///
/// `D3DCompileFromFile(w!("gpu.hlsl"), ...)`
#[macro_export]
macro_rules! w {
    ($s:literal) => {{
        const BYTES: &[u8] = $s.as_bytes();
        static WIDE: [u16; BYTES.len() + 1] = {
            let mut out = [0u16; BYTES.len() + 1];
            let mut i = 0;
            while i < BYTES.len() {
                assert!(BYTES[i] < 0x80, "w!: so ASCII");
                out[i] = BYTES[i] as u16;
                i += 1;
            }
            out
        };
        WIDE.as_ptr()
    }};
}

mod audioclient;
mod audiosessiontypes;
mod basetsd;
mod combaseapi;
mod consoleapi;
mod d3d11;
mod d3dcommon;
mod d3dcompiler;
mod debugapi;
mod dxgi;
mod dxgicommon;
mod dxgiformat;
mod dxgitype;
mod errhandlingapi;
mod fileapi;
mod guiddef;
mod handleapi;
mod heapapi;
mod hidusage;
mod knownfolders;
mod ksmedia;
mod libloaderapi;
mod memoryapi;
mod minwinbase;
mod minwindef;
mod mmdeviceapi;
mod mmeapi;
mod mmreg;
mod processenv;
mod processthreadsapi;
mod profileapi;
mod shlobj_core;
mod shtypes;
mod synchapi;
mod sysinfoapi;
mod unknwn;
mod winbase;
mod windef;
mod winerror;
mod wingdi;
mod winnt;
mod winuser;
mod wtypesbase;
mod xinput;

pub use audioclient::*;
pub use audiosessiontypes::*;
pub use basetsd::*;
pub use combaseapi::*;
pub use consoleapi::*;
pub use d3d11::*;
pub use d3dcommon::*;
pub use d3dcompiler::*;
pub use debugapi::*;
pub use dxgi::*;
pub use dxgicommon::*;
pub use dxgiformat::*;
pub use dxgitype::*;
pub use errhandlingapi::*;
pub use fileapi::*;
pub use guiddef::*;
pub use handleapi::*;
pub use heapapi::*;
pub use hidusage::*;
pub use knownfolders::*;
pub use ksmedia::*;
pub use libloaderapi::*;
pub use memoryapi::*;
pub use minwinbase::*;
pub use minwindef::*;
pub use mmdeviceapi::*;
pub use mmeapi::*;
pub use mmreg::*;
pub use processenv::*;
pub use processthreadsapi::*;
pub use profileapi::*;
pub use shlobj_core::*;
pub use shtypes::*;
pub use synchapi::*;
pub use sysinfoapi::*;
pub use unknwn::*;
pub use winbase::*;
pub use windef::*;
pub use winerror::*;
pub use wingdi::*;
pub use winnt::*;
pub use winuser::*;
pub use wtypesbase::*;
pub use xinput::*;
