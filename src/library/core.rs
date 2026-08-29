use std::env;
use std::process::{Command, exit};
use mlua::{Lua, Result, Table};

#[cfg(target_os = "windows")]
fn get_parent_pid() -> Option<u32> {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32Next, Process32First, PROCESSENTRY32, TH32CS_SNAPPROCESS,
    };
    use windows_sys::Win32::System::Threading::GetCurrentProcessId;

    let current_pid = unsafe { GetCurrentProcessId() };
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    
    if snapshot == 0 || snapshot == (-1isize as isize) {
        return None;
    }

    let mut entry: PROCESSENTRY32 = unsafe { std::mem::zeroed() };
    entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

    if unsafe { Process32First(snapshot, &mut entry) } != 0 {
        loop {
            if entry.th32ProcessID == current_pid {
                unsafe { CloseHandle(snapshot); }
                return Some(entry.th32ParentProcessID);
            }
            if unsafe { Process32Next(snapshot, &mut entry) } == 0 {
                break;
            }
        }
    }
    
    unsafe { CloseHandle(snapshot); }
    None
}

pub fn register(lua: &Lua, fred_table: &Table) -> Result<()> {
    let version_fn = lua.create_function(|_, ()| {
        Ok(crate::FRED_VERSION)
    })?;
    fred_table.set("version", version_fn)?;

    let platform_fn = lua.create_function(|_, ()| {
        Ok(env::consts::OS)
    })?;
    fred_table.set("platform", platform_fn)?;

    let exit_fn = lua.create_function(|_, ()| {
        #[cfg(target_os = "windows")]
        {
            let _ = Command::new("cmd")
                .args(["/C", "taskkill /IM WindowsTerminal.exe /F 2>nul || taskkill /IM conhost.exe /F 2>nul"])
                .output();
        }
        #[cfg(not(target_os = "windows"))]
        {
            let ppid = unsafe { libc::getppid() };
            let _ = Command::new("kill")
                .args(["-9", &ppid.to_string()])
                .output();
        }
        exit(0);
        #[allow(unreachable_code)]
        Ok(())
    })?;
    fred_table.set("exit", exit_fn)?;

    Ok(())
}