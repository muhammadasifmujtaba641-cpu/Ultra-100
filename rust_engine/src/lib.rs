// Ultra Guardian - Rust 100% Power Engine
// Final Clean Build

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

fn contains_any(text: &str, parts: &[&str]) -> bool {
    parts.iter().any(|part| text.contains(part))
}

#[no_mangle]
pub extern "C" fn is_safe_to_delete(path: *const c_char) -> bool {
    if path.is_null() {
        return false;
    }
    
    // C string ko Rust string mein convert karna aur normalize karna
    let c_str = unsafe { CStr::from_ptr(path) };
    let p = c_str.to_string_lossy().to_lowercase().replace('/', "\\");

    // 1. Blocked - Kabhi delete nahi karna (False)
    let blocked = [
        "documents", "downloads", "desktop", "pictures", 
        "videos", "recycle bin", "$recycle.bin", "d:\\", "e:\\"
    ];
    if contains_any(&p, &blocked) {
        return false;
    }

    // 2. Allowed - 100% safe delete (True)
    let allowed = [
        "c:\\windows\\temp", "c:\\windows\\prefetch", 
        "thumbnail cache", "thumbcache", "hiberfil.sys"
    ];
    if contains_any(&p, &allowed) {
        return true;
    }

    // 3. AppData leftover of uninstalled apps (True)
    if p.contains("appdata") && 
       (p.contains("leftover") || p.contains("uninstalled") || p.contains("orphan")) {
        return true;
    }

    // Baqi sab unsafe hain
    false
}

#[no_mangle]
pub extern "C" fn push_to_blackhole(ip: *const c_char) -> bool {
    if ip.is_null() {
        return false;
    }
    
    let c_str = unsafe { CStr::from_ptr(ip) };
    let ip_str = c_str.to_string_lossy();
    
    // Exact requested output
    println!("Pushed {} to 127.0.0.1 - Local Safe", ip_str);
    true
}

// ==========================================
// ONECOMPILER TEST MAIN
// OneCompiler par run karne ke liye ye zaroori hai.
// ==========================================
fn main() {
    println!("=== Ultra Guardian Engine Test ===\n");
    
    // Test paths
    let test_paths = [
        "C:\\Windows\\Temp\\junk.tmp",           // Safe (Allowed)
        "C:\\Users\\Ali\\Documents\\safe.txt",   // Unsafe (Blocked)
        "C:\\Windows\\Prefetch\\app.pf",         // Safe (Allowed)
        "D:\\Games\\game.exe",                   // Unsafe (D:\ drive blocked)
        "C:\\Users\\Ali\\AppData\\Local\\uninstalled_app_data", // Safe (Leftover)
        "E:\\Movies\\video.mp4"                  // Unsafe (E:\ drive blocked)
    ];
    
    for p in test_paths.iter() {
        let c_string = CString::new(*p).unwrap();
        let is_safe = is_safe_to_delete(c_string.as_ptr());
        println!("Path: {:<55} | is_safe_to_delete: {}", p, is_safe);
    }
    
    println!("\n--- Testing Push to Blackhole ---");
    let ip = CString::new("203.0.113.5").unwrap();
    push_to_blackhole(ip.as_ptr());
    
    println!("\n=== Engine Test Complete ===");
}
