//! Teste do `src/crt.rs` no ambiente real do projeto: `no_std`, sem CRT e com
//! as mesmas flags de link do app. Compara cada funcao com uma implementacao
//! de referencia "obviamente correta" (loops byte a byte), em ~2,3 milhoes de
//! casos: tamanhos, alinhamentos, sobreposicoes e valores diferentes.
//!
//! Rodar nos dois perfis (debug e release geram codigo diferente):
//!     cargo run --bin crt_check
//!     cargo run --bin crt_check --release
//! Sucesso: imprime "OK" e sai com codigo 0. Falha: imprime o caso e sai com 1.
//!
//! Por que as referencias usam `read_volatile`/`write_volatile`: um loop comum
//! seria trocado pelo LLVM por uma chamada a memset/memcpy/memcmp, ou seja,
//! pelas proprias funcoes que estamos testando. Volatile proibe essa troca.
//! Pelo mesmo motivo, copiar e comparar buffers aqui usa essas referencias,
//! nunca `a = b` ou `a == b`.

#![no_std]
#![no_main]

#[path = "../crt.rs"]
mod crt;
#[path = "../windows/mod.rs"]
mod windows;

use core::arch::asm;
use core::fmt::Write;
use core::ptr::{null, null_mut, read_volatile, write_volatile};

use crt::{memcmp, memcpy, memmove, memset};
use windows::{ExitProcess, GetStdHandle, WriteFile};

const STD_OUTPUT_HANDLE: u32 = -11i32 as u32;
const LEN: usize = 640;

/// Se a condicao falhar: imprime o caso e encerra com codigo 1.
macro_rules! check {
    ($cond:expr, $($msg:tt)*) => {
        if !$cond {
            let _ = write!(Out, "FALHOU: ");
            let _ = writeln!(Out, $($msg)*);
            unsafe { ExitProcess(1) }
        }
    };
}

// --- Ponto de entrada -------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "system" fn mainCRTStartup() -> ! {
    let cases = test_memset() + test_memcpy() + test_memmove() + test_memcmp();
    let _ = writeln!(Out, "OK: {cases} casos passaram (memset, memcpy, memmove, memcmp)");
    unsafe { ExitProcess(0) }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = writeln!(Out, "PANIC: {info}");
    unsafe { ExitProcess(101) }
}

// Mesmo motivo do winentrypoint.rs: o `core::fmt` referencia este simbolo.
#[unsafe(no_mangle)]
pub static _fltused: i32 = 0;

// --- Testes -----------------------------------------------------------------

/// Varios deslocamentos, tamanhos e valores (inclusive int > 255, onde so o
/// byte baixo vale). O buffer inteiro e comparado, entao uma escrita fora de
/// [dest, dest+n) tambem e detectada.
fn test_memset() -> u64 {
    let mut rng = Rng(0x9E3779B97F4A7C15);
    let mut base = [0u8; LEN];
    let mut a = [0u8; LEN];
    let mut b = [0u8; LEN];
    let mut cases = 0;
    for off in 0..32 {
        for n in 0..=300 {
            for c in [0i32, 0xAB, -1, 0x1FF, 0x7F] {
                rng.fill(&mut base);
                ref_copy(a.as_mut_ptr(), base.as_ptr(), LEN);
                ref_copy(b.as_mut_ptr(), base.as_ptr(), LEN);
                let pa = unsafe { a.as_mut_ptr().add(off) };
                let r = unsafe { memset(pa, c, n) };
                ref_fill(unsafe { b.as_mut_ptr().add(off) }, c as u8, n);
                check!(r == pa, "memset retornou ponteiro errado (off={off} n={n})");
                check!(same(&a, &b), "memset off={off} n={n} c={c}");
                cases += 1;
            }
        }
    }
    cases
}

/// Origem e destino em buffers separados, todas as combinacoes de alinhamento.
fn test_memcpy() -> u64 {
    let mut rng = Rng(0xD1B54A32D192ED03);
    let mut src = [0u8; LEN];
    let mut base = [0u8; LEN];
    let mut a = [0u8; LEN];
    let mut b = [0u8; LEN];
    let mut cases = 0;
    for so in 0..32 {
        for d_o in 0..32 {
            for n in 0..=300 {
                rng.fill(&mut src);
                rng.fill(&mut base);
                ref_copy(a.as_mut_ptr(), base.as_ptr(), LEN);
                ref_copy(b.as_mut_ptr(), base.as_ptr(), LEN);
                let pa = unsafe { a.as_mut_ptr().add(d_o) };
                let r = unsafe { memcpy(pa, src.as_ptr().add(so), n) };
                ref_copy(unsafe { b.as_mut_ptr().add(d_o) }, unsafe { src.as_ptr().add(so) }, n);
                check!(r == pa, "memcpy retornou ponteiro errado (so={so} do={d_o} n={n})");
                check!(same(&a, &b), "memcpy so={so} do={d_o} n={n}");
                cases += 1;
            }
        }
    }
    cases
}

/// Origem e destino no MESMO buffer, com sobreposicao nos dois sentidos
/// (incluindo dest == src e distancia de 1 byte). Depois de cada chamada,
/// confere que a Direction Flag voltou a 0, como a ABI exige.
fn test_memmove() -> u64 {
    let mut rng = Rng(0x2545F4914F6CDD1D);
    let mut base = [0u8; LEN];
    let mut a = [0u8; LEN];
    let mut b = [0u8; LEN];
    let mut cases = 0;
    for so in 0..80 {
        for d_o in 0..80 {
            let max_n = if LEN - so.max(d_o) < 300 { LEN - so.max(d_o) } else { 300 };
            for n in 0..=max_n {
                rng.fill(&mut base);
                ref_copy(a.as_mut_ptr(), base.as_ptr(), LEN);
                ref_copy(b.as_mut_ptr(), base.as_ptr(), LEN);
                let pa = unsafe { a.as_mut_ptr().add(d_o) };
                let r = unsafe { memmove(pa, a.as_ptr().add(so), n) };
                check!(!direction_flag(), "memmove deixou DF=1 (so={so} do={d_o} n={n})");
                let pb = b.as_mut_ptr();
                ref_copy(unsafe { pb.add(d_o) }, unsafe { pb.add(so) }, n);
                check!(r == pa, "memmove retornou ponteiro errado (so={so} do={d_o} n={n})");
                check!(same(&a, &b), "memmove so={so} do={d_o} n={n}");
                cases += 1;
            }
        }
    }
    cases
}

/// O padrao C so exige o SINAL certo. Bytes >= 0x80 testam que a comparacao
/// e sem sinal (0xFF > 0x01).
fn test_memcmp() -> u64 {
    let mut rng = Rng(0xA0761D6478BD642F);
    let mut a = [0u8; LEN];
    let mut b = [0u8; LEN];
    let mut cases = 0;
    for n in 0..=200 {
        for _ in 0..200 {
            rng.fill(&mut a);
            ref_copy(b.as_mut_ptr(), a.as_ptr(), LEN);
            // 3 em 4 vezes, muda um byte dentro dos n primeiros (pode sair igual).
            if n > 0 && rng.next() % 4 != 0 {
                let k = rng.next() as usize % n;
                b[k] = rng.next();
            }
            let got = unsafe { memcmp(a.as_ptr(), b.as_ptr(), n) }.signum();
            let want = ref_cmp(a.as_ptr(), b.as_ptr(), n);
            check!(got == want, "memcmp n={n}: deu {got}, esperado {want}");
            cases += 1;
        }
    }
    let got = unsafe { memcmp([0xFFu8].as_ptr(), [0x01u8].as_ptr(), 1) };
    check!(got > 0, "memcmp tratou 0xFF como menor que 0x01");
    let got = unsafe { memcmp(null(), null(), 0) };
    check!(got == 0, "memcmp com n=0 deveria dar 0");
    cases + 2
}

// --- Referencias "obviamente corretas" (byte a byte, volatile) --------------

fn ref_fill(p: *mut u8, c: u8, n: usize) {
    for i in 0..n {
        unsafe { write_volatile(p.add(i), c) };
    }
}

/// Copia correta mesmo com sobreposicao: se o destino esta depois da origem,
/// anda de tras para frente.
fn ref_copy(dst: *mut u8, src: *const u8, n: usize) {
    if (dst as usize) <= (src as usize) {
        for i in 0..n {
            unsafe { write_volatile(dst.add(i), read_volatile(src.add(i))) };
        }
    } else {
        for i in (0..n).rev() {
            unsafe { write_volatile(dst.add(i), read_volatile(src.add(i))) };
        }
    }
}

/// -1, 0 ou 1, comparando como bytes sem sinal.
fn ref_cmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let (x, y) = unsafe { (read_volatile(a.add(i)), read_volatile(b.add(i))) };
        if x != y {
            return if x < y { -1 } else { 1 };
        }
    }
    0
}

fn same(a: &[u8; LEN], b: &[u8; LEN]) -> bool {
    ref_cmp(a.as_ptr(), b.as_ptr(), LEN) == 0
}

/// Bit 10 do RFLAGS = Direction Flag.
fn direction_flag() -> bool {
    let flags: u64;
    unsafe { asm!("pushfq", "pop {}", out(reg) flags) };
    flags & (1 << 10) != 0
}

// --- Utilitarios ------------------------------------------------------------

/// Gerador pseudoaleatorio xorshift: deterministico, entao uma falha sempre
/// se repete igual.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u8 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 as u8
    }

    fn fill(&mut self, buf: &mut [u8; LEN]) {
        for x in buf.iter_mut() {
            *x = self.next();
        }
    }
}

/// Escreve no terminal (stdout herdado do `cargo run`).
struct Out;

impl Write for Out {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            let mut written = 0;
            WriteFile(GetStdHandle(STD_OUTPUT_HANDLE), s.as_ptr().cast(), s.len() as u32, &mut written, null_mut());
        }
        Ok(())
    }
}
