//! O minimo da CRT que o compilador espera encontrar no link.
//!
//! Sem CRT (`/NODEFAULTLIB`), ninguem fornece estes simbolos. O LLVM gera
//! chamadas a eles por conta propria: `mem::zeroed()` vira `memset`, copiar
//! uma struct grande vira `memcpy`, comparar slices vira `memcmp`. No release
//! a otimizacao costuma eliminar essas chamadas; no debug, nao.
//!
//! As funcoes de memoria usam `rep stosb`/`rep movsb` em assembly de
//! proposito: escritas como loop em Rust, o LLVM reconheceria o padrao e
//! trocaria o loop por... uma chamada a `memset`, dentro do proprio `memset`.

use core::arch::asm;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8 {
    unsafe {
        asm!(
            "rep stosb",
            inout("rcx") n => _,
            inout("rdi") dest => _,
            in("al") c as u8,
            options(nostack, preserves_flags)
        );
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        asm!(
            "rep movsb",
            inout("rcx") n => _,
            inout("rdi") dest => _,
            inout("rsi") src => _,
            options(nostack, preserves_flags)
        );
    }
    dest
}

/// Como `memcpy`, mas as regioes podem se sobrepor: se `dest` esta depois de
/// `src` dentro da mesma regiao, copia de tras para frente (`std` + `rep movsb`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        if (dest as usize).wrapping_sub(src as usize) >= n {
            // Sem sobreposicao perigosa: copia para frente.
            asm!(
                "rep movsb",
                inout("rcx") n => _,
                inout("rdi") dest => _,
                inout("rsi") src => _,
                options(nostack, preserves_flags)
            );
        } else {
            // `dest` dentro de [src, src + n): copia do fim para o comeco.
            asm!(
                "std",
                "rep movsb",
                "cld",
                inout("rcx") n => _,
                inout("rdi") dest.add(n - 1) => _,
                inout("rsi") src.add(n - 1) => _,
                options(nostack)
            );
        }
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let (x, y) = unsafe { (*a.add(i), *b.add(i)) };
        if x != y {
            return x as i32 - y as i32;
        }
        i += 1;
    }
    0
}

/// "Personality routine" de C++/SEH. O `core` vem pre-compilado com tabelas
/// de unwind que apontam para ela, mas com `panic = "abort"` nada e
/// desenrolado. Se o Windows chamar durante uma excecao (ex.: access
/// violation), responde `ExceptionContinueSearch` (1): "nao tratei, siga".
#[unsafe(no_mangle)]
pub extern "C" fn __CxxFrameHandler3() -> i32 {
    1
}
