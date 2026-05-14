#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
#![allow(non_snake_case)]
#![allow(static_mut_refs)]

mod localization;

use std::ffi::{OsStr, c_void};
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use std::process::Command;
use std::ptr::{null, null_mut};
use std::thread;

use localization::Locale;

type Bool = i32;
type Dword = u32;
type Hbrush = isize;
type Hcursor = isize;
type Hfont = isize;
type Hinstance = isize;
type Hicon = isize;
type Hmenu = isize;
type Hwnd = isize;
type Lparam = isize;
type Lresult = isize;
type Lpvoid = *mut c_void;
type Uint = u32;
type Wparam = usize;

const CW_USEDEFAULT: i32 = 0x80000000u32 as i32;
const WS_OVERLAPPEDWINDOW: Dword = 0x00CF0000;
const WS_CHILD: Dword = 0x40000000;
const WS_VISIBLE: Dword = 0x10000000;
const WS_BORDER: Dword = 0x00800000;
const WS_VSCROLL: Dword = 0x00200000;
const WS_TABSTOP: Dword = 0x00010000;
const ES_LEFT: Dword = 0x0000;
const ES_MULTILINE: Dword = 0x0004;
const ES_AUTOVSCROLL: Dword = 0x0040;
const ES_AUTOHSCROLL: Dword = 0x0080;
const ES_READONLY: Dword = 0x0800;
const BS_PUSHBUTTON: Dword = 0x00000000;
const BS_AUTOCHECKBOX: Dword = 0x00000003;
const CBS_DROPDOWNLIST: Dword = 0x0003;
const CBS_HASSTRINGS: Dword = 0x0200;
const OFN_EXPLORER: Dword = 0x00080000;
const OFN_FILEMUSTEXIST: Dword = 0x00001000;
const OFN_PATHMUSTEXIST: Dword = 0x00000800;
const COLOR_BTNFACE: isize = 15;
const SW_SHOW: i32 = 5;

const WM_CREATE: Uint = 0x0001;
const WM_DESTROY: Uint = 0x0002;
const WM_COMMAND: Uint = 0x0111;
const WM_APP_LOG: Uint = 0x8001;
const WM_APP_DONE: Uint = 0x8002;
const WM_SETICON: Uint = 0x0080;
const WM_GETTEXT: Uint = 0x000D;
const WM_GETTEXTLENGTH: Uint = 0x000E;
const WM_ENABLE: Uint = 0x000A;
const WM_SETFONT: Uint = 0x0030;
const BM_GETCHECK: Uint = 0x00F0;
const BM_SETCHECK: Uint = 0x00F1;
const BST_CHECKED: Wparam = 1;
const CB_ADDSTRING: Uint = 0x0143;
const CB_GETCURSEL: Uint = 0x0147;
const CB_SETCURSEL: Uint = 0x014E;
const EM_SETSEL: Uint = 0x00B1;
const EM_REPLACESEL: Uint = 0x00C2;
const EM_LIMITTEXT: Uint = 0x00C5;
const ICON_SMALL: Wparam = 0;
const ICON_BIG: Wparam = 1;

const FW_NORMAL: i32 = 400;
const DEFAULT_CHARSET: Dword = 1;
const OUT_DEFAULT_PRECIS: Dword = 0;
const CLIP_DEFAULT_PRECIS: Dword = 0;
const DEFAULT_QUALITY: Dword = 0;
const DEFAULT_PITCH: Dword = 0;

const IDC_OPERATION: u16 = 100;
const IDC_BROWSE_DA: u16 = 101;
const IDC_BROWSE_PL: u16 = 102;
const IDC_BROWSE_AUTH: u16 = 103;
const IDC_BROWSE_FILE: u16 = 104;
const IDC_BROWSE_OUTPUT: u16 = 105;
const IDC_BUILD: u16 = 106;
const IDC_RUN: u16 = 107;
const IDC_VERSION: u16 = 108;
const IDC_EXPLOIT: u16 = 109;

const ID_DA: u16 = 200;
const ID_PL: u16 = 201;
const ID_AUTH: u16 = 202;
const ID_PARTITION: u16 = 203;
const ID_FILE: u16 = 204;
const ID_OUTPUT: u16 = 205;
const ID_ADDRESS: u16 = 206;
const ID_LENGTH: u16 = 207;
const ID_VALUE: u16 = 208;
const ID_SKIP: u16 = 209;
const ID_EXTRA: u16 = 210;
const ID_VERBOSE: u16 = 211;
const ID_USB_LOG: u16 = 212;
const ID_IGNORE_MISSING: u16 = 213;
const ID_LOG: u16 = 214;

#[repr(C)]
struct WndClassW {
    style: Uint,
    lpfnWndProc: Option<unsafe extern "system" fn(Hwnd, Uint, Wparam, Lparam) -> Lresult>,
    cbClsExtra: i32,
    cbWndExtra: i32,
    hInstance: Hinstance,
    hIcon: Hicon,
    hCursor: Hcursor,
    hbrBackground: Hbrush,
    lpszMenuName: *const u16,
    lpszClassName: *const u16,
}

#[repr(C)]
struct Point {
    x: i32,
    y: i32,
}

#[repr(C)]
struct Msg {
    hwnd: Hwnd,
    message: Uint,
    wParam: Wparam,
    lParam: Lparam,
    time: Dword,
    pt: Point,
}

#[repr(C)]
struct OpenFileNameW {
    lStructSize: Dword,
    hwndOwner: Hwnd,
    hInstance: Hinstance,
    lpstrFilter: *const u16,
    lpstrCustomFilter: *mut u16,
    nMaxCustFilter: Dword,
    nFilterIndex: Dword,
    lpstrFile: *mut u16,
    nMaxFile: Dword,
    lpstrFileTitle: *mut u16,
    nMaxFileTitle: Dword,
    lpstrInitialDir: *const u16,
    lpstrTitle: *const u16,
    Flags: Dword,
    nFileOffset: u16,
    nFileExtension: u16,
    lpstrDefExt: *const u16,
    lCustData: Lparam,
    lpfnHook: Lpvoid,
    lpTemplateName: *const u16,
    pvReserved: Lpvoid,
    dwReserved: Dword,
    FlagsEx: Dword,
}

#[link(name = "user32")]
unsafe extern "system" {
    fn RegisterClassW(lpWndClass: *const WndClassW) -> u16;
    fn CreateWindowExW(
        dwExStyle: Dword,
        lpClassName: *const u16,
        lpWindowName: *const u16,
        dwStyle: Dword,
        X: i32,
        Y: i32,
        nWidth: i32,
        nHeight: i32,
        hWndParent: Hwnd,
        hMenu: Hmenu,
        hInstance: Hinstance,
        lpParam: Lpvoid,
    ) -> Hwnd;
    fn DefWindowProcW(hWnd: Hwnd, Msg: Uint, wParam: Wparam, lParam: Lparam) -> Lresult;
    fn DispatchMessageW(lpMsg: *const Msg) -> Lresult;
    fn GetMessageW(lpMsg: *mut Msg, hWnd: Hwnd, wMsgFilterMin: Uint, wMsgFilterMax: Uint) -> Bool;
    fn LoadCursorW(hInstance: Hinstance, lpCursorName: *const u16) -> Hcursor;
    fn LoadIconW(hInstance: Hinstance, lpIconName: *const u16) -> Hicon;
    fn MessageBoxW(hWnd: Hwnd, lpText: *const u16, lpCaption: *const u16, uType: Uint) -> i32;
    fn PostMessageW(hWnd: Hwnd, Msg: Uint, wParam: Wparam, lParam: Lparam) -> Bool;
    fn PostQuitMessage(nExitCode: i32);
    fn SendMessageW(hWnd: Hwnd, Msg: Uint, wParam: Wparam, lParam: Lparam) -> Lresult;
    fn SetWindowTextW(hWnd: Hwnd, lpString: *const u16) -> Bool;
    fn ShowWindow(hWnd: Hwnd, nCmdShow: i32) -> Bool;
    fn TranslateMessage(lpMsg: *const Msg) -> Bool;
    fn UpdateWindow(hWnd: Hwnd) -> Bool;
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetModuleHandleW(lpModuleName: *const u16) -> Hinstance;
    fn GetUserDefaultUILanguage() -> u16;
}

#[link(name = "comdlg32")]
unsafe extern "system" {
    fn GetOpenFileNameW(param0: *mut OpenFileNameW) -> Bool;
}

#[link(name = "gdi32")]
unsafe extern "system" {
    fn CreateFontW(
        cHeight: i32,
        cWidth: i32,
        cEscapement: i32,
        cOrientation: i32,
        cWeight: i32,
        bItalic: Dword,
        bUnderline: Dword,
        bStrikeOut: Dword,
        iCharSet: Dword,
        iOutPrecision: Dword,
        iClipPrecision: Dword,
        iQuality: Dword,
        iPitchAndFamily: Dword,
        pszFaceName: *const u16,
    ) -> Hfont;
    fn DeleteObject(ho: isize) -> Bool;
}

#[derive(Default)]
struct App {
    hwnd: Hwnd,
    font: Hfont,
    operation: Hwnd,
    da: Hwnd,
    preloader: Hwnd,
    auth: Hwnd,
    partition: Hwnd,
    file: Hwnd,
    output: Hwnd,
    address: Hwnd,
    length: Hwnd,
    value: Hwnd,
    skip: Hwnd,
    extra: Hwnd,
    verbose: Hwnd,
    usb_log: Hwnd,
    ignore_missing: Hwnd,
    log: Hwnd,
    run: Hwnd,
}

static mut APP: Option<App> = None;
static mut UI_FONT: Hfont = 0;
static mut UI_LOCALE: Locale = Locale::English;

const OPERATION_KEYS: [&str; 20] = [
    "op_pgpt",
    "op_read",
    "op_read_flash",
    "op_write",
    "op_write_flash",
    "op_erase",
    "op_format",
    "op_read_all",
    "op_write_all",
    "op_read_offset",
    "op_write_offset",
    "op_unlock",
    "op_lock",
    "op_reboot_normal",
    "op_reboot_fastboot",
    "op_shutdown",
    "op_slot_a",
    "op_slot_b",
    "op_rpmb_read",
    "op_xflash_rsc",
];

fn wide(text: &str) -> Vec<u16> {
    OsStr::new(text).encode_wide().chain(Some(0)).collect()
}

fn loword(value: Wparam) -> u16 {
    (value & 0xffff) as u16
}

fn primary_lang(langid: u16) -> u16 {
    langid & 0x03ff
}

unsafe fn detect_locale() -> Locale {
    unsafe {
        match primary_lang(GetUserDefaultUILanguage()) {
            0x04 => Locale::Chinese,
            0x11 => Locale::Japanese,
            0x12 => Locale::Korean,
            _ => Locale::English,
        }
    }
}

fn font_for(locale: Locale) -> &'static str {
    match locale {
        Locale::Japanese => "Yu Gothic UI",
        Locale::Chinese => "Microsoft YaHei UI",
        Locale::Korean => "Malgun Gothic",
        Locale::English => "Segoe UI",
    }
}

unsafe fn create_ui_font(locale: Locale) -> Hfont {
    unsafe {
        CreateFontW(
            -13,
            0,
            0,
            0,
            FW_NORMAL,
            0,
            0,
            0,
            DEFAULT_CHARSET,
            OUT_DEFAULT_PRECIS,
            CLIP_DEFAULT_PRECIS,
            DEFAULT_QUALITY,
            DEFAULT_PITCH,
            wide(font_for(locale)).as_ptr(),
        )
    }
}

unsafe fn tr(key: &str) -> &'static str {
    unsafe {
        localization::tr(UI_LOCALE, key)
    }
}

unsafe fn create_control(
    parent: Hwnd,
    class: &str,
    text: &str,
    style: Dword,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    id: u16,
) -> Hwnd {
    unsafe {
        let hwnd = CreateWindowExW(
            0,
            wide(class).as_ptr(),
            wide(text).as_ptr(),
            WS_CHILD | WS_VISIBLE | style,
            x,
            y,
            w,
            h,
            parent,
            id as Hmenu,
            GetModuleHandleW(null()),
            null_mut(),
        );
        if UI_FONT != 0 {
            SendMessageW(hwnd, WM_SETFONT, UI_FONT as Wparam, 1);
        }
        hwnd
    }
}

unsafe fn add_label(parent: Hwnd, text: &str, x: i32, y: i32, w: i32) -> Hwnd {
    unsafe {
        create_control(parent, "STATIC", text, 0, x, y, w, 20, 0)
    }
}

unsafe fn add_edit(parent: Hwnd, x: i32, y: i32, w: i32, id: u16) -> Hwnd {
    unsafe { create_control(parent, "EDIT", "", WS_BORDER | ES_LEFT | ES_AUTOHSCROLL | WS_TABSTOP, x, y, w, 24, id) }
}

unsafe fn get_text(hwnd: Hwnd) -> String {
    unsafe {
        let len = SendMessageW(hwnd, WM_GETTEXTLENGTH, 0, 0) as usize;
        let mut buf = vec![0u16; len + 1];
        SendMessageW(hwnd, WM_GETTEXT, buf.len(), buf.as_mut_ptr() as Lparam);
        String::from_utf16_lossy(&buf[..len])
    }
}

unsafe fn set_text(hwnd: Hwnd, value: &str) {
    unsafe {
        SetWindowTextW(hwnd, wide(value).as_ptr());
    }
}

unsafe fn checked(hwnd: Hwnd) -> bool {
    unsafe { SendMessageW(hwnd, BM_GETCHECK, 0, 0) as Wparam == BST_CHECKED }
}

unsafe fn append_log(text: &str) {
    unsafe {
        if let Some(app) = &APP {
            let ws = wide(text);
            SendMessageW(app.log, EM_SETSEL, usize::MAX, -1);
            SendMessageW(app.log, EM_REPLACESEL, 0, ws.as_ptr() as Lparam);
        }
    }
}

unsafe fn operation_index() -> usize {
    unsafe {
        APP.as_ref()
            .map(|app| SendMessageW(app.operation, CB_GETCURSEL, 0, 0) as usize)
            .unwrap_or(0)
    }
}

unsafe fn current_args() -> Result<Vec<String>, String> {
    unsafe {
        let Some(app) = &APP else {
            return Err(tr("gui_not_ready").to_string());
        };

        let mut args = global_args(app);

        let partition = get_text(app.partition);
        let file = get_text(app.file);
        let output = get_text(app.output);
        let address = get_text(app.address);
        let length = get_text(app.length);
        let value = get_text(app.value);
        let skip = get_text(app.skip);

        match operation_index() {
            0 => args.push("pgpt".to_string()),
            1 => args.extend(["read".to_string(), need(&partition, tr("label_partition"))?, need(&output, tr("label_output_file"))?]),
            2 => args.extend(["read-flash".to_string(), need(&partition, tr("label_partition"))?, need(&output, tr("label_output_file"))?]),
            3 => args.extend(["write".to_string(), need(&partition, tr("label_partition"))?, need(&file, tr("label_input_file"))?]),
            4 => args.extend(["write-flash".to_string(), need(&partition, tr("label_partition"))?, need(&file, tr("label_input_file"))?]),
            5 => args.extend(["erase".to_string(), need(&partition, tr("label_partition"))?]),
            6 => args.extend(["format".to_string(), need(&partition, tr("label_partition"))?]),
            7 => {
                args.extend(["read-all".to_string(), need(&output, tr("label_output_dir"))?]);
                if !skip.trim().is_empty() {
                    args.extend(["--skip".to_string(), skip]);
                }
            }
            8 => {
                args.extend(["write-all".to_string(), need(&file, tr("label_input_dir"))?]);
                if !skip.trim().is_empty() {
                    args.extend(["--skip".to_string(), skip]);
                }
                if checked(app.ignore_missing) {
                    args.push("--ignore-missing".to_string());
                }
            }
            9 => args.extend(["read-offset".to_string(), need(&address, tr("label_address"))?, need(&length, tr("label_length"))?, need(&output, tr("label_output_file"))?]),
            10 => args.extend(["write-offset".to_string(), need(&address, tr("label_address"))?, need(&length, tr("label_length"))?, need(&file, tr("label_input_file"))?]),
            11 => args.extend(["seccfg".to_string(), "unlock".to_string()]),
            12 => args.extend(["seccfg".to_string(), "lock".to_string()]),
            13 => args.extend(["reboot".to_string(), "normal".to_string()]),
            14 => args.extend(["reboot".to_string(), "fastboot".to_string()]),
            15 => args.push("shutdown".to_string()),
            16 => args.extend(["set-active-slot".to_string(), "a".to_string()]),
            17 => args.extend(["set-active-slot".to_string(), "b".to_string()]),
            18 => {
                args.extend(["rpmb".to_string(), "read".to_string()]);
                if !address.trim().is_empty() {
                    args.extend(["--start-sector".to_string(), address]);
                }
                if !length.trim().is_empty() {
                    args.extend(["--num-sectors".to_string(), length]);
                }
                args.extend(["--region".to_string(), if value.trim().is_empty() { "0".to_string() } else { value }]);
                args.push(need(&output, tr("label_output_file"))?);
            }
            19 => args.extend(["x-flash".to_string(), "rsc-flash".to_string(), need(&partition, tr("label_partition"))?, need(&file, tr("label_input_file"))?]),
            _ => return Err(tr("unknown_operation").to_string()),
        }

        let extra = get_text(app.extra);
        if !extra.trim().is_empty() {
            args.extend(split_extra(&extra));
        }

        Ok(args)
    }
}

unsafe fn global_args(app: &App) -> Vec<String> {
    unsafe {
        let mut args = vec!["--cli".to_string()];
        let da = get_text(app.da);
        let pl = get_text(app.preloader);
        let auth = get_text(app.auth);
        if !da.trim().is_empty() {
            args.extend(["--da".to_string(), da]);
        }
        if !pl.trim().is_empty() {
            args.extend(["--pl".to_string(), pl]);
        }
        if !auth.trim().is_empty() {
            args.extend(["--auth".to_string(), auth]);
        }
        if checked(app.verbose) {
            args.push("--verbose".to_string());
        }
        if checked(app.usb_log) {
            args.push("--usb-log".to_string());
        }
        args
    }
}

fn need(value: &str, label: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(format!("{label} {}", unsafe { tr("required_suffix") }))
    } else {
        Ok(trimmed.to_string())
    }
}

fn split_extra(value: &str) -> Vec<String> {
    value.split_whitespace().map(ToOwned::to_owned).collect()
}

fn antumbra_path() -> PathBuf {
    let exe = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    let dir = exe.parent().map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
    let local = dir.join("antumbra.exe");
    if local.exists() {
        local
    } else {
        PathBuf::from("antumbra.exe")
    }
}

unsafe fn build_command_preview() {
    unsafe {
        match current_args() {
            Ok(args) => {
                append_log("\r\n> ");
                append_log(&format!("{} {}\r\n", antumbra_path().display(), quote_args(&args)));
            }
            Err(e) => message(&e),
        }
    }
}

fn quote_args(args: &[String]) -> String {
    args.iter()
        .map(|arg| {
            if arg.contains(' ') {
                format!("\"{}\"", arg.replace('"', "\\\""))
            } else {
                arg.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

unsafe fn run_command() {
    unsafe {
        let args = match current_args() {
            Ok(args) => args,
            Err(e) => {
                message(&e);
                return;
            }
        };
        let Some(app) = &APP else {
            return;
        };
        SendMessageW(app.run, WM_ENABLE, 0, 0);
        append_log(tr("running"));
        append_log(&format!("{} {}\r\n\r\n", antumbra_path().display(), quote_args(&args)));
        let hwnd = app.hwnd;
        let launch_failed = tr("launch_failed").to_string();
        let place_antumbra = tr("place_antumbra").to_string();
        thread::spawn(move || {
            let output = Command::new(antumbra_path()).args(&args).output();
            let log = match output {
                Ok(out) => {
                    let mut text = String::new();
                    text.push_str(&String::from_utf8_lossy(&out.stdout));
                    text.push_str(&String::from_utf8_lossy(&out.stderr));
                    text.push_str(&format!("\r\nExit status: {}\r\n", out.status));
                    text
                }
                Err(e) => format!("{launch_failed}: {e}\r\n{place_antumbra}\r\n"),
            };
            let boxed = Box::new(log);
            PostMessageW(hwnd, WM_APP_LOG, 0, Box::into_raw(boxed) as Lparam);
            PostMessageW(hwnd, WM_APP_DONE, 0, 0);
        });
    }
}

unsafe fn check_version() {
    unsafe {
        let Some(app) = &APP else {
            return;
        };
        append_log(tr("checking_version"));
        let hwnd = app.hwnd;
        let launch_failed = tr("launch_failed").to_string();
        let place_antumbra = tr("place_antumbra").to_string();
        thread::spawn(move || {
            let output = Command::new(antumbra_path()).arg("--version").output();
            let log = match output {
                Ok(out) => {
                    let mut text = String::new();
                    text.push_str(&String::from_utf8_lossy(&out.stdout));
                    text.push_str(&String::from_utf8_lossy(&out.stderr));
                    text.push_str(&format!("\r\nExit status: {}\r\n", out.status));
                    text
                }
                Err(e) => format!("{launch_failed}: {e}\r\n{place_antumbra}\r\n"),
            };
            let boxed = Box::new(log);
            PostMessageW(hwnd, WM_APP_LOG, 0, Box::into_raw(boxed) as Lparam);
        });
    }
}

unsafe fn trigger_exploit() {
    unsafe {
        let Some(app) = &APP else {
            return;
        };
        let mut args = global_args(app);
        args.push("crash".to_string());
        append_log(tr("triggering_exploit"));
        append_log(&format!("{} {}\r\n\r\n", antumbra_path().display(), quote_args(&args)));
        let hwnd = app.hwnd;
        let launch_failed = tr("launch_failed").to_string();
        let place_antumbra = tr("place_antumbra").to_string();
        thread::spawn(move || {
            let output = Command::new(antumbra_path()).args(&args).output();
            let log = match output {
                Ok(out) => {
                    let mut text = String::new();
                    text.push_str(&String::from_utf8_lossy(&out.stdout));
                    text.push_str(&String::from_utf8_lossy(&out.stderr));
                    text.push_str(&format!("\r\nExit status: {}\r\n", out.status));
                    text
                }
                Err(e) => format!("{launch_failed}: {e}\r\n{place_antumbra}\r\n"),
            };
            let boxed = Box::new(log);
            PostMessageW(hwnd, WM_APP_LOG, 0, Box::into_raw(boxed) as Lparam);
            PostMessageW(hwnd, WM_APP_DONE, 0, 0);
        });
    }
}

unsafe fn browse_into(target: Hwnd, title: &str) {
    unsafe {
        let mut file = vec![0u16; 32768];
        let filter = wide(&format!("{}\0*.*\0{}\0*.bin\0\0", tr("all_files"), tr("binary_files")));
        let title_w = wide(title);
        let mut ofn: OpenFileNameW = zeroed();
        ofn.lStructSize = size_of::<OpenFileNameW>() as Dword;
        ofn.hwndOwner = APP.as_ref().map(|a| a.hwnd).unwrap_or(0);
        ofn.lpstrFilter = filter.as_ptr();
        ofn.lpstrFile = file.as_mut_ptr();
        ofn.nMaxFile = file.len() as Dword;
        ofn.lpstrTitle = title_w.as_ptr();
        ofn.Flags = OFN_EXPLORER | OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST;
        if GetOpenFileNameW(&mut ofn) != 0 {
            let nul = file.iter().position(|&c| c == 0).unwrap_or(file.len());
            let selected = String::from_utf16_lossy(&file[..nul]);
            set_text(target, &selected);
        }
    }
}

unsafe fn message(text: &str) {
    unsafe {
        let caption = wide(tr("title"));
        let body = wide(text);
        let owner = APP.as_ref().map(|a| a.hwnd).unwrap_or(0);
        MessageBoxW(owner, body.as_ptr(), caption.as_ptr(), 0);
    }
}

unsafe extern "system" fn wnd_proc(hwnd: Hwnd, msg: Uint, wparam: Wparam, lparam: Lparam) -> Lresult {
    unsafe {
        match msg {
            WM_CREATE => {
                create_ui(hwnd);
                0
            }
            WM_COMMAND => {
                match loword(wparam) {
                    IDC_BROWSE_DA => if let Some(app) = &APP { browse_into(app.da, tr("select_da")); },
                    IDC_BROWSE_PL => if let Some(app) = &APP { browse_into(app.preloader, tr("select_preloader")); },
                    IDC_BROWSE_AUTH => if let Some(app) = &APP { browse_into(app.auth, tr("select_auth")); },
                    IDC_BROWSE_FILE => if let Some(app) = &APP { browse_into(app.file, tr("select_input")); },
                    IDC_BROWSE_OUTPUT => if let Some(app) = &APP { browse_into(app.output, tr("select_output")); },
                    IDC_BUILD => build_command_preview(),
                    IDC_RUN => run_command(),
                    IDC_VERSION => check_version(),
                    IDC_EXPLOIT => trigger_exploit(),
                    _ => {}
                }
                0
            }
            WM_APP_LOG => {
                let text = Box::from_raw(lparam as *mut String);
                append_log(&text);
                0
            }
            WM_APP_DONE => {
                if let Some(app) = &APP {
                    SendMessageW(app.run, WM_ENABLE, 1, 0);
                }
                append_log(tr("done"));
                0
            }
            WM_DESTROY => {
                if let Some(app) = &APP {
                    if app.font != 0 {
                        DeleteObject(app.font);
                    }
                }
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

unsafe fn create_ui(hwnd: Hwnd) {
    unsafe {
        UI_LOCALE = detect_locale();
        UI_FONT = create_ui_font(UI_LOCALE);

        add_label(hwnd, tr("global_files"), 16, 12, 160);
        add_label(hwnd, tr("da"), 16, 42, 135);
        let da = add_edit(hwnd, 165, 38, 595, ID_DA);
        let browse_da = create_control(hwnd, "BUTTON", tr("browse"), BS_PUSHBUTTON | WS_TABSTOP, 772, 37, 86, 24, IDC_BROWSE_DA);

        add_label(hwnd, tr("preloader"), 16, 72, 135);
        let preloader = add_edit(hwnd, 165, 68, 595, ID_PL);
        let browse_pl = create_control(hwnd, "BUTTON", tr("browse"), BS_PUSHBUTTON | WS_TABSTOP, 772, 67, 86, 24, IDC_BROWSE_PL);

        add_label(hwnd, tr("auth"), 16, 102, 135);
        let auth = add_edit(hwnd, 165, 98, 595, ID_AUTH);
        let browse_auth = create_control(hwnd, "BUTTON", tr("browse"), BS_PUSHBUTTON | WS_TABSTOP, 772, 97, 86, 24, IDC_BROWSE_AUTH);
        let _ = (browse_da, browse_pl, browse_auth);

        let verbose = create_control(hwnd, "BUTTON", tr("verbose_log"), BS_AUTOCHECKBOX | WS_TABSTOP, 880, 38, 170, 22, ID_VERBOSE);
        let usb_log = create_control(hwnd, "BUTTON", tr("usb_da_log"), BS_AUTOCHECKBOX | WS_TABSTOP, 880, 68, 170, 22, ID_USB_LOG);

        add_label(hwnd, tr("operation"), 16, 143, 135);
        let operation = create_control(
            hwnd,
            "COMBOBOX",
            "",
            WS_BORDER | CBS_DROPDOWNLIST | CBS_HASSTRINGS | WS_TABSTOP,
            165,
            138,
            440,
            240,
            IDC_OPERATION,
        );
        for key in OPERATION_KEYS {
            SendMessageW(operation, CB_ADDSTRING, 0, wide(tr(key)).as_ptr() as Lparam);
        }
        SendMessageW(operation, CB_SETCURSEL, 0, 0);

        add_label(hwnd, tr("partition"), 16, 184, 135);
        let partition = add_edit(hwnd, 165, 180, 225, ID_PARTITION);
        add_label(hwnd, tr("address_sector"), 410, 184, 135);
        let address = add_edit(hwnd, 552, 180, 135, ID_ADDRESS);
        add_label(hwnd, tr("length_sectors"), 706, 184, 135);
        let length = add_edit(hwnd, 848, 180, 135, ID_LENGTH);

        add_label(hwnd, tr("input_file_dir"), 16, 218, 145);
        let file = add_edit(hwnd, 165, 214, 595, ID_FILE);
        create_control(hwnd, "BUTTON", tr("browse"), BS_PUSHBUTTON | WS_TABSTOP, 772, 213, 86, 24, IDC_BROWSE_FILE);

        add_label(hwnd, tr("output_file_dir"), 16, 252, 145);
        let output = add_edit(hwnd, 165, 248, 595, ID_OUTPUT);
        create_control(hwnd, "BUTTON", tr("browse"), BS_PUSHBUTTON | WS_TABSTOP, 772, 247, 86, 24, IDC_BROWSE_OUTPUT);

        add_label(hwnd, tr("value_region"), 16, 286, 145);
        let value = add_edit(hwnd, 165, 282, 160, ID_VALUE);
        add_label(hwnd, tr("skip_csv"), 345, 286, 90);
        let skip = add_edit(hwnd, 440, 282, 240, ID_SKIP);
        let ignore_missing = create_control(hwnd, "BUTTON", tr("ignore_missing"), BS_AUTOCHECKBOX | WS_TABSTOP, 700, 282, 280, 22, ID_IGNORE_MISSING);

        add_label(hwnd, tr("extra_cli_args"), 16, 320, 145);
        let extra = add_edit(hwnd, 165, 316, 480, ID_EXTRA);
        create_control(hwnd, "BUTTON", tr("preview"), BS_PUSHBUTTON | WS_TABSTOP, 666, 315, 82, 26, IDC_BUILD);
        let run = create_control(hwnd, "BUTTON", tr("run"), BS_PUSHBUTTON | WS_TABSTOP, 756, 315, 82, 26, IDC_RUN);
        create_control(hwnd, "BUTTON", tr("version"), BS_PUSHBUTTON | WS_TABSTOP, 846, 315, 92, 26, IDC_VERSION);
        create_control(hwnd, "BUTTON", tr("exploit"), BS_PUSHBUTTON | WS_TABSTOP, 946, 315, 98, 26, IDC_EXPLOIT);

        add_label(hwnd, tr("log"), 16, 360, 80);
        let log = create_control(
            hwnd,
            "EDIT",
            "",
            WS_BORDER | WS_VSCROLL | ES_MULTILINE | ES_AUTOVSCROLL | ES_READONLY,
            16,
            386,
            1040,
            240,
            ID_LOG,
        );
        SendMessageW(log, EM_LIMITTEXT, 0, 0);

        SendMessageW(verbose, BM_SETCHECK, BST_CHECKED, 0);
        APP = Some(App {
            hwnd,
            font: UI_FONT,
            operation,
            da,
            preloader,
            auth,
            partition,
            file,
            output,
            address,
            length,
            value,
            skip,
            extra,
            verbose,
            usb_log,
            ignore_missing,
            log,
            run,
        });
        append_log(tr("ready"));
    }
}

fn main() {
    unsafe {
        UI_LOCALE = detect_locale();
        let class = wide("PenumbraFlashToolWindow");
        let hinstance = GetModuleHandleW(null());
        let app_icon = LoadIconW(hinstance, 1usize as *const u16);
        let wc = WndClassW {
            style: 0,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: app_icon,
            hCursor: LoadCursorW(0, 32512usize as *const u16),
            hbrBackground: COLOR_BTNFACE + 1,
            lpszMenuName: null(),
            lpszClassName: class.as_ptr(),
        };
        RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            0,
            class.as_ptr(),
            wide(tr("title")).as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            1090,
            700,
            0,
            0,
            hinstance,
            null_mut(),
        );
        if hwnd == 0 {
            return;
        }
        if app_icon != 0 {
            SendMessageW(hwnd, WM_SETICON, ICON_BIG, app_icon as Lparam);
            SendMessageW(hwnd, WM_SETICON, ICON_SMALL, app_icon as Lparam);
        }

        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);

        let mut msg: Msg = zeroed();
        while GetMessageW(&mut msg, 0, 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
