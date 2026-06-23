pub fn set_badge_count(window: &tauri::WebviewWindow, count: i32) {
    #[cfg(target_os = "macos")]
    {
        let _ = window.set_badge_count(Some(count as i64));
    }

    #[cfg(target_os = "windows")]
    {
        set_badge_count_windows(window, count);
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        let badge = if count > 0 { Some(count as i64) } else { None };
        let _ = window.set_badge_count(badge);
    }
}

#[cfg(target_os = "windows")]
fn set_badge_count_windows(window: &tauri::WebviewWindow, count: i32) {
    use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};
    use windows::Win32::UI::Shell::{ITaskbarList3, TaskbarList};

    let Ok(taskbar) =
        (unsafe { CoCreateInstance::<_, ITaskbarList3>(&TaskbarList, None, CLSCTX_ALL) })
    else {
        return;
    };

    let hwnd = match window.hwnd() {
        Ok(hwnd) if !hwnd.0.is_null() => hwnd,
        _ => return,
    };

    if count == 0 {
        let _ = unsafe {
            taskbar.SetOverlayIcon(
                hwnd,
                windows::Win32::UI::WindowsAndMessaging::HICON::default(),
                windows::core::w!(""),
            )
        };
        return;
    }

    if let Ok(hicon) = create_count_icon(count) {
        let _ = unsafe { taskbar.SetOverlayIcon(hwnd, hicon, windows::core::w!("")) };
        unsafe {
            let _ = windows::Win32::UI::WindowsAndMessaging::DestroyIcon(hicon);
        }
    }
}

#[cfg(target_os = "windows")]
fn create_count_icon(
    count: i32,
) -> windows::core::Result<windows::Win32::UI::WindowsAndMessaging::HICON> {
    use windows::Win32::Foundation::{COLORREF, RECT};
    use windows::Win32::Graphics::Gdi::*;
    use windows::Win32::UI::WindowsAndMessaging::*;
    use windows::core::w;

    unsafe {
        let size = 32i32;

        let hdc_screen = GetDC(None);
        let hdc = CreateCompatibleDC(Some(hdc_screen));

        let hbmp = CreateCompatibleBitmap(hdc_screen, size, size);

        let old_bmp = SelectObject(hdc, hbmp.into());

        // Red circle background
        let red_brush = CreateSolidBrush(COLORREF(0x000000FF));
        let old_brush = SelectObject(hdc, red_brush.into());
        let old_pen = SelectObject(hdc, GetStockObject(NULL_PEN));
        let _ = Ellipse(hdc, 0, 0, size, size);
        SelectObject(hdc, old_pen);
        SelectObject(hdc, old_brush);
        let _ = DeleteObject(red_brush.into());

        // White count text
        SetBkMode(hdc, TRANSPARENT);
        SetTextColor(hdc, COLORREF(0x00FFFFFF));

        let text: String = if count > 99 {
            "99+".into()
        } else {
            count.to_string()
        };
        let mut wide_text: Vec<u16> = text.encode_utf16().collect();
        wide_text.push(0);

        let font_size = if count > 99 { 14 } else { 20 };
        let hfont = CreateFontW(
            font_size,
            0,
            0,
            0,
            FW_BOLD.0 as i32,
            0,
            0,
            0,
            DEFAULT_CHARSET,
            OUT_DEFAULT_PRECIS,
            CLIP_DEFAULT_PRECIS,
            DEFAULT_QUALITY,
            0,
            w!("Segoe UI"),
        );
        let old_font = SelectObject(hdc, hfont.into());

        let mut rect = RECT {
            left: 0,
            top: 0,
            right: size,
            bottom: size,
        };
        DrawTextW(
            hdc,
            &mut wide_text,
            &mut rect,
            DRAW_TEXT_FORMAT(DT_CENTER.0 | DT_VCENTER.0 | DT_SINGLELINE.0),
        );

        // Restore and clean up DC
        SelectObject(hdc, old_font);
        let _ = DeleteObject(hfont.into());
        SelectObject(hdc, old_bmp);

        // Mask: all-white = fully opaque (32x32x1bpp = 128 bytes)
        let mask_bits = vec![0xFFu8; (size * size / 8) as usize];
        let hbm_mask = CreateBitmap(
            size,
            size,
            1,
            1,
            Some(mask_bits.as_ptr() as *const std::ffi::c_void),
        );

        let icon_info = ICONINFO {
            fIcon: windows::core::BOOL::from(true),
            xHotspot: 0,
            yHotspot: 0,
            hbmMask: hbm_mask,
            hbmColor: hbmp,
        };

        let hicon = CreateIconIndirect(&icon_info)?;

        let _ = DeleteDC(hdc);
        ReleaseDC(None, hdc_screen);
        let _ = DeleteObject(hbm_mask.into());
        let _ = DeleteObject(hbmp.into());

        Ok(hicon)
    }
}
