#![allow(non_snake_case, clippy::upper_case_acronyms, non_camel_case_types)]

use windows_sys::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{ShowWindow, FindWindowExA, GetWindowThreadProcessId, GetWindow, GW_OWNER}
};

use neon::prelude::*;


fn find_hwnd_for_process(pid: u32) -> Option<HWND> {
    let mut hwnd = HWND::default();
    loop {
        hwnd = unsafe { FindWindowExA(HWND::default(), hwnd, std::ptr::null(), std::ptr::null()) };
        let mut check_pid = 0;

        unsafe { GetWindowThreadProcessId(hwnd, &mut check_pid) };

        unsafe {
            if GetWindow(hwnd, GW_OWNER) != 0 {
                continue;
            }
        }

        if check_pid == pid {
            return Some(hwnd);
        }

        if hwnd == HWND::default() {
            return None;
        }
    }
}

fn getWindowHandle(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let arg0: Handle<JsNumber> = cx.argument(0)?;
    let pid = arg0.value(&mut cx) as u32;

    let hwnd = find_hwnd_for_process(pid).unwrap();

    Ok(cx.number(hwnd as f64))
}

fn showWindow(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let arg0: Handle<JsNumber> = cx.argument(0)?;
    let hwnd = arg0.value(&mut cx) as HWND;

    let arg1: Handle<JsNumber> = cx.argument(1)?;
    let ncmdshow = arg1.value(&mut cx) as u32;

    let result: bool;
    unsafe {
        result = ShowWindow(hwnd, ncmdshow) != 0;
    }

    Ok(cx.boolean(result))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("getWindowHandle", getWindowHandle)?;
    cx.export_function("showWindow", showWindow)?;

    Ok(())
}