//! Win32 creation-time Job assignment smoke probe (Phase B S0, T43).
//!
//! This binary proves, on THIS host, that the design's process-containment
//! primitive works: a child created **inside** a fresh Job object with
//! `PROC_THREAD_ATTRIBUTE_JOB_LIST` and `CREATE_SUSPENDED`, identified while
//! suspended, then terminated through the Job, leaving an empty process list
//! and zero active accounting. There is deliberately **no spawn-then-assign
//! fallback** — the design forbids it (T41 §8 R4).
//!
//! It is a test-only smoke probe, not the production `OwnedProcess` (B2.1).
//! It makes no network, model or credential access.
//!
//! Exit codes: 0 = PASS. Non-zero identifies the failing step; every failure
//! prints the step and the exact Win32 error (HRESULT and the low 16 bits).

#![deny(unsafe_op_in_unsafe_fn)]

use std::ffi::c_void;
use std::mem::{offset_of, size_of};
use std::os::windows::ffi::OsStrExt;

use windows::core::{Error, PCWSTR, PWSTR};
use windows::Win32::Foundation::{CloseHandle, FILETIME, HANDLE};
use windows::Win32::System::JobObjects::{
    CreateJobObjectW, JobObjectBasicAccountingInformation, JobObjectBasicProcessIdList,
    JobObjectExtendedLimitInformation, QueryInformationJobObject, SetInformationJobObject,
    TerminateJobObject, JOBOBJECT_BASIC_ACCOUNTING_INFORMATION, JOBOBJECT_BASIC_PROCESS_ID_LIST,
    JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
};
use windows::Win32::System::Threading::{
    CreateProcessW, DeleteProcThreadAttributeList, GetProcessTimes,
    InitializeProcThreadAttributeList, ResumeThread, UpdateProcThreadAttribute, CREATE_SUSPENDED,
    EXTENDED_STARTUPINFO_PRESENT, LPPROC_THREAD_ATTRIBUTE_LIST, PROCESS_INFORMATION,
    PROC_THREAD_ATTRIBUTE_JOB_LIST, STARTUPINFOEXW,
};

/// Maximum child processes we care to enumerate for this probe.
const MAX_JOB_PROCESSES: usize = 64;

struct Fail {
    step: &'static str,
    exit_code: i32,
    detail: String,
}

impl Fail {
    fn assertion(step: &'static str, exit_code: i32, detail: impl Into<String>) -> Self {
        Self {
            step,
            exit_code,
            detail: detail.into(),
        }
    }

    fn win32(step: &'static str, exit_code: i32, error: Error) -> Self {
        let hresult = error.code().0 as u32;
        let win32 = hresult & 0xFFFF;
        Self {
            step,
            exit_code,
            detail: format!("hresult=0x{hresult:08X} win32_error={win32} message={error}"),
        }
    }
}

/// Closes the Job handle on every path. With `KILL_ON_JOB_CLOSE` set this is a
/// last-resort containment guarantee, not the probe's termination mechanism.
struct JobHandle(HANDLE);

impl Drop for JobHandle {
    fn drop(&mut self) {
        // SAFETY: the handle came from CreateJobObjectW and is closed once here.
        unsafe {
            let _ = CloseHandle(self.0);
        }
    }
}

fn job_process_ids(job: HANDLE) -> Result<Vec<u32>, Fail> {
    // JOBOBJECT_BASIC_PROCESS_ID_LIST is a variable-length structure: two u32
    // counters followed by `NumberOfProcessIdsInList` usize PIDs. The API
    // writes PIDs starting at `offset_of!(..., ProcessIdList)`, i.e. 8 bytes
    // in, not at `size_of::<...>()` (16, because of the `[usize; 1]` tail).
    // Allocate 8-byte-aligned storage for the header word plus the capacity.
    let mut buffer: Vec<u64> = vec![0; 1 + MAX_JOB_PROCESSES];
    let capacity_bytes = (buffer.len() * size_of::<u64>()) as u32;
    let header = buffer.as_mut_ptr() as *mut JOBOBJECT_BASIC_PROCESS_ID_LIST;
    // SAFETY: `buffer` is 8-byte aligned and `capacity_bytes` covers the two
    // u32 counters plus MAX_JOB_PROCESSES usize entries; `header` points into
    // that live allocation.
    let filled = unsafe {
        QueryInformationJobObject(
            Some(job),
            JobObjectBasicProcessIdList,
            header as *mut c_void,
            capacity_bytes,
            None,
        )
    };
    if let Err(error) = filled {
        return Err(Fail::win32("enumerate_suspended", 8, error));
    }
    // SAFETY: the API filled the header region of the live buffer above.
    let count = unsafe { (*header).NumberOfProcessIdsInList as usize };
    if count > MAX_JOB_PROCESSES {
        return Err(Fail::assertion(
            "enumerate_suspended",
            8,
            format!("reported {count} process ids, exceeding probe capacity {MAX_JOB_PROCESSES}"),
        ));
    }
    // SAFETY: PIDs begin at `offset_of!(ProcessIdList)` inside the same live
    // buffer and `count` entries were written there.
    let ids = unsafe {
        let base = (header as *const u8)
            .add(offset_of!(JOBOBJECT_BASIC_PROCESS_ID_LIST, ProcessIdList))
            as *const usize;
        std::slice::from_raw_parts(base, count)
            .iter()
            .map(|id| *id as u32)
            .collect()
    };
    Ok(ids)
}

fn accounting(job: HANDLE) -> Result<JOBOBJECT_BASIC_ACCOUNTING_INFORMATION, Fail> {
    let mut info = JOBOBJECT_BASIC_ACCOUNTING_INFORMATION::default();
    // SAFETY: the struct is the documented size for this information class.
    unsafe {
        QueryInformationJobObject(
            Some(job),
            JobObjectBasicAccountingInformation,
            &mut info as *mut _ as *mut c_void,
            size_of::<JOBOBJECT_BASIC_ACCOUNTING_INFORMATION>() as u32,
            None,
        )
    }
    .map_err(|error| Fail::win32("accounting", 11, error))?;
    Ok(info)
}

fn run() -> Result<(), Fail> {
    // Step 1: fresh unnamed Job. A null security descriptor gives a
    // non-inheritable handle.
    // SAFETY: standard job creation with a null name and attributes.
    let job = unsafe { CreateJobObjectW(None, PCWSTR::null()) }
        .map_err(|error| Fail::win32("create_job", 2, error))?;
    let job = JobHandle(job);
    println!("step=create_job ok");

    // Step 2: KILL_ON_JOB_CLOSE, then read the setting back.
    let mut limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
    limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
    // SAFETY: struct and size match JobObjectExtendedLimitInformation.
    unsafe {
        SetInformationJobObject(
            job.0,
            JobObjectExtendedLimitInformation,
            &limits as *const _ as *const c_void,
            size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
        )
    }
    .map_err(|error| Fail::win32("set_kill_on_job_close", 3, error))?;
    println!("step=set_kill_on_job_close ok");

    let mut readback = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
    // SAFETY: same contract as the set call.
    unsafe {
        QueryInformationJobObject(
            Some(job.0),
            JobObjectExtendedLimitInformation,
            &mut readback as *mut _ as *mut c_void,
            size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            None,
        )
    }
    .map_err(|error| Fail::win32("readback_kill_on_job_close", 4, error))?;
    let flags = readback.BasicLimitInformation.LimitFlags.0;
    if flags & JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE.0 != JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE.0 {
        return Err(Fail::assertion(
            "readback_kill_on_job_close",
            4,
            format!(
                "expected flag {} present, read back limit_flags=0x{flags:08X}",
                JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE.0
            ),
        ));
    }
    println!("step=readback_kill_on_job_close ok limit_flags=0x{flags:08X} kill_on_job_close=1");

    // Step 3: thread attribute list holding the Job handle.
    let mut attribute_size: usize = 0;
    // SAFETY: first call with a null list is the documented size query; it is
    // expected to fail with ERROR_INSUFFICIENT_BUFFER.
    let _ = unsafe { InitializeProcThreadAttributeList(None, 1, None, &mut attribute_size) };
    if attribute_size == 0 {
        return Err(Fail::assertion(
            "init_attribute_list",
            5,
            "size query returned zero",
        ));
    }
    // The attribute list stores pointers internally, so its storage must be
    // suitably aligned. Vec<u128> guarantees 16-byte alignment, matching the
    // Windows heap alignment the API's own examples rely on.
    let words = attribute_size.div_ceil(16);
    let mut attribute_storage: Vec<u128> = vec![0; words];
    let attribute_list =
        LPPROC_THREAD_ATTRIBUTE_LIST(attribute_storage.as_mut_ptr() as *mut c_void);
    // SAFETY: storage is at least attribute_size bytes and the list is
    // initialised exactly once with one attribute.
    unsafe {
        InitializeProcThreadAttributeList(Some(attribute_list), 1, None, &mut attribute_size)
    }
    .map_err(|error| Fail::win32("init_attribute_list", 5, error))?;
    // SAFETY: value points to a live HANDLE and its size is correct for the
    // JOB_LIST attribute.
    let update = unsafe {
        UpdateProcThreadAttribute(
            attribute_list,
            0,
            PROC_THREAD_ATTRIBUTE_JOB_LIST as usize,
            Some(&job.0 as *const HANDLE as *const c_void),
            size_of::<HANDLE>(),
            None,
            None,
        )
    };
    if let Err(error) = update {
        // SAFETY: the list was initialised above and is deleted once.
        unsafe { DeleteProcThreadAttributeList(attribute_list) };
        return Err(Fail::win32("update_job_list_attribute", 5, error));
    }
    println!(
        "step=attribute_list ok attribute=PROC_THREAD_ATTRIBUTE_JOB_LIST size={attribute_size}"
    );

    // Step 4: create the child suspended, assigned to the Job at creation.
    let mut command_line: Vec<u16> = std::ffi::OsStr::new("cmd.exe /c exit 0")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let mut startup = STARTUPINFOEXW::default();
    startup.StartupInfo.cb = size_of::<STARTUPINFOEXW>() as u32;
    startup.lpAttributeList = attribute_list;
    let mut process = PROCESS_INFORMATION::default();
    // SAFETY: startup is a live STARTUPINFOEXW with the attribute list attached,
    // the command line is a mutable NUL-terminated UTF-16 buffer, and the
    // process information struct is writable.
    let created = unsafe {
        CreateProcessW(
            PCWSTR::null(),
            Some(PWSTR(command_line.as_mut_ptr())),
            None,
            None,
            false,
            EXTENDED_STARTUPINFO_PRESENT | CREATE_SUSPENDED,
            None,
            PCWSTR::null(),
            &startup.StartupInfo as *const _,
            &mut process,
        )
    };
    // The attribute list may be deleted once the process exists.
    // SAFETY: initialised above; deleted exactly once.
    unsafe { DeleteProcThreadAttributeList(attribute_list) };
    created.map_err(|error| Fail::win32("create_process_suspended", 6, error))?;
    println!(
        "step=create_process_suspended ok root_pid={} thread_id={}",
        process.dwProcessId, process.dwThreadId
    );

    let process_handle = process.hProcess;
    let thread_handle = process.hThread;

    // Step 5: identity while suspended — creation FILETIME plus Job membership.
    let mut creation = FILETIME::default();
    let mut exit_time = FILETIME::default();
    let mut kernel_time = FILETIME::default();
    let mut user_time = FILETIME::default();
    // SAFETY: process_handle is live and suspended; four FILETIME out-pointers.
    unsafe {
        GetProcessTimes(
            process_handle,
            &mut creation,
            &mut exit_time,
            &mut kernel_time,
            &mut user_time,
        )
    }
    .map_err(|error| {
        // SAFETY: handles are live and closed once on this failure path.
        unsafe {
            let _ = CloseHandle(thread_handle);
            let _ = CloseHandle(process_handle);
        }
        Fail::win32("process_identity", 7, error)
    })?;
    let creation_value = ((creation.dwHighDateTime as u64) << 32) | creation.dwLowDateTime as u64;
    let suspended_ids = job_process_ids(job.0).inspect_err(|_| {
        // SAFETY: handles are live and closed once on this failure path.
        unsafe {
            let _ = CloseHandle(thread_handle);
            let _ = CloseHandle(process_handle);
        }
    })?;
    if suspended_ids != vec![process.dwProcessId] {
        // SAFETY: handles are live and closed once on this failure path.
        unsafe {
            let _ = CloseHandle(thread_handle);
            let _ = CloseHandle(process_handle);
        }
        return Err(Fail::assertion(
            "enumerate_suspended",
            8,
            format!(
                "expected exactly root pid {} in Job while suspended, found {suspended_ids:?}",
                process.dwProcessId
            ),
        ));
    }
    println!(
        "step=process_identity ok root_pid={} creation_filetime={}",
        process.dwProcessId, creation_value
    );
    println!("step=enumerate_suspended ok count=1 pids={suspended_ids:?}");

    // Step 6: resume once.
    // SAFETY: thread_handle is live and was created suspended.
    let previous_suspend_count = unsafe { ResumeThread(thread_handle) };
    if previous_suspend_count == u32::MAX {
        let error = Error::from_thread();
        // SAFETY: handles are live and closed once on this failure path.
        unsafe {
            let _ = CloseHandle(thread_handle);
            let _ = CloseHandle(process_handle);
        }
        return Err(Fail::win32("resume_thread", 9, error));
    }
    println!("step=resume_thread ok previous_suspend_count={previous_suspend_count}");

    // Step 7: terminate through the Job, not through the child handle.
    // SAFETY: job handle is live; the exit code is arbitrary and recorded.
    unsafe { TerminateJobObject(job.0, 1) }
        .map_err(|error| Fail::win32("terminate_job", 10, error))?;
    println!("step=terminate_job ok exit_code=1");

    // Step 8: verify empty process list and zero active accounting.
    let mut final_accounting = accounting(job.0)?;
    let mut waited_ms = 0u32;
    while (final_accounting.ActiveProcesses != 0 || !job_process_ids(job.0)?.is_empty())
        && waited_ms < 5_000
    {
        std::thread::sleep(std::time::Duration::from_millis(50));
        waited_ms += 50;
        final_accounting = accounting(job.0)?;
    }
    let remaining = job_process_ids(job.0)?;
    if !remaining.is_empty() || final_accounting.ActiveProcesses != 0 {
        return Err(Fail::assertion(
            "job_empty",
            11,
            format!(
                "job not empty after termination: pids={remaining:?} active_processes={} waited_ms={waited_ms}",
                final_accounting.ActiveProcesses
            ),
        ));
    }

    // SAFETY: all three handles are live and each is closed exactly once.
    unsafe {
        let _ = CloseHandle(thread_handle);
        let _ = CloseHandle(process_handle);
    }

    println!(
        "step=job_empty ok pids=[] active_processes=0 total_processes={} total_terminated={} waited_ms={waited_ms}",
        final_accounting.TotalProcesses, final_accounting.TotalTerminatedProcesses
    );
    println!("PASS job_smoke");
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => std::process::exit(0),
        Err(fail) => {
            eprintln!("FAIL step={} detail={}", fail.step, fail.detail);
            std::process::exit(fail.exit_code);
        }
    }
}
