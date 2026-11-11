use core::{mem, ptr::{null, null_mut}};

use crate::windows::{
    CreateWindowExA, D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL_11_0, D3D11_SDK_VERSION, D3D11CreateDeviceAndSwapChain, D3DCompileFromFile, DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_R32G32B32A32_TYPELESS, DXGI_MODE_DESC, DXGI_MODE_SCALING_UNSPECIFIED, DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED, DXGI_SAMPLE_DESC, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_CHAIN_FLAG_NONPREROTATED, DXGI_SWAP_EFFECT_DISCARD, DXGI_SWAP_EFFECT_FLIP_DISCARD, DXGI_USAGE_RENDER_TARGET_OUTPUT, DefWindowProcA, HWND, ID3D11Device, ID3D11DeviceContext, ID3D11PixelShader, ID3D11RenderTargetView, ID3D11Texture2D, ID3D11VertexShader, ID3DBlob, IDXGISwapChain, IID_ID3D11Texture2D, LPCSTR, RegisterClassA, UINT, WNDCLASSA, WS_MAXIMIZE, WS_POPUP, WS_VISIBLE,
};

const TITLE: LPCSTR = c"d3d11".as_ptr();

const EXIT_CODE_SUCCESS: u32 = 0;

pub fn d3d11_main() -> u32 {
    unsafe {
        let mut swapchain: *mut IDXGISwapChain = null_mut();
        let mut device: *mut ID3D11Device = null_mut();
        let mut devicecontext: *mut ID3D11DeviceContext = null_mut();
        let mut rendertarget: *mut ID3D11Texture2D = null_mut();
        let mut rendertargetview: *mut ID3D11RenderTargetView = null_mut();
        let mut cso: *mut ID3DBlob = null_mut();
        let mut vertexshader: *mut ID3D11VertexShader = null_mut();
        let mut pixelshader: *mut ID3D11PixelShader = null_mut();
        let wndclass = WNDCLASSA {
            style: 0,
            lpfnWndProc: DefWindowProcA ,
            cbClsExtra: 0 ,
            cbWndExtra: 0 ,
            hInstance: null_mut() ,
            hIcon: null_mut() ,
            hCursor: null_mut() ,
            hbrBackground: null_mut() ,
            lpszMenuName: null_mut() ,
            lpszClassName: TITLE ,
        };
        RegisterClassA(&wndclass);
        let window: HWND = CreateWindowExA(0, TITLE, null(), 
        WS_POPUP | WS_VISIBLE | WS_MAXIMIZE, 0, 0, 0, 0, null_mut(), null_mut(), null_mut(), null_mut()
        );
        if window.is_null() {
            return 1;
        }
        
        let swapchaindesc = DXGI_SWAP_CHAIN_DESC {
            BufferDesc: DXGI_MODE_DESC {
                Width: 0,
                Height: 0,
                RefreshRate: mem::zeroed(),
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                Scaling: DXGI_MODE_SCALING_UNSPECIFIED,
            },
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 2,
            OutputWindow: window,
            Windowed: true as i32,
            SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
            Flags: 0,
        };
        
        D3D11CreateDeviceAndSwapChain(
            null_mut(),
            D3D_DRIVER_TYPE_HARDWARE, 
            null_mut(), 
            0, 
            null_mut(), 
            0, 
            D3D11_SDK_VERSION, 
            &raw mut swapchaindesc, 
            &raw mut swapchain, 
            &raw mut device, 
            null_mut(), 
            &raw mut devicecontext
        );

        let swapchain = &*swapchain;
        swapchain.GetDesc(&mut swapchaindesc);
        swapchain.GetBuffer(0, &IID_ID3D11Texture2D, (&raw mut rendertarget).cast());
        
        let device = &*device;
        device.CreateRenderTargetView(rendertarget.cast(), null_mut(), &raw mut rendertargetview);
        
        D3DCompileFromFile(c"gpu.hlsl".as_ptr(), null_mut(), null_mut(), c"vertex_shader".as_ptr(), c"vs_5_0".as_ptr(), 0, 0, &raw mut cso, null_mut());
        device->CreateVertexShader(cso->GetBufferPointer(), cso->GetBufferSize(), 0, &vertexshader);
        D3DCompileFromFile(L"gpu.hlsl", 0, 0, "pixel_shader", "ps_5_0", 0, 0, &cso, 0);
        device->CreatePixelShader(cso->GetBufferPointer(), cso->GetBufferSize(), 0, &pixelshader);
        D3D11_VIEWPORT viewport = { 0, 0, (float)swapchaindesc.BufferDesc.Width, (float)swapchaindesc.BufferDesc.Height, 0, 1 };
        devicecontext->IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        devicecontext->VSSetShader(vertexshader, 0, 0);
        devicecontext->RSSetViewports(1, &viewport);
        devicecontext->PSSetShader(pixelshader, 0, 0);
        devicecontext->OMSetRenderTargets(1, &rendertargetview, 0);
        devicecontext->Draw(3, 0);
        swapchain->Present(1, 0);
        GetMessageA((MSG*)WinMain, 0, WM_KEYFIRST, WM_KEYLAST); // PRESS ANY KEY TO EXIT
    }

    EXIT_CODE_SUCCESS
}
