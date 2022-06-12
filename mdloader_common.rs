#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
#[macro_use]
extern crate c2rust_bitfields;
extern crate lazy_static_include;

use lazy_static_include::lazy_static_include_bytes;
lazy_static_include_bytes! {
    /// doc
    applet => "applet-mdflash.bin",
}

mod mdloader_unix;
mod mdloader_parser;

mod atmel;
use atmel::applet::*;
use atmel::status::*;

pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;

pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn read_data(addr: libc::c_int, readsize: libc::c_int) -> libc::c_int;
    fn recv_file(addr: libc::c_int, bytes: libc::c_int) -> *mut libc::c_char;
    fn write_data(addr: libc::c_int, writesize: libc::c_int, data: libc::c_int) -> libc::c_int;
    fn list_devices(first: *mut libc::c_char);
    fn config_port() -> libc::c_int;
    fn goto_address(addr: libc::c_int) -> libc::c_int;
    fn close_port(silent: libc::c_char) -> libc::c_int;
    fn open_port(portname: *mut libc::c_char, silent: libc::c_char) -> libc::c_int;
    fn jump_application() -> libc::c_int;
    fn set_normal_mode() -> libc::c_int;
    fn print_bootloader_version() -> libc::c_int;
    fn send_file(addr: libc::c_int, bytes: libc::c_int, data: *mut libc::c_char) -> libc::c_int;
    fn print_com_example();
    fn free_data(data: *mut data_t);
    fn load_file(fname: *mut libc::c_char) -> *mut data_t;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type size_t = libc::c_ulong;
pub type int8_t = __int8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct appinfo_s {
    pub magic: uint32_t,
    pub load_addr: uint32_t,
    pub mail_addr: uint32_t,
    pub unused: [uint32_t; 5],
}
pub type appinfo_t = appinfo_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mcu_s {
    pub name: [libc::c_char; 20],
    pub cidr_addr: libc::c_int,
    pub cidr: libc::c_int,
    pub flash_size: libc::c_int,
    pub ram_size: libc::c_int,
    pub flash_addr: libc::c_int,
    pub ram_addr: libc::c_int,
}
pub type mcu_t = mcu_s;
pub type command = libc::c_uint;
pub const CMD_ABORT: command = 7;
pub const CMD_TEST: command = 6;
pub const CMD_UPLOAD: command = 5;
pub const CMD_DOWNLOAD: command = 4;
pub const CMD_LISTDEV: command = 3;
pub const CMD_VERSION: command = 2;
pub const CMD_HELP: command = 1;
pub const CMD_NONE: command = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union NVMCTRL_CTRLB_Type {
    pub bit: C2RustUnnamed_16,
    pub reg: uint16_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    #[bitfield(name = "CMD", ty = "uint16_t", bits = "0..=6")]
    #[bitfield(name = "c2rust_unnamed", ty = "uint16_t", bits = "7..=7")]
    #[bitfield(name = "CMDEX", ty = "uint16_t", bits = "8..=15")]
    pub CMD_c2rust_unnamed_CMDEX: [u8; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union NVMCTRL_CTRLA_Type {
    pub bit: C2RustUnnamed_17,
    pub reg: uint16_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    #[bitfield(name = "c2rust_unnamed", ty = "uint16_t", bits = "0..=1")]
    #[bitfield(name = "AUTOWS", ty = "uint16_t", bits = "2..=2")]
    #[bitfield(name = "SUSPEN", ty = "uint16_t", bits = "3..=3")]
    #[bitfield(name = "WMODE", ty = "uint16_t", bits = "4..=5")]
    #[bitfield(name = "PRM", ty = "uint16_t", bits = "6..=7")]
    #[bitfield(name = "RWS", ty = "uint16_t", bits = "8..=11")]
    #[bitfield(name = "AHBNS0", ty = "uint16_t", bits = "12..=12")]
    #[bitfield(name = "AHBNS1", ty = "uint16_t", bits = "13..=13")]
    #[bitfield(name = "CACHEDIS0", ty = "uint16_t", bits = "14..=14")]
    #[bitfield(name = "CACHEDIS1", ty = "uint16_t", bits = "15..=15")]
    pub c2rust_unnamed_AUTOWS_SUSPEN_WMODE_PRM_RWS_AHBNS0_AHBNS1_CACHEDIS0_CACHEDIS1: [u8; 2],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    #[bitfield(name = "SBLK", ty = "uint32_t", bits = "0..=3")]
    #[bitfield(name = "PSZ", ty = "uint32_t", bits = "4..=6")]
    #[bitfield(name = "RAM_ECCDIS", ty = "uint32_t", bits = "7..=7")]
    #[bitfield(name = "c2rust_unnamed", ty = "uint32_t", bits = "8..=15")]
    #[bitfield(name = "WDT_ENABLE", ty = "uint32_t", bits = "16..=16")]
    #[bitfield(name = "WDT_ALWAYS_ON", ty = "uint32_t", bits = "17..=17")]
    #[bitfield(name = "WDT_PERIOD", ty = "uint32_t", bits = "18..=21")]
    #[bitfield(name = "WDT_WINDOW", ty = "uint32_t", bits = "22..=25")]
    #[bitfield(name = "WDT_EWOFFSET", ty = "uint32_t", bits = "26..=29")]
    #[bitfield(name = "WDT_WEN", ty = "uint32_t", bits = "30..=30")]
    #[bitfield(name = "c2rust_unnamed_0", ty = "uint32_t", bits = "31..=31")]
    pub SBLK_PSZ_RAM_ECCDIS_c2rust_unnamed_WDT_ENABLE_WDT_ALWAYS_ON_WDT_PERIOD_WDT_WINDOW_WDT_EWOFFSET_WDT_WEN_c2rust_unnamed_0:
        [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union NVMCTRL_USER_ROW_MAPPING1_Type {
    pub bit: C2RustUnnamed_18,
    pub reg: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_s {
    pub addr: uint32_t,
    pub size: uint32_t,
    pub data: *mut libc::c_char,
}
pub type data_t = data_s;
#[no_mangle]
pub static mut verbose: libc::c_char = 0;
#[no_mangle]
pub static mut testmode: libc::c_char = 0;
#[no_mangle]
pub static mut first_device: libc::c_char = 0;
#[no_mangle]
pub static mut restart_after_program: libc::c_int = 0;
#[no_mangle]
pub static mut ignore_smarteeprom_config: libc::c_int = 0;
#[no_mangle]
pub static mut hex_cols: libc::c_int = 0;
#[no_mangle]
pub static mut hex_colw: libc::c_int = 0;
#[no_mangle]
pub static mut initparams: mailbox_t = mailbox_t {
    command: 0,
    status: 0,
    argument: C2RustUnnamed {
        inputInit: C2RustUnnamed_15 {
            comType: 0,
            traceLevel: 0,
            bank: 0,
        },
    },
};
#[no_mangle]
pub static mut appletinfo: mailbox_t = mailbox_t {
    command: 0,
    status: 0,
    argument: C2RustUnnamed {
        inputInit: C2RustUnnamed_15 {
            comType: 0,
            traceLevel: 0,
            bank: 0,
        },
    },
};
#[no_mangle]
pub static mut appinfo: appinfo_t = appinfo_t {
    magic: 0,
    load_addr: 0,
    mail_addr: 0,
    unused: [0; 5],
};
#[no_mangle]
pub static mut mcus: [mcu_t; 25] = unsafe {
    [
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAME54P19A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x61840001 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAME54P20A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x61840000 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAME54N19A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x61840003 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAME54N20A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x61840002 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAME53N20A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x61830002 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAME53N19A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x61830003 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAME53J18A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x61830006 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAME53J19A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x61830005 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAME53J20A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x61830004 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAME51N19A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x61810001 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAME51N20A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x61810000 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAME51J18A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x61810003 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAME51J19A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x61810002 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAME51J20A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x61810004 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAMD51P20A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x60060000 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAMD51P19A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x60060001 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAMD51N19A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x60060003 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAMD51N20A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x60060002 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAMD51J18A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x60060006 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAMD51J19A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x60060005 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAMD51J20A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x60060004 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAMD51G18A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x60060008 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAMD51G19A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x60060007 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAME51G18A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x61810306 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = mcu_s {
                name: *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
                    b"SAME51G19A\0\0\0\0\0\0\0\0\0\0",
                ),
                cidr_addr: 0x41002018 as libc::c_int,
                cidr: 0x61810305 as libc::c_int,
                flash_size: 0x40000 as libc::c_int,
                ram_size: 0x20000 as libc::c_int,
                flash_addr: 0 as libc::c_int,
                ram_addr: 0x20000000 as libc::c_int,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut mcu: *mut mcu_t = 0 as *const mcu_t as *mut mcu_t;
#[no_mangle]
pub static mut bootloader_length: uint32_t = 0;
#[no_mangle]
pub unsafe extern "C" fn check_bootloader_write_attempt(mut addr: libc::c_int) -> libc::c_int {
    if (addr as libc::c_uint) < ((*mcu).flash_addr as libc::c_uint).wrapping_add(bootloader_length)
    {
        printf(
            b"Attempt to write to bootloader section denied!\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut read_error: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn print_bootloader_serial() -> libc::c_int {
    let mut seroffset: libc::c_int =
        read_word(bootloader_length.wrapping_sub(4 as libc::c_int as libc::c_uint) as libc::c_int);
    if read_error != 0 {
        printf(b"Serial Number: Read error!\n\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    if verbose as libc::c_int > 0 as libc::c_int {
        printf(
            b"Serial Number offset: 0x%08x\n\0" as *const u8 as *const libc::c_char,
            seroffset,
        );
    }
    if seroffset as libc::c_uint
        >= ((*mcu).flash_addr as libc::c_uint).wrapping_add(bootloader_length)
    {
        printf(b"Serial Number: Not programmed!\n\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    let mut readbuf: *mut libc::c_char = recv_file(seroffset, 20 as libc::c_int * 2 as libc::c_int);
    if readbuf.is_null() {
        printf(b"Serial Number: Error retrieving!\n\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    printf(b"Serial Number: \0" as *const u8 as *const libc::c_char);
    let mut ind: libc::c_int = 0;
    ind = 0 as libc::c_int;
    while ind < 20 as libc::c_int * 2 as libc::c_int {
        printf(
            b"%c\0" as *const u8 as *const libc::c_char,
            *readbuf.offset(ind as isize) as libc::c_int,
        );
        ind += 2 as libc::c_int;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    free(readbuf as *mut libc::c_void);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn send_mail(mut mail: *mut mailbox_t) -> libc::c_int {
    let mut byte: libc::c_int = 0;
    let mut pmail: *mut libc::c_int = mail as *mut libc::c_int;
    if verbose != 0 {
        printf(b"Sending mail {\n\0" as *const u8 as *const libc::c_char);
    }
    byte = 0 as libc::c_int;
    while (byte as libc::c_ulong)
        < (::std::mem::size_of::<mailbox_t>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        if verbose != 0 {
            printf(b"  \0" as *const u8 as *const libc::c_char);
        }
        write_word(
            (appinfo.mail_addr as libc::c_ulong).wrapping_add(
                (byte as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as libc::c_int,
            *pmail,
        );
        pmail = pmail.offset(1);
        byte += 1;
    }
    if verbose != 0 {
        printf(b"}\n\0" as *const u8 as *const libc::c_char);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn read_mail(mut mail: *mut mailbox_t) -> libc::c_int {
    let mut byte: libc::c_int = 0;
    let mut pmail: *mut libc::c_int = mail as *mut libc::c_int;
    if verbose != 0 {
        printf(b"Retrieving mail {\n\0" as *const u8 as *const libc::c_char);
    }
    byte = 0 as libc::c_int;
    while (byte as libc::c_ulong)
        < (::std::mem::size_of::<mailbox_t>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        if verbose != 0 {
            printf(b"  \0" as *const u8 as *const libc::c_char);
        }
        *pmail = read_word(
            (appinfo.mail_addr as libc::c_ulong).wrapping_add(
                (byte as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as libc::c_int,
        );
        pmail = pmail.offset(1);
        byte += 1;
    }
    if verbose != 0 {
        printf(b"}\n\0" as *const u8 as *const libc::c_char);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn print_mail(mut mail: *mut mailbox_t) -> libc::c_int {
    let mut arg: libc::c_int = 0;
    let mut pmail: *mut libc::c_int = mail as *mut libc::c_int;
    printf(b"Mailbox contents:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Command: %i\n\0" as *const u8 as *const libc::c_char,
        (*mail).command,
    );
    printf(
        b"Status: %i\n\0" as *const u8 as *const libc::c_char,
        (*mail).status,
    );
    pmail = pmail.offset(2 as libc::c_int as isize);
    arg = 0 as libc::c_int;
    while (arg as libc::c_ulong)
        < (::std::mem::size_of::<mailbox_t>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
    {
        printf(
            b"Arg %i: %08X\n\0" as *const u8 as *const libc::c_char,
            arg,
            *pmail,
        );
        pmail = pmail.offset(1);
        arg += 1;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn run_applet(mut mail: *mut mailbox_t) -> libc::c_int {
    let mut retries: libc::c_int = 0;
    let mut command: libc::c_int = 0;
    command = (*mail).command as libc::c_int;
    if command == 0x1 as libc::c_int {
        retries = 25 as libc::c_int;
    } else {
        retries = 10 as libc::c_int;
    }
    goto_address(appinfo.load_addr as libc::c_int);
    if verbose != 0 {
        printf(
            b"RUN: Command out: %08x\n\0" as *const u8 as *const libc::c_char,
            command,
        );
    }
    loop {
        usleep((10 as libc::c_int * 1000 as libc::c_int) as __useconds_t);
        if verbose != 0 {
            printf(b"RUN: Waiting on applet return\n\0" as *const u8 as *const libc::c_char);
        }
        (*mail).command = read_word(appinfo.mail_addr as libc::c_int) as uint32_t;
        if !((*mail).command != !command as libc::c_uint && {
            let fresh0 = retries;
            retries = retries - 1;
            fresh0 != 0
        }) {
            break;
        }
    }
    if verbose != 0 {
        print_mail(mail);
    }
    if retries == -(1 as libc::c_int) {
        if verbose != 0 {
            printf(b"RUN: Error running applet\n\0" as *const u8 as *const libc::c_char);
        }
        return 0 as libc::c_int;
    }
    read_mail(mail);
    if (*mail).status == STATUS_OK as libc::c_int as libc::c_uint {
        if verbose != 0 {
            printf(b"RUN: Applet return OK!\n\0" as *const u8 as *const libc::c_char);
        }
        return 1 as libc::c_int;
    } else {
        printf(b"RUN: Applet return ERROR!\n\0" as *const u8 as *const libc::c_char);
        if verbose != 0 {
            print_mail(mail);
        }
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn print_hex_listing(
    mut data: *mut libc::c_char,
    mut binfilesize: libc::c_int,
    mut markbyte: libc::c_int,
    mut base_addr: libc::c_int,
) {
    let mut pbinfile: *mut libc::c_uchar = data as *mut libc::c_uchar;
    let mut chrnum: libc::c_int = 0;
    let mut binfileend: libc::c_int = binfilesize;
    let mut addr: libc::c_int = base_addr;
    let mut pf: libc::c_int = (markbyte == 0 as libc::c_int) as libc::c_int;
    let mut ascii: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut pascii: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut cols: libc::c_uint = hex_cols as libc::c_uint;
    let mut colw: libc::c_uint = hex_colw as libc::c_uint;
    if cols < 1 as libc::c_int as libc::c_uint {
        printf(b"Error: Hex listing column count invalid!\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    if colw < 1 as libc::c_int as libc::c_uint {
        printf(b"Error: Hex listing column width invalid!\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    ascii = calloc(
        (cols.wrapping_mul(colw) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
    ) as *mut libc::c_uchar;
    if ascii.is_null() {
        printf(
            b"Error: Could not allocate memory for data listing!\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    pascii = ascii;
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    chrnum = 0 as libc::c_int;
    while chrnum < binfileend {
        if chrnum != 0 {
            if (chrnum as libc::c_uint).wrapping_rem(cols.wrapping_mul(colw))
                == 0 as libc::c_int as libc::c_uint
            {
                *pascii = 0 as libc::c_int as libc::c_uchar;
                printf(b"|%s|\0" as *const u8 as *const libc::c_char, ascii);
                if chrnum > markbyte && pf == 0 {
                    printf(
                        b" @%i\0" as *const u8 as *const libc::c_char,
                        (markbyte as libc::c_uint)
                            .wrapping_rem(cols.wrapping_mul(colw))
                            .wrapping_add(1 as libc::c_int as libc::c_uint),
                    );
                    pf = 1 as libc::c_int;
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
                addr = (addr as libc::c_uint).wrapping_add(cols.wrapping_mul(colw)) as libc::c_int
                    as libc::c_int;
                printf(b"%08X | \0" as *const u8 as *const libc::c_char, addr);
                pascii = ascii;
            } else if (chrnum as libc::c_uint).wrapping_rem(colw)
                == 0 as libc::c_int as libc::c_uint
            {
                printf(b"%c\0" as *const u8 as *const libc::c_char, ' ' as i32);
            }
        } else {
            printf(b"%08X | \0" as *const u8 as *const libc::c_char, addr);
        }
        if *pbinfile as libc::c_int >= 0x20 as libc::c_int
            && *pbinfile as libc::c_int <= 0x7e as libc::c_int
        {
            *pascii = *pbinfile;
        } else {
            *pascii = ' ' as i32 as libc::c_uchar;
        }
        pascii = pascii.offset(1);
        if (*pbinfile as libc::c_int) < 0x10 as libc::c_int {
            printf(b"0\0" as *const u8 as *const libc::c_char);
        }
        printf(
            b"%X%c\0" as *const u8 as *const libc::c_char,
            *pbinfile as libc::c_int,
            ' ' as i32,
        );
        chrnum += 1;
        pbinfile = pbinfile.offset(1);
    }
    while (chrnum as libc::c_uint).wrapping_rem(cols.wrapping_mul(colw))
        != 0 as libc::c_int as libc::c_uint
    {
        if (chrnum as libc::c_uint).wrapping_rem(colw) == 0 as libc::c_int as libc::c_uint {
            printf(b"%c\0" as *const u8 as *const libc::c_char, ' ' as i32);
        }
        *pascii = ' ' as i32 as libc::c_uchar;
        pascii = pascii.offset(1);
        printf(b"  %c\0" as *const u8 as *const libc::c_char, ' ' as i32);
        chrnum += 1;
    }
    *pascii = 0 as libc::c_int as libc::c_uchar;
    printf(b"|%s|\0" as *const u8 as *const libc::c_char, ascii);
    if chrnum > markbyte && pf == 0 {
        printf(
            b" @%i\0" as *const u8 as *const libc::c_char,
            (markbyte as libc::c_uint)
                .wrapping_rem(colw.wrapping_mul(cols))
                .wrapping_add(1 as libc::c_int as libc::c_uint),
        );
    }
    printf(b"\n\n\0" as *const u8 as *const libc::c_char);
    free(ascii as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn test_port(
    mut portname: *mut libc::c_char,
    mut silent: libc::c_char,
) -> libc::c_int {
    if open_port(portname, silent) == 0 {
        if silent == 0 {
            printf(
                b"Error: Could not open port! (Correct port?)\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    if config_port() == 0 {
        if silent == 0 {
            printf(
                b"Error: Could not configure port! (Correct port?)\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        close_port(silent);
        return 0 as libc::c_int;
    }
    if set_normal_mode() == 0 {
        if silent == 0 {
            printf(
                b"Error: Could not communicate with device! (Correct port?)\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        close_port(silent);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn test_mcu(mut silent: libc::c_char) -> libc::c_int {
    let mut mcu_index: int8_t = -(1 as libc::c_int) as int8_t;
    let mut mcu_max: int8_t = (::std::mem::size_of::<[mcu_t; 25]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<mcu_t>() as libc::c_ulong)
        as int8_t;
    let mut deviceid: libc::c_int = 0;
    mcu_index = 0 as libc::c_int as int8_t;
    while (mcu_index as libc::c_int) < mcu_max as libc::c_int {
        mcu = &mut *mcus.as_mut_ptr().offset(mcu_index as isize) as *mut mcu_t;
        deviceid = read_word((*mcu).cidr_addr);
        if read_error != 0 {
            if silent == 0 && verbose as libc::c_int != 0 {
                printf(
                    b"Notice: Could not read device ID at %08X!\n\0" as *const u8
                        as *const libc::c_char,
                    (*mcu).cidr_addr,
                );
            }
        } else if deviceid as libc::c_uint & 0xfffff0ff as libc::c_uint
            == (*mcu).cidr as libc::c_uint
        {
            if silent == 0 && verbose as libc::c_int != 0 {
                printf(
                    b"Found supported device ID: %08X\n\0" as *const u8 as *const libc::c_char,
                    deviceid,
                );
            }
            break;
        }
        mcu_index += 1;
    }
    if mcu_index as libc::c_int == mcu_max as libc::c_int {
        if silent == 0 {
            printf(
                b"Error: Could not find matching device ID!\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sleep_between_writes() {
    printf(b".\0" as *const u8 as *const libc::c_char);
    fflush(stdout);
    usleep((200 as libc::c_int * 1000 as libc::c_int) as __useconds_t);
}
#[no_mangle]
pub unsafe extern "C" fn write_user_row(mut data: *mut uint32_t) -> uint8_t {
    let mut ctrla: NVMCTRL_CTRLA_Type = NVMCTRL_CTRLA_Type {
        bit: C2RustUnnamed_17 {
            c2rust_unnamed_AUTOWS_SUSPEN_WMODE_PRM_RWS_AHBNS0_AHBNS1_CACHEDIS0_CACHEDIS1: [0; 2],
        },
    };
    ctrla.reg = read_half_word(0x41004000 as libc::c_int) as uint16_t;
    if verbose != 0 {
        printf(
            b"NVMCTRL.CTRLA: 0x%04x\n\tAUTOWS: 0x%01x\n\tSUSPEN: 0x%01x\n\tWMODE: 0x%02x\n\tPRM: 0x%02x\n\tRWS: 0x%04x\n\tAHBNS0: 0x%01x\n\tAHBNS1: 0x%01x\n\tCACHEDIS0: 0x%01x\n\tCACHEDIS1: 0x%01x\n\0"
                as *const u8 as *const libc::c_char,
            ctrla.reg as libc::c_int,
            (ctrla.bit).AUTOWS() as libc::c_int,
            (ctrla.bit).SUSPEN() as libc::c_int,
            (ctrla.bit).WMODE() as libc::c_int,
            (ctrla.bit).PRM() as libc::c_int,
            (ctrla.bit).RWS() as libc::c_int,
            (ctrla.bit).AHBNS0() as libc::c_int,
            (ctrla.bit).AHBNS1() as libc::c_int,
            (ctrla.bit).CACHEDIS0() as libc::c_int,
            (ctrla.bit).CACHEDIS1() as libc::c_int,
        );
    }
    printf(b"SmartEEPROM: Configuring...\0" as *const u8 as *const libc::c_char);
    (ctrla.bit).set_WMODE(0 as libc::c_int as uint16_t);
    if write_half_word(0x41004000 as libc::c_int, ctrla.reg as libc::c_int) == 0 {
        printf(
            b"Error: setting NVMCTRL.CTRLA.WMODE to Manual.\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int as uint8_t;
    }
    sleep_between_writes();
    if write_word(
        0x41004000 as libc::c_int + 0x14 as libc::c_int,
        0x804000 as libc::c_int,
    ) == 0
    {
        printf(
            b"Error: setting NVMCTRL_ADDR to NVMCTRL_USER (1).\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int as uint8_t;
    }
    sleep_between_writes();
    let mut ctrlb: NVMCTRL_CTRLB_Type = NVMCTRL_CTRLB_Type {
        bit: C2RustUnnamed_16 {
            CMD_c2rust_unnamed_CMDEX: [0; 2],
        },
    };
    ctrlb.reg = 0 as libc::c_int as uint16_t;
    (ctrlb.bit).set_CMD(0 as libc::c_int as uint16_t);
    (ctrlb.bit).set_CMDEX(0xa5 as libc::c_int as uint16_t);
    if write_half_word(
        0x41004000 as libc::c_int + 4 as libc::c_int,
        ctrlb.reg as libc::c_int,
    ) == 0
    {
        printf(
            b"Error: setting NVMCTRL_CTRLB to 0x%04x (Erase page).\n\0" as *const u8
                as *const libc::c_char,
            ctrlb.reg as libc::c_int,
        );
        return 0 as libc::c_int as uint8_t;
    }
    sleep_between_writes();
    ctrlb.reg = 0 as libc::c_int as uint16_t;
    (ctrlb.bit).set_CMD(0x15 as libc::c_int as uint16_t);
    (ctrlb.bit).set_CMDEX(0xa5 as libc::c_int as uint16_t);
    if write_half_word(
        0x41004000 as libc::c_int + 4 as libc::c_int,
        ctrlb.reg as libc::c_int,
    ) == 0
    {
        printf(
            b"Error: setting NVMCTRL_CTRLB to 0x%04x (Page buffer clear).\n\0" as *const u8
                as *const libc::c_char,
            ctrlb.reg as libc::c_int,
        );
        return 0 as libc::c_int as uint8_t;
    }
    sleep_between_writes();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if write_word(
            0x804000 as libc::c_int + i * 4 as libc::c_int,
            *data.offset(i as isize) as libc::c_int,
        ) == 0
        {
            printf(
                b"Error: Unable to write NVMCTRL_USER page %i.\n\0" as *const u8
                    as *const libc::c_char,
                i,
            );
            return 0 as libc::c_int as uint8_t;
        }
        sleep_between_writes();
        i += 1;
    }
    if write_word(
        0x41004000 as libc::c_int + 0x14 as libc::c_int,
        0x804000 as libc::c_int,
    ) == 0
    {
        printf(
            b"Error: setting NVMCTRL_ADDR to NVMCTRL_USER (2).\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int as uint8_t;
    }
    sleep_between_writes();
    ctrlb.reg = 0 as libc::c_int as uint16_t;
    (ctrlb.bit).set_CMD(0x4 as libc::c_int as uint16_t);
    (ctrlb.bit).set_CMDEX(0xa5 as libc::c_int as uint16_t);
    if write_half_word(
        0x41004000 as libc::c_int + 4 as libc::c_int,
        ctrlb.reg as libc::c_int,
    ) == 0
    {
        printf(
            b"Error: setting NVMCTRL_CTRLB to 0x%04x (Write Quad Word).\n\0" as *const u8
                as *const libc::c_char,
            ctrlb.reg as libc::c_int,
        );
        return 0 as libc::c_int as uint8_t;
    }
    sleep_between_writes();
    printf(b" Success!\n\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn configure_smarteeprom() -> uint8_t {
    let mut user_row: [uint32_t; 4] = [0; 4];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        user_row[i as usize] =
            read_word(0x804000 as libc::c_int + i * 4 as libc::c_int) as uint32_t;
        i += 1;
    }
    let mut puser_row1: *mut NVMCTRL_USER_ROW_MAPPING1_Type =
        &mut *user_row.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut uint32_t
            as *mut NVMCTRL_USER_ROW_MAPPING1_Type;
    if verbose != 0 {
        printf(
            b"SmartEEPROM: config - SBLK: 0x%04x - PSZ: 0x%03x.\n\0" as *const u8
                as *const libc::c_char,
            ((*puser_row1).bit).SBLK() as libc::c_int,
            ((*puser_row1).bit).PSZ() as libc::c_int,
        );
    }
    if ((*puser_row1).bit).SBLK() as libc::c_int == 1 as libc::c_int
        && ((*puser_row1).bit).PSZ() as libc::c_int == 3 as libc::c_int
    {
        if verbose != 0 {
            printf(b"SmartEEPROM: Configured!\n\0" as *const u8 as *const libc::c_char);
        }
        return 1 as libc::c_int as uint8_t;
    }
    if ignore_smarteeprom_config != 0 {
        printf(
            b"SmartEEPROM: Your settings do not match the recommended values - Some functionality may not work as expected!\0"
                as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int as uint8_t;
    }
    let ref mut fresh1 = (*puser_row1).bit;
    (*fresh1).set_SBLK(1 as libc::c_int as uint32_t);
    let ref mut fresh2 = (*puser_row1).bit;
    (*fresh2).set_PSZ(3 as libc::c_int as uint32_t);
    return write_user_row(user_row.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn strlower(mut str: *mut libc::c_char) {
    let mut c: *mut libc::c_char = str;
    while *c != 0 {
        if *c as libc::c_int >= 'A' as i32 && *c as libc::c_int <= 'Z' as i32 {
            *c = (*c as libc::c_int + 32 as libc::c_int) as libc::c_char;
        }
        c = c.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn strupper(mut str: *mut libc::c_char) {
    let mut c: *mut libc::c_char = str;
    while *c != 0 {
        if *c as libc::c_int >= 'a' as i32 && *c as libc::c_int <= 'z' as i32 {
            *c = (*c as libc::c_int - 32 as libc::c_int) as libc::c_char;
        }
        c = c.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn filesize(mut fname: *mut libc::c_char) -> libc::c_int {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    stat(fname, &mut st);
    return st.st_size as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn read_byte(mut addr: libc::c_int) -> libc::c_int {
    return read_data(addr, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn read_half_word(mut addr: libc::c_int) -> libc::c_int {
    return read_data(addr, 2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn read_word(mut addr: libc::c_int) -> libc::c_int {
    return read_data(addr, 4 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn write_word(mut addr: libc::c_int, mut data: libc::c_int) -> libc::c_int {
    return write_data(addr, 4 as libc::c_int, data);
}
#[no_mangle]
pub unsafe extern "C" fn write_half_word(
    mut addr: libc::c_int,
    mut data: libc::c_int,
) -> libc::c_int {
    return write_data(addr, 2 as libc::c_int, data);
}
#[no_mangle]
pub unsafe extern "C" fn write_byte(mut addr: libc::c_int, mut data: libc::c_int) -> libc::c_int {
    return write_data(addr, 1 as libc::c_int, data);
}
#[no_mangle]
pub unsafe extern "C" fn set_terminal_mode() -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn display_version() {
    printf(
        b"Massdrop Loader %i.%02i\n\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        6 as libc::c_int,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn display_copyright() {
    printf(
        b"Massdrop Loader  Copyright (C) 2018-2021 Massdrop Inc.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"This program is Free Software and has ABSOLUTELY NO WARRANTY\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn display_help() {
    printf(b"Usage: mdloader [options] ...\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"  -h --help                      Print this help message\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"  -v --verbose                   Print verbose messages\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"  -V --version                   Print version information\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"  -f --first                     Use first found device port as programming port\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"  -l --list                      Print valid attached devices for programming\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"  -p --port port                 Specify programming port\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"  -U --upload file               Read firmware from device into <file>\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"  -a --addr address              Read firmware starting from <address>\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"  -s --size size                 Read firmware size of <size>\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"  -D --download file             Write firmware from <file> into device\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"  -t --test                      Test mode (download/upload writes disabled, upload outputs data to stdout, restart disabled)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"     --ignore-eep                Ignore differences in SmartEEPROM configuration\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"     --cols count                Hex listing column count <count> [%i]\n\0" as *const u8
            as *const libc::c_char,
        8 as libc::c_int,
    );
    printf(
        b"     --colw width                Hex listing column width <width> [%i]\n\0" as *const u8
            as *const libc::c_char,
        4 as libc::c_int,
    );
    printf(
        b"     --restart                   Restart device after successful programming\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub static mut long_options: [option; 16] = unsafe {
    [
        {
            let mut init = option {
                name: b"restart\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &restart_after_program as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"ignore-eep\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &ignore_smarteeprom_config as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"verbose\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'V' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"list\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"first\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"port\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'p' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"download\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'D' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"upload\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'U' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"addr\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'a' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"size\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"test\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 't' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"cols\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 1000 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"colw\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 1001 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ]
};
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut mail: mailbox_t = mailbox_t {
        command: 0,
        status: 0,
        argument: C2RustUnnamed {
            inputInit: C2RustUnnamed_15 {
                comType: 0,
                traceLevel: 0,
                bank: 0,
            },
        },
    };
    let mut current_block: u64;
    verbose = 0 as libc::c_int as libc::c_char;
    testmode = 0 as libc::c_int as libc::c_char;
    first_device = 0 as libc::c_int as libc::c_char;
    restart_after_program = 0 as libc::c_int;
    ignore_smarteeprom_config = 0 as libc::c_int;
    hex_cols = 8 as libc::c_int;
    hex_colw = 4 as libc::c_int;
    display_version();
    display_copyright();
    let mut command: libc::c_int = CMD_NONE as libc::c_int;
    let mut portname: [libc::c_char; 500] = *::std::mem::transmute::<
        &[u8; 500],
        &mut [libc::c_char; 500],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut fname: [libc::c_char; 1024] = *::std::mem::transmute::<
        &[u8; 1024],
        &mut [libc::c_char; 1024],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut upload_address: libc::c_int = 0 as libc::c_int;
    let mut upload_size: libc::c_int = 0 as libc::c_int;
    let mut upload_address_set: libc::c_int = 0 as libc::c_int;
    let mut upload_size_set: libc::c_int = 0 as libc::c_int;
    while command != CMD_ABORT as libc::c_int && command != CMD_HELP as libc::c_int {
        let mut c: libc::c_int = 0;
        let mut option_index: libc::c_int = 0 as libc::c_int;
        let mut base: libc::c_int = 0;
        c = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"hvVlftp:D:U:a:s:\0" as *const u8 as *const libc::c_char,
            long_options.as_mut_ptr(),
            &mut option_index,
        );
        if c == -(1 as libc::c_int) {
            break;
        }
        match c {
            0 => {}
            118 => {
                verbose = 1 as libc::c_int as libc::c_char;
            }
            104 => {
                command = CMD_HELP as libc::c_int;
            }
            86 => {
                command = CMD_VERSION as libc::c_int;
            }
            108 => {
                if command == CMD_NONE as libc::c_int {
                    command = CMD_LISTDEV as libc::c_int;
                } else {
                    printf(
                        b"Error: Another command conflicts with list!\n\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    command = CMD_ABORT as libc::c_int;
                }
            }
            102 => {
                first_device = 1 as libc::c_int as libc::c_char;
            }
            112 => {
                sprintf(
                    portname.as_mut_ptr(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    optarg,
                );
            }
            85 => {
                sprintf(
                    fname.as_mut_ptr(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    optarg,
                );
                if command == CMD_NONE as libc::c_int {
                    command = CMD_UPLOAD as libc::c_int;
                } else {
                    printf(
                        b"Error: Another command conflicts with upload!\n\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    command = CMD_ABORT as libc::c_int;
                }
            }
            97 => {
                strlower(optarg);
                if !(strstr(optarg, b"0x\0" as *const u8 as *const libc::c_char)).is_null() {
                    base = 16 as libc::c_int;
                } else {
                    base = 10 as libc::c_int;
                }
                upload_address = strtol(optarg, 0 as *mut *mut libc::c_char, base) as libc::c_int;
                upload_address_set = 1 as libc::c_int;
            }
            115 => {
                strlower(optarg);
                if !(strstr(optarg, b"0x\0" as *const u8 as *const libc::c_char)).is_null() {
                    base = 16 as libc::c_int;
                } else {
                    base = 10 as libc::c_int;
                }
                upload_size = strtol(optarg, 0 as *mut *mut libc::c_char, base) as libc::c_int;
                upload_size_set = 1 as libc::c_int;
            }
            68 => {
                sprintf(
                    fname.as_mut_ptr(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    optarg,
                );
                if command == CMD_NONE as libc::c_int {
                    command = CMD_DOWNLOAD as libc::c_int;
                } else {
                    printf(
                        b"Error: Another command conflicts with download!\n\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    command = CMD_ABORT as libc::c_int;
                }
            }
            116 => {
                testmode = 1 as libc::c_int as libc::c_char;
            }
            1000 => {
                hex_cols = atoi(optarg);
                if hex_cols < 1 as libc::c_int {
                    printf(
                        b"Error: Hex listing column count must be greater than 0\n\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    command = CMD_ABORT as libc::c_int;
                }
            }
            1001 => {
                hex_colw = atoi(optarg);
                if hex_colw < 1 as libc::c_int {
                    printf(
                        b"Error: Hex listing column width must be greater than 0\n\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    command = CMD_ABORT as libc::c_int;
                }
            }
            _ => {
                command = CMD_ABORT as libc::c_int;
            }
        }
    }
    if command == CMD_HELP as libc::c_int || command == CMD_ABORT as libc::c_int {
        display_help();
    } else if !(command == CMD_VERSION as libc::c_int) {
        if command == CMD_LISTDEV as libc::c_int {
            list_devices(0 as *mut libc::c_char);
        } else {
            if command == CMD_NONE as libc::c_int {
                if testmode == 0 && restart_after_program == 0 {
                    display_help();
                    current_block = 12938180289712612313;
                } else {
                    current_block = 2754258178208450300;
                }
            } else {
                current_block = 2754258178208450300;
            }
            match current_block {
                12938180289712612313 => {}
                _ => {
                    if command == CMD_UPLOAD as libc::c_int {
                        let mut upload_error: libc::c_int = 0 as libc::c_int;
                        if upload_address_set == 0 {
                            printf(
                                b"Error: Upload address must be set! Ex: --addr 0x4000\n\0"
                                    as *const u8
                                    as *const libc::c_char,
                            );
                            upload_error = 1 as libc::c_int;
                        }
                        if upload_size_set == 0 {
                            printf(
                                b"Error: Upload size must be set! Ex: --size 8192\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            upload_error = 1 as libc::c_int;
                        }
                        if upload_error != 0 {
                            current_block = 12938180289712612313;
                        } else {
                            current_block = 10369920510435091891;
                        }
                    } else {
                        current_block = 10369920510435091891;
                    }
                    match current_block {
                        12938180289712612313 => {}
                        _ => {
                            if first_device != 0 {
                                let mut tries: libc::c_int = 60 as libc::c_int;
                                printf(
                                    b"Scanning for device for %i seconds\n\0" as *const u8
                                        as *const libc::c_char,
                                    tries,
                                );
                                while tries != 0 {
                                    printf(b".\0" as *const u8 as *const libc::c_char);
                                    fflush(stdout);
                                    list_devices(portname.as_mut_ptr());
                                    if *portname.as_mut_ptr() as libc::c_int != 0 as libc::c_int {
                                        printf(b"\n\0" as *const u8 as *const libc::c_char);
                                        break;
                                    } else {
                                        tries -= 1;
                                        usleep(
                                            (1000 as libc::c_int * 1000 as libc::c_int)
                                                as __useconds_t,
                                        );
                                    }
                                }
                                if tries == 0 {
                                    printf(b"\n\0" as *const u8 as *const libc::c_char);
                                    printf(
                                        b"Error: Could not find a valid device port!\n\0"
                                            as *const u8
                                            as *const libc::c_char,
                                    );
                                    current_block = 12938180289712612313;
                                } else {
                                    current_block = 11865390570819897086;
                                }
                            } else {
                                current_block = 11865390570819897086;
                            }
                            match current_block {
                                12938180289712612313 => {}
                                _ => {
                                    if *portname.as_mut_ptr() as libc::c_int == 0 as libc::c_int {
                                        printf(
                                            b"Error: No port specified!\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        print_com_example();
                                    } else {
                                        if testmode != 0 {
                                            printf(
                                                b"NOTICE: Test mode is active. Writes are disabled!\n\n\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        mail = mailbox_t {
                                            command: 0,
                                            status: 0,
                                            argument: C2RustUnnamed {
                                                inputInit: C2RustUnnamed_15 {
                                                    comType: 0,
                                                    traceLevel: 0,
                                                    bank: 0,
                                                },
                                            },
                                        };
                                        if !(test_port(
                                            portname.as_mut_ptr(),
                                            0 as libc::c_int as libc::c_char,
                                        ) == 0)
                                        {
                                            if !(test_mcu(0 as libc::c_int as libc::c_char) == 0) {
                                                printf(
                                                    b"Found MCU: %s\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    ((*mcu).name).as_mut_ptr(),
                                                );
                                                print_bootloader_version();
                                                if verbose != 0 {
                                                    printf(
                                                        b"Device ID: %08X\n\0" as *const u8
                                                            as *const libc::c_char,
                                                        (*mcu).cidr,
                                                    );
                                                }
                                                if configure_smarteeprom() == 0 {
                                                    printf(
                                                        b"Error: Config feature failed!\n\0"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                } else {
                                                    memcpy(
                                                        &mut appinfo as *mut appinfo_t
                                                            as *mut libc::c_void,
                                                        applet
                                                            .as_ref()
                                                            .as_ptr()
                                                            .offset(applet.len() as isize)
                                                            .offset(
                                                                -(::std::mem::size_of::<appinfo_t>()
                                                                    as libc::c_ulong
                                                                    as isize),
                                                            )
                                                            as *const libc::c_void,
                                                        ::std::mem::size_of::<appinfo_t>()
                                                            as libc::c_ulong,
                                                    );
                                                    if appinfo.magic
                                                        != 0x4142444d as libc::c_int as libc::c_uint
                                                    {
                                                        printf(
                                                            b"Error: Applet info not found!\n\0"
                                                                as *const u8
                                                                as *const libc::c_char,
                                                        );
                                                    } else {
                                                        if verbose != 0 {
                                                            printf(
                                                                b"Applet load address: %08X\n\0"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                                appinfo.load_addr,
                                                            );
                                                            printf(
                                                                b"Applet mail address: %08X\n\0"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                                appinfo.mail_addr,
                                                            );
                                                        }
                                                        if verbose != 0 {
                                                            printf(
                                                                b"Applet size: %i\n\0" as *const u8
                                                                    as *const libc::c_char,
                                                                applet.len(),
                                                            );
                                                        }
                                                        if send_file(
                                                            appinfo.load_addr as libc::c_int,
                                                            applet.len() as libc::c_int,
                                                            applet.as_ref().as_ptr()
                                                                as *mut libc::c_char,
                                                        ) == 0
                                                        {
                                                            printf(
                                                                b"Error: Could not send applet!\n\0"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                            );
                                                        } else {
                                                            initparams.command =
                                                                0 as libc::c_int as uint32_t;
                                                            initparams.status = STATUS_BUSY
                                                                as libc::c_int
                                                                as uint32_t;
                                                            initparams.argument.inputInit.bank =
                                                                0 as libc::c_int as uint32_t;
                                                            initparams.argument.inputInit.comType =
                                                                0 as libc::c_int as uint32_t;
                                                            initparams
                                                                .argument
                                                                .inputInit
                                                                .traceLevel =
                                                                0 as libc::c_int as uint32_t;
                                                            send_mail(&mut initparams);
                                                            if run_applet(&mut initparams)
                                                                == 0 as libc::c_int
                                                            {
                                                                printf(
                                                                    b"Error: Applet run error for init!\n\0" as *const u8
                                                                        as *const libc::c_char,
                                                                );
                                                            } else {
                                                                if verbose != 0 {
                                                                    printf(
                                                                        b"Memory Size: %08X\n\0"
                                                                            as *const u8
                                                                            as *const libc::c_char,
                                                                        initparams
                                                                            .argument
                                                                            .outputInit
                                                                            .memorySize,
                                                                    );
                                                                    printf(
                                                                        b"Buffer Addr: %08X\n\0"
                                                                            as *const u8
                                                                            as *const libc::c_char,
                                                                        initparams
                                                                            .argument
                                                                            .outputInit
                                                                            .bufferAddress,
                                                                    );
                                                                    printf(
                                                                        b"Buffer Size: %08X\n\0"
                                                                            as *const u8
                                                                            as *const libc::c_char,
                                                                        initparams
                                                                            .argument
                                                                            .outputInit
                                                                            .bufferSize,
                                                                    );
                                                                    printf(
                                                                        b"Lock Region Size: %04X\n\0" as *const u8
                                                                            as *const libc::c_char,
                                                                        initparams.argument.outputInit.memoryInfo.lockRegionSize
                                                                            as libc::c_int,
                                                                    );
                                                                    printf(
                                                                        b"Lock Region Bits: %04X\n\0" as *const u8
                                                                            as *const libc::c_char,
                                                                        initparams.argument.outputInit.memoryInfo.numbersLockBits
                                                                            as libc::c_int,
                                                                    );
                                                                    printf(
                                                                        b"Page Size: %08X\n\0"
                                                                            as *const u8
                                                                            as *const libc::c_char,
                                                                        initparams
                                                                            .argument
                                                                            .outputInit
                                                                            .pageSize,
                                                                    );
                                                                    printf(
                                                                        b"Number of Pages: %08X\n\0"
                                                                            as *const u8
                                                                            as *const libc::c_char,
                                                                        initparams
                                                                            .argument
                                                                            .outputInit
                                                                            .nbPages,
                                                                    );
                                                                    printf(
                                                                        b"App Start Page: %08X\n\0"
                                                                            as *const u8
                                                                            as *const libc::c_char,
                                                                        initparams
                                                                            .argument
                                                                            .outputInit
                                                                            .appStartPage,
                                                                    );
                                                                    printf(
                                                                        b"\n\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                }
                                                                appletinfo.command =
                                                                    0xf0 as libc::c_int as uint32_t;
                                                                appletinfo.status = STATUS_BUSY
                                                                    as libc::c_int
                                                                    as uint32_t;
                                                                send_mail(&mut appletinfo);
                                                                if run_applet(&mut appletinfo)
                                                                    == 0 as libc::c_int
                                                                {
                                                                    printf(
                                                                        b"Error: Applet run error for info!\n\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                } else {
                                                                    printf(
                                                                        b"Applet Version: %i\n\0"
                                                                            as *const u8
                                                                            as *const libc::c_char,
                                                                        appletinfo
                                                                            .argument
                                                                            .outputInfo
                                                                            .version_number
                                                                            as libc::c_int,
                                                                    );
                                                                    if initparams
                                                                        .argument
                                                                        .outputInit
                                                                        .memorySize
                                                                        != (*mcu).flash_size
                                                                            as libc::c_uint
                                                                    {
                                                                        printf(
                                                                            b"Error: MCU memory size mismatch! (Given %08X, Applet reported %08X)\n\0"
                                                                                as *const u8 as *const libc::c_char,
                                                                            (*mcu).flash_size,
                                                                            initparams.argument.outputInit.memorySize,
                                                                        );
                                                                    } else {
                                                                        bootloader_length =
                                                                            (initparams
                                                                                .argument
                                                                                .outputInit
                                                                                .appStartPage)
                                                                                .wrapping_mul(
                                                                                    initparams
                                                                                        .argument
                                                                                        .outputInit
                                                                                        .pageSize,
                                                                                );
                                                                        if bootloader_length
                                                                            == 0 as libc::c_int
                                                                                as libc::c_uint
                                                                        {
                                                                            printf(
                                                                                b"Error: Applet reported zero length bootloader!\n\0"
                                                                                    as *const u8 as *const libc::c_char,
                                                                            );
                                                                        } else {
                                                                            if verbose != 0 {
                                                                                printf(
                                                                                    b"Bootloader length: 0x%X\n\0" as *const u8
                                                                                        as *const libc::c_char,
                                                                                    bootloader_length,
                                                                                );
                                                                                print_bootloader_serial();
                                                                            }
                                                                            if command
                                                                                == CMD_DOWNLOAD
                                                                                    as libc::c_int
                                                                            {
                                                                                let mut data: *mut data_t = 0 as *mut data_t;
                                                                                data = load_file(
                                                                                    fname
                                                                                        .as_mut_ptr(
                                                                                        ),
                                                                                );
                                                                                if data.is_null() {
                                                                                    printf(
                                                                                        b"Error: Could not parse file!\n\0" as *const u8
                                                                                            as *const libc::c_char,
                                                                                    );
                                                                                    current_block = 14756504909253142488;
                                                                                } else if (*data).addr
                                                                                        < ((*mcu).flash_addr as libc::c_uint)
                                                                                            .wrapping_add(bootloader_length)
                                                                                    {
                                                                                    printf(
                                                                                        b"Error: Attempt to write to bootloader section!\n\0"
                                                                                            as *const u8 as *const libc::c_char,
                                                                                    );
                                                                                    free_data(data);
                                                                                    current_block = 14756504909253142488;
                                                                                } else if (*data).size
                                                                                        > ((*mcu).flash_size as libc::c_uint)
                                                                                            .wrapping_sub(
                                                                                                ((*mcu).flash_addr as libc::c_uint)
                                                                                                    .wrapping_add(bootloader_length),
                                                                                            )
                                                                                    {
                                                                                    printf(
                                                                                        b"Error: Attempt to write outside memory bounds!\n\0"
                                                                                            as *const u8 as *const libc::c_char,
                                                                                    );
                                                                                    free_data(data);
                                                                                    current_block = 14756504909253142488;
                                                                                } else {
                                                                                    let mut pds: *mut libc::c_char = (*data).data;
                                                                                    let mut pde: *mut libc::c_char = ((*data).data)
                                                                                        .offset((*data).size as isize);
                                                                                    let mut readbytes: libc::c_int = 0;
                                                                                    printf(
                                                                                        b"Writing firmware... \0" as *const u8
                                                                                            as *const libc::c_char,
                                                                                    );
                                                                                    if testmode != 0 {
                                                                                        printf(
                                                                                            b"(test mode disables writes) \0" as *const u8
                                                                                                as *const libc::c_char,
                                                                                        );
                                                                                    }
                                                                                    readbytes = (if (pde.offset_from(pds) as libc::c_long)
                                                                                        < initparams.argument.outputInit.bufferSize as libc::c_long
                                                                                    {
                                                                                        pde.offset_from(pds) as libc::c_long
                                                                                    } else {
                                                                                        initparams.argument.outputInit.bufferSize as libc::c_long
                                                                                    }) as libc::c_int;
                                                                                    let mut memoryOffset: libc::c_int = (*data).addr
                                                                                        as libc::c_int;
                                                                                    loop {
                                                                                        if !(readbytes > 0 as libc::c_int) {
                                                                                            current_block = 6497888915984600225;
                                                                                            break;
                                                                                        }
                                                                                        if send_file(
                                                                                            initparams.argument.outputInit.bufferAddress as libc::c_int,
                                                                                            readbytes,
                                                                                            pds,
                                                                                        ) == 0
                                                                                        {
                                                                                            printf(
                                                                                                b"\nError: Failed write to applet buffer!\n\0" as *const u8
                                                                                                    as *const libc::c_char,
                                                                                            );
                                                                                            free_data(data);
                                                                                            current_block = 14756504909253142488;
                                                                                            break;
                                                                                        } else {
                                                                                            memset(
                                                                                                &mut mail as *mut mailbox_t as *mut libc::c_void,
                                                                                                0 as libc::c_int,
                                                                                                ::std::mem::size_of::<mailbox_t>() as libc::c_ulong,
                                                                                            );
                                                                                            if testmode != 0 {
                                                                                                mail.command = 0x3 as libc::c_int as uint32_t;
                                                                                                mail
                                                                                                    .argument
                                                                                                    .inputRead
                                                                                                    .bufferAddr = initparams.argument.outputInit.bufferAddress;
                                                                                                mail.argument.inputRead.bufferSize = readbytes as uint32_t;
                                                                                                mail
                                                                                                    .argument
                                                                                                    .inputRead
                                                                                                    .memoryOffset = memoryOffset as uint32_t;
                                                                                            } else {
                                                                                                mail.command = 0x2 as libc::c_int as uint32_t;
                                                                                                mail
                                                                                                    .argument
                                                                                                    .inputWrite
                                                                                                    .bufferAddr = initparams.argument.outputInit.bufferAddress;
                                                                                                mail.argument.inputWrite.bufferSize = readbytes as uint32_t;
                                                                                                mail
                                                                                                    .argument
                                                                                                    .inputWrite
                                                                                                    .memoryOffset = memoryOffset as uint32_t;
                                                                                            }
                                                                                            send_mail(&mut mail);
                                                                                            run_applet(&mut mail);
                                                                                            if mail.status != STATUS_OK as libc::c_int as libc::c_uint {
                                                                                                printf(
                                                                                                    b"\nError: Applet failed write!\n\0" as *const u8
                                                                                                        as *const libc::c_char,
                                                                                                );
                                                                                                free_data(data);
                                                                                                current_block = 14756504909253142488;
                                                                                                break;
                                                                                            } else {
                                                                                                if testmode != 0 {
                                                                                                    if mail.argument.outputRead.bytesRead
                                                                                                        != readbytes as libc::c_uint
                                                                                                    {
                                                                                                        printf(
                                                                                                            b"\nError: Sent bytes != written bytes (%i != %i)\n\0"
                                                                                                                as *const u8 as *const libc::c_char,
                                                                                                            readbytes,
                                                                                                            mail.argument.outputRead.bytesRead,
                                                                                                        );
                                                                                                        free_data(data);
                                                                                                        current_block = 14756504909253142488;
                                                                                                        break;
                                                                                                    }
                                                                                                } else if mail.argument.outputWrite.bytesWritten
                                                                                                        != readbytes as libc::c_uint
                                                                                                    {
                                                                                                    printf(
                                                                                                        b"\nError: Sent bytes != written bytes (%i != %i)\n\0"
                                                                                                            as *const u8 as *const libc::c_char,
                                                                                                        readbytes,
                                                                                                        mail.argument.outputWrite.bytesWritten,
                                                                                                    );
                                                                                                    free_data(data);
                                                                                                    current_block = 14756504909253142488;
                                                                                                    break;
                                                                                                }
                                                                                                memoryOffset += readbytes;
                                                                                                pds = pds.offset(readbytes as isize);
                                                                                                readbytes = (if (pde.offset_from(pds) as libc::c_long)
                                                                                                    < initparams.argument.outputInit.bufferSize as libc::c_long
                                                                                                {
                                                                                                    pde.offset_from(pds) as libc::c_long
                                                                                                } else {
                                                                                                    initparams.argument.outputInit.bufferSize as libc::c_long
                                                                                                }) as libc::c_int;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    match current_block {
                                                                                        14756504909253142488 => {}
                                                                                        _ => {
                                                                                            free_data(data);
                                                                                            printf(
                                                                                                b"Complete!\n\0" as *const u8 as *const libc::c_char,
                                                                                            );
                                                                                            if restart_after_program != 0 {
                                                                                                jump_application();
                                                                                            }
                                                                                            current_block = 15905385608785565022;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                current_block = 15905385608785565022;
                                                                            }
                                                                            match current_block {
                                                                                14756504909253142488 => {}
                                                                                _ => {
                                                                                    if command == CMD_UPLOAD as libc::c_int {
                                                                                        if upload_address + upload_size > (*mcu).flash_size {
                                                                                            printf(
                                                                                                b"Error: Attempt to read outside memory bounds!\n\0"
                                                                                                    as *const u8 as *const libc::c_char,
                                                                                            );
                                                                                            current_block = 14756504909253142488;
                                                                                        } else if upload_size <= 0 as libc::c_int {
                                                                                            printf(
                                                                                                b"Error: Invalid read size!\n\0" as *const u8
                                                                                                    as *const libc::c_char,
                                                                                            );
                                                                                            current_block = 14756504909253142488;
                                                                                        } else {
                                                                                            let mut readbuffer: *mut libc::c_char = 0
                                                                                                as *mut libc::c_char;
                                                                                            let mut preadbuffer: *mut libc::c_char = 0
                                                                                                as *mut libc::c_char;
                                                                                            readbuffer = malloc(upload_size as libc::c_ulong)
                                                                                                as *mut libc::c_char;
                                                                                            preadbuffer = readbuffer;
                                                                                            if readbuffer.is_null() {
                                                                                                printf(
                                                                                                    b"Error: Could not allocate memory for firmware read!\n\0"
                                                                                                        as *const u8 as *const libc::c_char,
                                                                                                );
                                                                                                current_block = 2619327811932403950;
                                                                                            } else {
                                                                                                printf(
                                                                                                    b"Reading memory %08X - %08X... \n\0" as *const u8
                                                                                                        as *const libc::c_char,
                                                                                                    upload_address,
                                                                                                    upload_address + upload_size - 1 as libc::c_int,
                                                                                                );
                                                                                                let mut readbytes_0: libc::c_int = upload_size;
                                                                                                let mut curbytes: libc::c_int = initparams
                                                                                                    .argument
                                                                                                    .outputInit
                                                                                                    .bufferSize as libc::c_int;
                                                                                                let mut memoryOffset_0: libc::c_int = upload_address;
                                                                                                loop {
                                                                                                    if !(readbytes_0 > 0 as libc::c_int) {
                                                                                                        current_block = 5564518856185825108;
                                                                                                        break;
                                                                                                    }
                                                                                                    if curbytes > readbytes_0 {
                                                                                                        curbytes = readbytes_0;
                                                                                                    }
                                                                                                    memset(
                                                                                                        &mut mail as *mut mailbox_t as *mut libc::c_void,
                                                                                                        0 as libc::c_int,
                                                                                                        ::std::mem::size_of::<mailbox_t>() as libc::c_ulong,
                                                                                                    );
                                                                                                    mail.command = 0x3 as libc::c_int as uint32_t;
                                                                                                    mail
                                                                                                        .argument
                                                                                                        .inputRead
                                                                                                        .bufferAddr = initparams.argument.outputInit.bufferAddress;
                                                                                                    mail.argument.inputRead.bufferSize = curbytes as uint32_t;
                                                                                                    mail
                                                                                                        .argument
                                                                                                        .inputRead
                                                                                                        .memoryOffset = memoryOffset_0 as uint32_t;
                                                                                                    if send_mail(&mut mail) != 1 as libc::c_int {
                                                                                                        printf(
                                                                                                            b"\nError: Failed to send applet mail!\n\0" as *const u8
                                                                                                                as *const libc::c_char,
                                                                                                        );
                                                                                                        free(readbuffer as *mut libc::c_void);
                                                                                                        current_block = 14756504909253142488;
                                                                                                        break;
                                                                                                    } else if run_applet(&mut mail) != 1 as libc::c_int {
                                                                                                        printf(
                                                                                                            b"\nError: Failed to run applet!\n\0" as *const u8
                                                                                                                as *const libc::c_char,
                                                                                                        );
                                                                                                        free(readbuffer as *mut libc::c_void);
                                                                                                        current_block = 14756504909253142488;
                                                                                                        break;
                                                                                                    } else if mail.status
                                                                                                            != STATUS_OK as libc::c_int as libc::c_uint
                                                                                                        {
                                                                                                        printf(
                                                                                                            b"\nError: Applet status not OK! [%i]\n\0" as *const u8
                                                                                                                as *const libc::c_char,
                                                                                                            mail.status,
                                                                                                        );
                                                                                                        free(readbuffer as *mut libc::c_void);
                                                                                                        current_block = 14756504909253142488;
                                                                                                        break;
                                                                                                    } else if mail.argument.outputRead.bytesRead
                                                                                                            != curbytes as libc::c_uint
                                                                                                        {
                                                                                                        printf(
                                                                                                            b"\nError: Sent bytes != written bytes (%i != %i)\n\0"
                                                                                                                as *const u8 as *const libc::c_char,
                                                                                                            curbytes,
                                                                                                            mail.argument.outputRead.bytesRead,
                                                                                                        );
                                                                                                        free(readbuffer as *mut libc::c_void);
                                                                                                        current_block = 14756504909253142488;
                                                                                                        break;
                                                                                                    } else {
                                                                                                        let mut bufread: *mut libc::c_char = recv_file(
                                                                                                            initparams.argument.outputInit.bufferAddress as libc::c_int,
                                                                                                            curbytes,
                                                                                                        );
                                                                                                        if !bufread.is_null() {
                                                                                                            memcpy(
                                                                                                                preadbuffer as *mut libc::c_void,
                                                                                                                bufread as *const libc::c_void,
                                                                                                                curbytes as libc::c_ulong,
                                                                                                            );
                                                                                                            preadbuffer = preadbuffer.offset(curbytes as isize);
                                                                                                            free(bufread as *mut libc::c_void);
                                                                                                            memoryOffset_0 += curbytes;
                                                                                                            readbytes_0 -= curbytes;
                                                                                                        } else {
                                                                                                            printf(
                                                                                                                b"Error: Could not read data buffer!\n\0" as *const u8
                                                                                                                    as *const libc::c_char,
                                                                                                            );
                                                                                                            free(readbuffer as *mut libc::c_void);
                                                                                                            current_block = 14756504909253142488;
                                                                                                            break;
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                match current_block {
                                                                                                    14756504909253142488 => {}
                                                                                                    _ => {
                                                                                                        if testmode != 0 {
                                                                                                            print_hex_listing(
                                                                                                                readbuffer,
                                                                                                                upload_size,
                                                                                                                0 as libc::c_int,
                                                                                                                upload_address,
                                                                                                            );
                                                                                                            current_block = 15712984148872257586;
                                                                                                        } else {
                                                                                                            printf(
                                                                                                                b"Writing firmware to file... \0" as *const u8
                                                                                                                    as *const libc::c_char,
                                                                                                            );
                                                                                                            let mut fOut: *mut FILE = fopen(
                                                                                                                fname.as_mut_ptr(),
                                                                                                                b"wb\0" as *const u8 as *const libc::c_char,
                                                                                                            );
                                                                                                            if fOut.is_null() {
                                                                                                                printf(b"Failed!\n\0" as *const u8 as *const libc::c_char);
                                                                                                                printf(
                                                                                                                    b"Error: Could not open file for firmware read output!\n\0"
                                                                                                                        as *const u8 as *const libc::c_char,
                                                                                                                );
                                                                                                                current_block = 14756504909253142488;
                                                                                                            } else {
                                                                                                                fwrite(
                                                                                                                    readbuffer as *const libc::c_void,
                                                                                                                    upload_size as libc::c_ulong,
                                                                                                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                                                                                    fOut,
                                                                                                                );
                                                                                                                fclose(fOut);
                                                                                                                printf(b"Success!\n\0" as *const u8 as *const libc::c_char);
                                                                                                                current_block = 15712984148872257586;
                                                                                                            }
                                                                                                        }
                                                                                                        match current_block {
                                                                                                            14756504909253142488 => {}
                                                                                                            _ => {
                                                                                                                free(readbuffer as *mut libc::c_void);
                                                                                                                printf(
                                                                                                                    b"Complete!\n\0" as *const u8 as *const libc::c_char,
                                                                                                                );
                                                                                                                current_block = 2619327811932403950;
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        current_block = 2619327811932403950;
                                                                                    }
                                                                                    match current_block {
                                                                                        14756504909253142488 => {}
                                                                                        _ => {
                                                                                            if command == CMD_NONE as libc::c_int
                                                                                                && restart_after_program != 0
                                                                                            {
                                                                                                jump_application();
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            close_port(0 as libc::c_int as libc::c_char);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
