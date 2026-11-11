//! `d3dcompiler.h`: compila HLSL em bytecode de shader em tempo de execucao.
//!
//! A DLL `d3dcompiler_47.dll` vem com o proprio Windows (8.1 em diante), em
//! `System32`. Nao precisa do SDK nem de redistribuivel.
//!
//! O resultado (`ppCode`) e um `ID3DBlob`: `GetBufferPointer`/`GetBufferSize`
//! dele vao direto para `ID3D11Device::CreateVertexShader`/`CreatePixelShader`.
//! Se a compilacao falhar, `ppErrorMsgs` recebe um blob com o texto do erro
//! (string ASCII terminada em zero). Os dois blobs precisam de `Release`.

use super::d3dcommon::{D3D_SHADER_MACRO, ID3DBlob, ID3DInclude};
use super::minwindef::UINT;
use super::winnt::{HRESULT, LPCSTR, LPCWSTR};

// ---- D3DCOMPILE_* (Flags1) ------------------------------------------------
//
// So os mais usados. `Flags2` e para effects (`.fx`): passe 0.

/// Inclui informacao de debug no bytecode (para o PIX / Graphics Debugger).
pub const D3DCOMPILE_DEBUG: UINT = 1 << 0;
pub const D3DCOMPILE_SKIP_VALIDATION: UINT = 1 << 1;
/// Normalmente junto com `D3DCOMPILE_DEBUG`, para o debugger seguir o HLSL linha a linha.
pub const D3DCOMPILE_SKIP_OPTIMIZATION: UINT = 1 << 2;
pub const D3DCOMPILE_PACK_MATRIX_ROW_MAJOR: UINT = 1 << 3;
pub const D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR: UINT = 1 << 4;
/// Recusa sintaxe antiga/obsoleta do HLSL.
pub const D3DCOMPILE_ENABLE_STRICTNESS: UINT = 1 << 11;
pub const D3DCOMPILE_OPTIMIZATION_LEVEL0: UINT = 1 << 14;
/// Nivel padrao (valor 0: e o que voce ganha passando `Flags1 = 0`).
pub const D3DCOMPILE_OPTIMIZATION_LEVEL1: UINT = 0;
pub const D3DCOMPILE_OPTIMIZATION_LEVEL2: UINT = (1 << 14) | (1 << 15);
pub const D3DCOMPILE_OPTIMIZATION_LEVEL3: UINT = 1 << 15;
pub const D3DCOMPILE_WARNINGS_ARE_ERRORS: UINT = 1 << 18;

// ---- pInclude -------------------------------------------------------------

/// `((ID3DInclude*)(UINT_PTR)1)`: em vez de um objeto de verdade, o valor 1
/// pede o tratamento padrao de `#include "arquivo"` (relativo ao arquivo
/// sendo compilado). Passar null faz qualquer `#include` falhar.
pub const D3D_COMPILE_STANDARD_FILE_INCLUDE: *mut ID3DInclude = 1 as *mut ID3DInclude;

// ---- Funcoes --------------------------------------------------------------

#[link(name = "d3dcompiler_47", kind = "raw-dylib")]
unsafe extern "system" {
    /// `pTarget` = modelo de shader: `"vs_5_0"` (vertex), `"ps_5_0"` (pixel).
    /// `pEntrypoint` = nome da funcao HLSL. `pDefines` pode ser null.
    pub fn D3DCompileFromFile(
        pFileName: LPCWSTR,
        pDefines: *const D3D_SHADER_MACRO,
        pInclude: *mut ID3DInclude,
        pEntrypoint: LPCSTR,
        pTarget: LPCSTR,
        Flags1: UINT,
        Flags2: UINT,
        ppCode: *mut *mut ID3DBlob,
        ppErrorMsgs: *mut *mut ID3DBlob,
    ) -> HRESULT;
}
