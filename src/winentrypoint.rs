#![no_std]
#![no_main]

mod crt;
mod windows;
mod d3d11;

// --- Ponto de entrada -------------------------------------------------------
// Chamado direto pelo loader do Windows (/ENTRY no .cargo/config.toml).
#[unsafe(no_mangle)]
pub extern "system" fn mainCRTStartup() -> ! {
    let exit_code = d3d11::d3d11_main();
    unsafe { windows::ExitProcess(exit_code) }
}

// --- Obrigatórios em no_std -------------------------------------------------
// O que fazer num panic: sem std, somos nós que decidimos. Aqui: encerra o processo.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { windows::ExitProcess(1) }
}

// O compilador MSVC/LLVM referencia este símbolo sempre que o programa usa
// float (f32/f64). Normalmente vem da CRT; como não temos CRT, definimos.
#[unsafe(no_mangle)]
pub static _fltused: i32 = 0;
