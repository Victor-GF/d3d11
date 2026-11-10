use core::{mem, ptr::{null, null_mut}};

use crate::windows::{
    CreateWindowExA, D3D_FEATURE_LEVEL_11_0, DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_EFFECT_FLIP_DISCARD, DXGI_USAGE_RENDER_TARGET_OUTPUT, DefWindowProcA, IDXGISwapChain, LPCSTR, RegisterClassA, WNDCLASSA, WS_MAXIMIZE, WS_POPUP, WS_VISIBLE,
};

const TITLE: LPCSTR = c"d3d11".as_ptr();

const EXIT_CODE_SUCCESS: u32 = 0;

pub fn d3d11_main() -> u32 {
    // WNDCLASSA wndclass = { 0, DefWindowProcA, 0, 0, 0, 0, 0, 0, 0, TITLE };
    // Em Rust não existe "o resto é zero": todo campo precisa ser escrito.
    let wndclass = WNDCLASSA {
        style: 0,
        lpfnWndProc: DefWindowProcA,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: null_mut(),
        hIcon: null_mut(),
        hCursor: null_mut(),
        hbrBackground: null_mut(),
        lpszMenuName: null(),
        lpszClassName: TITLE,
    };

    unsafe {
        RegisterClassA(&wndclass);

        let window = CreateWindowExA(
            0,
            TITLE,
            TITLE,
            WS_POPUP | WS_MAXIMIZE | WS_VISIBLE,
            0, 0, 0, 0,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
        );

        if window.is_null() {
            return 1;
        }

        let featureLevels = [ D3D_FEATURE_LEVEL_11_0 ];
        let mut swapchaindesc: DXGI_SWAP_CHAIN_DESC = mem::zeroed();
        swapchaindesc.BufferDesc.Width = 0;
        swapchaindesc.BufferDesc.Height = 0;
        swapchaindesc.BufferDesc.Format = DXGI_FORMAT_B8G8R8A8_UNORM;
        swapchaindesc.SampleDesc.Count = 1;
        swapchaindesc.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
        swapchaindesc.BufferCount = 2;
        swapchaindesc.OutputWindow = window;
        swapchaindesc.Windowed     = true as i32;
        swapchaindesc.SwapEffect   = DXGI_SWAP_EFFECT_FLIP_DISCARD;

        let mut swapchain: *mut IDXGISwapChain = null_mut();
        let mut device: *mut ID3D11De

    }


    EXIT_CODE_SUCCESS
}
