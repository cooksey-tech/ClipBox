use windows_sys::Win32::Graphics::Gdi::{PAINTSTRUCT, BeginPaint, CreatePen, PS_SOLID, SelectObject, Ellipse, DeleteObject, EndPaint};
use windows_sys::Win32::Foundation::{HWND, WPARAM, LPARAM, LRESULT};
use windows_sys::Win32::UI::WindowsAndMessaging::{PostQuitMessage, GetClientRect, DefWindowProcW};

pub(crate) extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => {
            // Handle window destruction
            unsafe { PostQuitMessage(0) };
            return 0;
        }
        WM_PAINT => {
            // Handle window painting
            let mut ps: PAINTSTRUCT = unsafe { std::mem::zeroed() };
            let hdc = unsafe { BeginPaint(hwnd, &mut ps) };

            let pen = unsafe {
                CreatePen(PS_SOLID, 1, 0)
            };
            let old_pen = unsafe {
                SelectObject(hdc, pen)
            };

            // dimesions of button
            let mut rect = unsafe { std::mem::zeroed() };
            unsafe { GetClientRect(hwnd, &mut rect) };

            // draw a circle for the button
            unsafe { Ellipse(hdc, rect.left, rect.top, rect.right, rect.bottom) };

            // Clean up
            unsafe {
                SelectObject(hdc, old_pen);
                DeleteObject(pen);
                EndPaint(hwnd, &ps);
            }

            return 0;
        }
        _ => {
            // Handle other messages or pass to default handler
            return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
        }
    }
}
