use ::libc;
use crate::atmel::applet::*;
extern "C" {
    pub type __dirstream;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn cfsetspeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    fn cfmakeraw(__termios_p: *mut termios);
    fn tcflush(__fd: libc::c_int, __queue_selector: libc::c_int) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static mut verbose: libc::c_char;
    static mut testmode: libc::c_char;
    static mut initparams: mailbox_t;
    static mut mcu: *mut mcu_t;
    fn check_bootloader_write_attempt(addr: libc::c_int) -> libc::c_int;
    static mut read_error: libc::c_int;
    fn test_mcu(silent: libc::c_char) -> libc::c_int;
    fn test_port(portname: *mut libc::c_char, silent: libc::c_char) -> libc::c_int;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type C2RustUnnamed = libc::c_uint;
pub const DT_WHT: C2RustUnnamed = 14;
pub const DT_SOCK: C2RustUnnamed = 12;
pub const DT_LNK: C2RustUnnamed = 10;
pub const DT_REG: C2RustUnnamed = 8;
pub const DT_BLK: C2RustUnnamed = 6;
pub const DT_DIR: C2RustUnnamed = 4;
pub const DT_CHR: C2RustUnnamed = 2;
pub const DT_FIFO: C2RustUnnamed = 1;
pub const DT_UNKNOWN: C2RustUnnamed = 0;
pub type DIR = __dirstream;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

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
#[no_mangle]
pub static mut gport: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn print_com_example() {
    printf(b"Example: -p /dev/ttyACM0\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn read_data(
    mut addr: libc::c_int,
    mut readsize: libc::c_int,
) -> libc::c_int {
    read_error = 1 as libc::c_int;
    let mut readdata: libc::c_int = 0 as libc::c_int;
    let mut wbuf: [libc::c_char; 12] = *::std::mem::transmute::<
        &[u8; 12],
        &mut [libc::c_char; 12],
    >(b"!XXXXXXXX,#\0");
    let mut ret: libc::c_long = 0;
    if readsize == 1 as libc::c_int {
        sprintf(
            wbuf.as_mut_ptr(),
            b"%c%02x,%c\0" as *const u8 as *const libc::c_char,
            'o' as i32,
            addr,
            '#' as i32,
        );
    } else if readsize == 2 as libc::c_int {
        sprintf(
            wbuf.as_mut_ptr(),
            b"%c%04x,%c\0" as *const u8 as *const libc::c_char,
            'h' as i32,
            addr,
            '#' as i32,
        );
    } else if readsize == 4 as libc::c_int {
        sprintf(
            wbuf.as_mut_ptr(),
            b"%c%08x,%c\0" as *const u8 as *const libc::c_char,
            'w' as i32,
            addr,
            '#' as i32,
        );
    } else {
        if verbose != 0 {
            printf(
                b"Error: Invalid read size given (%i)\n\0" as *const u8
                    as *const libc::c_char,
                readsize,
            );
        }
        return 0 as libc::c_int;
    }
    if verbose as libc::c_int > 0 as libc::c_int {
        printf(
            b"Write: [%s]\n\0" as *const u8 as *const libc::c_char,
            wbuf.as_mut_ptr(),
        );
    }
    tcflush(gport, 2 as libc::c_int);
    let mut writelen: libc::c_long = strlen(wbuf.as_mut_ptr()) as libc::c_long;
    ret = write(gport, wbuf.as_mut_ptr() as *const libc::c_void, writelen as size_t);
    if ret == -(1 as libc::c_int) as libc::c_long {
        if verbose != 0 {
            printf(
                b"Error writing port [%s](%s)\n\0" as *const u8 as *const libc::c_char,
                wbuf.as_mut_ptr(),
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    usleep(
        (1000 as libc::c_int + 20 as libc::c_int * writelen as libc::c_int)
            as __useconds_t,
    );
    if ret != writelen {
        if verbose != 0 {
            printf(
                b"Error writing %ld bytes [%ld](%s)\n\0" as *const u8
                    as *const libc::c_char,
                writelen,
                ret,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    ret = read(
        gport,
        &mut readdata as *mut libc::c_int as *mut libc::c_void,
        readsize as size_t,
    );
    if ret == -(1 as libc::c_int) as libc::c_long {
        if verbose != 0 {
            printf(
                b"Error reading port [%i][%ld](%s)\n\0" as *const u8
                    as *const libc::c_char,
                readsize,
                ret,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    if ret != readsize as libc::c_long {
        if verbose != 0 {
            printf(
                b"Error reading %i bytes! [%ld]\n\0" as *const u8 as *const libc::c_char,
                readsize,
                ret,
            );
        }
        return 0 as libc::c_int;
    }
    read_error = 0 as libc::c_int;
    return readdata;
}
#[no_mangle]
pub unsafe extern "C" fn write_data(
    mut addr: libc::c_int,
    mut writesize: libc::c_int,
    mut data: libc::c_int,
) -> libc::c_int {
    if check_bootloader_write_attempt(addr) != 0 {
        return 0 as libc::c_int;
    }
    let mut wbuf: [libc::c_char; 20] = *::std::mem::transmute::<
        &[u8; 20],
        &mut [libc::c_char; 20],
    >(b"!XXXXXXXX,XXXXXXXX#\0");
    let mut ret: libc::c_long = 0;
    if writesize == 1 as libc::c_int {
        sprintf(
            wbuf.as_mut_ptr(),
            b"%c%08x,%02x%c\0" as *const u8 as *const libc::c_char,
            'O' as i32,
            addr,
            data,
            '#' as i32,
        );
    } else if writesize == 2 as libc::c_int {
        sprintf(
            wbuf.as_mut_ptr(),
            b"%c%08x,%04x%c\0" as *const u8 as *const libc::c_char,
            'H' as i32,
            addr,
            data,
            '#' as i32,
        );
    } else if writesize == 4 as libc::c_int {
        sprintf(
            wbuf.as_mut_ptr(),
            b"%c%08x,%08x%c\0" as *const u8 as *const libc::c_char,
            'W' as i32,
            addr,
            data,
            '#' as i32,
        );
    } else {
        if verbose != 0 {
            printf(
                b"Error: Invalid write size given (%i)\n\0" as *const u8
                    as *const libc::c_char,
                writesize,
            );
        }
        return 0 as libc::c_int;
    }
    if verbose != 0 {
        printf(
            b"Write %i bytes: [%s]\n\0" as *const u8 as *const libc::c_char,
            writesize,
            wbuf.as_mut_ptr(),
        );
    }
    let mut writelen: libc::c_long = strlen(wbuf.as_mut_ptr()) as libc::c_long;
    ret = write(gport, wbuf.as_mut_ptr() as *const libc::c_void, writelen as size_t);
    if ret == -(1 as libc::c_int) as libc::c_long {
        if verbose != 0 {
            printf(
                b"Error writing port [%s](%s)\n\0" as *const u8 as *const libc::c_char,
                wbuf.as_mut_ptr(),
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    usleep(
        (1000 as libc::c_int + 20 as libc::c_int * writelen as libc::c_int)
            as __useconds_t,
    );
    if ret != writelen {
        if verbose != 0 {
            printf(
                b"Error writing %i bytes! [%ld]\n\0" as *const u8 as *const libc::c_char,
                writesize,
                ret,
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn goto_address(mut addr: libc::c_int) -> libc::c_int {
    let mut wbuf: [libc::c_char; 11] = *::std::mem::transmute::<
        &[u8; 11],
        &mut [libc::c_char; 11],
    >(b"!XXXXXXXX#\0");
    let mut ret: libc::c_long = 0;
    sprintf(
        wbuf.as_mut_ptr(),
        b"%c%08x%c\0" as *const u8 as *const libc::c_char,
        'G' as i32,
        addr,
        '#' as i32,
    );
    if verbose != 0 {
        printf(
            b"Write: [%s]\n\0" as *const u8 as *const libc::c_char,
            wbuf.as_mut_ptr(),
        );
    }
    let mut writelen: libc::c_long = strlen(wbuf.as_mut_ptr()) as libc::c_long;
    ret = write(gport, wbuf.as_mut_ptr() as *const libc::c_void, writelen as size_t);
    if ret == -(1 as libc::c_int) as libc::c_long {
        if verbose != 0 {
            printf(
                b"Error writing port [%s](%s)\n\0" as *const u8 as *const libc::c_char,
                wbuf.as_mut_ptr(),
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    usleep(
        (1000 as libc::c_int + 20 as libc::c_int * writelen as libc::c_int)
            as __useconds_t,
    );
    if ret != writelen {
        if verbose != 0 {
            printf(
                b"Error writing goto address! [%ld]\n\0" as *const u8
                    as *const libc::c_char,
                ret,
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn recv_file(
    mut addr: libc::c_int,
    mut bytes: libc::c_int,
) -> *mut libc::c_char {
    let mut wbuf: [libc::c_char; 20] = *::std::mem::transmute::<
        &[u8; 20],
        &mut [libc::c_char; 20],
    >(b"!XXXXXXXX,XXXXXXXX#\0");
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    data = calloc(
        (bytes + 1 as libc::c_int) as libc::c_ulong,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) as *mut libc::c_char;
    if data.is_null() {
        printf(
            b"Error allocating read buffer memory!\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut libc::c_char;
    }
    let mut pdata: *mut libc::c_char = data;
    let mut ret: libc::c_long = 0;
    let mut retries: libc::c_int = 10 as libc::c_int;
    let mut readsize: libc::c_int = 250 as libc::c_int;
    if initparams.argument.outputInit.bufferSize > 0 as libc::c_int as libc::c_uint
        && initparams.argument.outputInit.bufferSize < readsize as libc::c_uint
    {
        readsize = initparams.argument.outputInit.bufferSize as libc::c_int;
    }
    while bytes > 0 as libc::c_int {
        if readsize > bytes {
            readsize = bytes;
        }
        sprintf(
            wbuf.as_mut_ptr(),
            b"%c%08x,%08x%c\0" as *const u8 as *const libc::c_char,
            'R' as i32,
            addr,
            readsize,
            '#' as i32,
        );
        if verbose as libc::c_int > 0 as libc::c_int {
            printf(
                b"Write: [%s]\n\0" as *const u8 as *const libc::c_char,
                wbuf.as_mut_ptr(),
            );
        }
        tcflush(gport, 2 as libc::c_int);
        let mut writelen: libc::c_long = strlen(wbuf.as_mut_ptr()) as libc::c_long;
        ret = write(gport, wbuf.as_mut_ptr() as *const libc::c_void, writelen as size_t);
        if ret == -(1 as libc::c_int) as libc::c_long {
            if verbose != 0 {
                printf(
                    b"Error writing port [%s](%s)\n\0" as *const u8
                        as *const libc::c_char,
                    wbuf.as_mut_ptr(),
                    strerror(*__errno_location()),
                );
            }
            free(data as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
        usleep(
            (1000 as libc::c_int + 20 as libc::c_int * writelen as libc::c_int)
                as __useconds_t,
        );
        ret = read(gport, pdata as *mut libc::c_void, readsize as size_t);
        if ret == -(1 as libc::c_int) as libc::c_long {
            if verbose != 0 {
                printf(
                    b"Error reading port [%i][%ld](%s)\n\0" as *const u8
                        as *const libc::c_char,
                    readsize,
                    ret,
                    strerror(*__errno_location()),
                );
            }
            free(data as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
        if ret != readsize as libc::c_long {
            if verbose != 0 {
                printf(
                    b"Error reading %i bytes! [%ld](%s)\n\0" as *const u8
                        as *const libc::c_char,
                    readsize,
                    ret,
                    strerror(*__errno_location()),
                );
            }
            if retries <= 0 as libc::c_int {
                if verbose != 0 {
                    printf(
                        b"No retries remain!\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                free(data as *mut libc::c_void);
                return 0 as *mut libc::c_char;
            }
            retries -= 1;
            if verbose != 0 {
                printf(
                    b"Retrying read... (%i)\n\0" as *const u8 as *const libc::c_char,
                    10 as libc::c_int - retries,
                );
            }
        } else {
            if verbose as libc::c_int > 0 as libc::c_int {
                printf(b"Recv: [%ld]\n\0" as *const u8 as *const libc::c_char, ret);
            }
            retries = 10 as libc::c_int;
            bytes = (bytes as libc::c_long - ret) as libc::c_int;
            addr = (addr as libc::c_long + ret) as libc::c_int;
            pdata = pdata.offset(ret as isize);
        }
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn send_file(
    mut addr: libc::c_int,
    mut bytes: libc::c_int,
    mut data: *mut libc::c_char,
) -> libc::c_int {
    if check_bootloader_write_attempt(addr) != 0 {
        return 0 as libc::c_int;
    }
    if bytes < 1 as libc::c_int {
        printf(b"Error: No data to send!\n\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    let mut wbuf: [libc::c_char; 20] = *::std::mem::transmute::<
        &[u8; 20],
        &mut [libc::c_char; 20],
    >(b"!XXXXXXXX,XXXXXXXX#\0");
    let mut ret: libc::c_long = 0;
    let mut pdata: *mut libc::c_char = data;
    let mut writelen: libc::c_long = 0;
    let mut writesize: libc::c_int = 250 as libc::c_int;
    if initparams.argument.outputInit.bufferSize > 0 as libc::c_int as libc::c_uint
        && initparams.argument.outputInit.bufferSize < writesize as libc::c_uint
    {
        writesize = initparams.argument.outputInit.bufferSize as libc::c_int;
    }
    while bytes > 0 as libc::c_int {
        if writesize > bytes {
            writesize = bytes;
        }
        sprintf(
            wbuf.as_mut_ptr(),
            b"%c%08x,%08x%c\0" as *const u8 as *const libc::c_char,
            'S' as i32,
            addr,
            writesize,
            '#' as i32,
        );
        if verbose as libc::c_int > 0 as libc::c_int {
            printf(
                b"Write: [%s]\n\0" as *const u8 as *const libc::c_char,
                wbuf.as_mut_ptr(),
            );
        }
        writelen = strlen(wbuf.as_mut_ptr()) as libc::c_long;
        ret = write(gport, wbuf.as_mut_ptr() as *const libc::c_void, writelen as size_t);
        if ret == -(1 as libc::c_int) as libc::c_long {
            if verbose != 0 {
                printf(
                    b"Error writing port [%s](%s)\n\0" as *const u8
                        as *const libc::c_char,
                    wbuf.as_mut_ptr(),
                    strerror(*__errno_location()),
                );
            }
            return 0 as libc::c_int;
        }
        usleep(
            (1000 as libc::c_int + 20 as libc::c_int * writelen as libc::c_int)
                as __useconds_t,
        );
        if ret != writelen {
            if verbose != 0 {
                printf(
                    b"Error writing port [%s](%s)\n\0" as *const u8
                        as *const libc::c_char,
                    wbuf.as_mut_ptr(),
                    strerror(*__errno_location()),
                );
            }
            return 0 as libc::c_int;
        }
        if verbose as libc::c_int > 0 as libc::c_int {
            printf(
                b"Write: %i bytes\n\0" as *const u8 as *const libc::c_char,
                writesize,
            );
        }
        ret = write(gport, pdata as *const libc::c_void, writesize as size_t);
        if ret == -(1 as libc::c_int) as libc::c_long {
            if verbose != 0 {
                printf(
                    b"Error writing port [%ld][%i](%s)\n\0" as *const u8
                        as *const libc::c_char,
                    ret,
                    writesize,
                    strerror(*__errno_location()),
                );
            }
            return 0 as libc::c_int;
        }
        usleep((1000 as libc::c_int + 20 as libc::c_int * writesize) as __useconds_t);
        if ret != writesize as libc::c_long {
            printf(
                b"Error writing port [%ld][%i](%s)\n\0" as *const u8
                    as *const libc::c_char,
                ret,
                writesize,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        }
        bytes = (bytes as libc::c_long - ret) as libc::c_int;
        addr = (addr as libc::c_long + ret) as libc::c_int;
        pdata = pdata.offset(ret as isize);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn print_bootloader_version() -> libc::c_int {
    let mut wbuf: [libc::c_char; 3] = *::std::mem::transmute::<
        &[u8; 3],
        &mut [libc::c_char; 3],
    >(b"!#\0");
    let mut readdata: [libc::c_char; 128] = *::std::mem::transmute::<
        &[u8; 128],
        &mut [libc::c_char; 128],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut ret: libc::c_long = 0;
    let mut readsize: libc::c_int = (::std::mem::size_of::<[libc::c_char; 128]>()
        as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    sprintf(
        wbuf.as_mut_ptr(),
        b"%c%c\0" as *const u8 as *const libc::c_char,
        'V' as i32,
        '#' as i32,
    );
    if verbose as libc::c_int > 0 as libc::c_int {
        printf(
            b"Write: [%s]\n\0" as *const u8 as *const libc::c_char,
            wbuf.as_mut_ptr(),
        );
    }
    let mut writelen: libc::c_long = strlen(wbuf.as_mut_ptr()) as libc::c_long;
    ret = write(gport, wbuf.as_mut_ptr() as *const libc::c_void, writelen as size_t);
    if ret == -(1 as libc::c_int) as libc::c_long {
        if verbose != 0 {
            printf(
                b"Version: Error writing port [%s](%s)\n\0" as *const u8
                    as *const libc::c_char,
                wbuf.as_mut_ptr(),
                strerror(*__errno_location()),
            );
        } else {
            printf(
                b"Version: Error retrieving!\n\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    usleep(
        (1000 as libc::c_int + 20 as libc::c_int * writelen as libc::c_int)
            as __useconds_t,
    );
    ret = read(
        gport,
        &mut readdata as *mut [libc::c_char; 128] as *mut libc::c_void,
        readsize as size_t,
    );
    if ret == -(1 as libc::c_int) as libc::c_long {
        if verbose != 0 {
            printf(
                b"Version: Error reading port [%i][%ld](%s)\n\0" as *const u8
                    as *const libc::c_char,
                readsize,
                ret,
                strerror(*__errno_location()),
            );
        } else {
            printf(
                b"Version: Error retrieving!\n\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    while readdata[(strlen(readdata.as_mut_ptr()))
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize] as libc::c_int
        == '\n' as i32
        || readdata[(strlen(readdata.as_mut_ptr()))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize] as libc::c_int
            == '\r' as i32
    {
        readdata[(strlen(readdata.as_mut_ptr()))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = 0 as libc::c_int as libc::c_char;
    }
    printf(
        b"Bootloader version: %s\n\0" as *const u8 as *const libc::c_char,
        readdata.as_mut_ptr(),
    );
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn set_normal_mode() -> libc::c_int {
    if verbose != 0 {
        printf(b"Setting normal mode... \0" as *const u8 as *const libc::c_char);
    }
    let mut retbuf: libc::c_int = 0 as libc::c_int;
    let mut wbuf: [libc::c_char; 3] = *::std::mem::transmute::<
        &[u8; 3],
        &mut [libc::c_char; 3],
    >(b"!#\0");
    let mut ret: libc::c_long = 0;
    sprintf(
        wbuf.as_mut_ptr(),
        b"%c%c\0" as *const u8 as *const libc::c_char,
        'N' as i32,
        '#' as i32,
    );
    let mut writelen: libc::c_long = strlen(wbuf.as_mut_ptr()) as libc::c_long;
    ret = write(gport, wbuf.as_mut_ptr() as *const libc::c_void, writelen as size_t);
    if ret == -(1 as libc::c_int) as libc::c_long {
        if verbose != 0 {
            printf(
                b"Failed! (%s)\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    usleep(
        (1000 as libc::c_int + 20 as libc::c_int * writelen as libc::c_int)
            as __useconds_t,
    );
    if ret != writelen {
        if verbose != 0 {
            printf(
                b"Error writing %ld bytes! [%ld](%s)\n\0" as *const u8
                    as *const libc::c_char,
                writelen,
                ret,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    let mut readlen: libc::c_int = 2 as libc::c_int;
    ret = read(
        gport,
        &mut retbuf as *mut libc::c_int as *mut libc::c_void,
        readlen as size_t,
    );
    if ret == -(1 as libc::c_int) as libc::c_long {
        if verbose != 0 {
            printf(
                b"Error reading port [%i][%ld](%s)\n\0" as *const u8
                    as *const libc::c_char,
                readlen,
                ret,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    if ret != readlen as libc::c_long {
        if verbose != 0 {
            printf(
                b"Error reading %i bytes! [%ld][%04x](%s)\n\0" as *const u8
                    as *const libc::c_char,
                readlen,
                ret,
                retbuf,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    if retbuf & 0xffff as libc::c_int != 0xd0a as libc::c_int {
        if verbose != 0 {
            printf(
                b"Error: Incorrect response! [%ld][%04x](%s)\n\0" as *const u8
                    as *const libc::c_char,
                ret,
                retbuf,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    if verbose != 0 {
        printf(b"Success!\n\0" as *const u8 as *const libc::c_char);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jump_application() -> libc::c_int {
    printf(b"Booting device... \0" as *const u8 as *const libc::c_char);
    if testmode != 0 {
        printf(b"(test mode disables restart)\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    let mut wbuf: [libc::c_char; 3] = *::std::mem::transmute::<
        &[u8; 3],
        &mut [libc::c_char; 3],
    >(b"!#\0");
    let mut ret: libc::c_long = 0;
    let mut writelen: libc::c_long = strlen(wbuf.as_mut_ptr()) as libc::c_long;
    sprintf(
        wbuf.as_mut_ptr(),
        b"%c%c\0" as *const u8 as *const libc::c_char,
        'X' as i32,
        '#' as i32,
    );
    ret = write(gport, wbuf.as_mut_ptr() as *const libc::c_void, writelen as size_t);
    if ret == -(1 as libc::c_int) as libc::c_long {
        printf(
            b"Failed! (%s)\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    usleep(
        (1000 as libc::c_int + 20 as libc::c_int * writelen as libc::c_int)
            as __useconds_t,
    );
    printf(b"Success!\n\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn open_port(
    mut portname: *mut libc::c_char,
    mut silent: libc::c_char,
) -> libc::c_int {
    if silent == 0 || verbose as libc::c_int != 0 {
        printf(b"Opening port '%s'... \0" as *const u8 as *const libc::c_char, portname);
    }
    gport = open(portname, 0o2 as libc::c_int | 0o400 as libc::c_int);
    if gport == -(1 as libc::c_int) {
        if silent == 0 || verbose as libc::c_int != 0 {
            printf(b"Failed!\0" as *const u8 as *const libc::c_char);
            printf(
                b" (%s)\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        return 0 as libc::c_int;
    }
    if silent == 0 || verbose as libc::c_int != 0 {
        printf(b"Success!\n\0" as *const u8 as *const libc::c_char);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn close_port(mut silent: libc::c_char) -> libc::c_int {
    if silent == 0 || verbose as libc::c_int != 0 {
        printf(b"Closing port... \0" as *const u8 as *const libc::c_char);
    }
    if close(gport) == -(1 as libc::c_int) {
        if silent == 0 || verbose as libc::c_int != 0 {
            printf(
                b"Failed! (%s)\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    if silent == 0 || verbose as libc::c_int != 0 {
        printf(b"Success!\n\0" as *const u8 as *const libc::c_char);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn config_port() -> libc::c_int {
    if verbose != 0 {
        printf(b"Configuring port... \n\0" as *const u8 as *const libc::c_char);
    }
    let mut tty: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    memset(
        &mut tty as *mut termios as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<termios>() as libc::c_ulong,
    );
    if verbose != 0 {
        printf(b"  Get config... \0" as *const u8 as *const libc::c_char);
    }
    if tcgetattr(gport, &mut tty) != 0 as libc::c_int {
        if verbose != 0 {
            printf(
                b"Failed! (%s)\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    if verbose != 0 {
        printf(b"Success!\n\0" as *const u8 as *const libc::c_char);
    }
    cfsetspeed(&mut tty, 0o10002 as libc::c_int as speed_t);
    cfmakeraw(&mut tty);
    tty
        .c_cflag = (0o60 as libc::c_int | 0o200 as libc::c_int | 0o4000 as libc::c_int)
        as tcflag_t;
    tty.c_iflag = 0o1 as libc::c_int as tcflag_t;
    tty.c_lflag = 0 as libc::c_int as tcflag_t;
    tty.c_oflag = 0 as libc::c_int as tcflag_t;
    tty.c_cc[6 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
    tty.c_cc[5 as libc::c_int as usize] = 5 as libc::c_int as cc_t;
    if verbose != 0 {
        printf(b"  Set config... \0" as *const u8 as *const libc::c_char);
    }
    if tcsetattr(gport, 0 as libc::c_int, &mut tty) != 0 as libc::c_int {
        if verbose != 0 {
            printf(
                b"Failed! (%s)\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    if verbose != 0 {
        printf(b"Success!\n\0" as *const u8 as *const libc::c_char);
    }
    tcflush(gport, 2 as libc::c_int);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn list_devices(mut first_device: *mut libc::c_char) {
    let mut devdir: [libc::c_char; 5] = *::std::mem::transmute::<
        &[u8; 5],
        &mut [libc::c_char; 5],
    >(b"/dev\0");
    let mut pdev: *mut DIR = 0 as *mut DIR;
    pdev = opendir(devdir.as_mut_ptr());
    if !pdev.is_null() {
        let mut pdevfile: *mut dirent = 0 as *mut dirent;
        let mut portcount: libc::c_int = 0 as libc::c_int;
        if first_device.is_null() {
            printf(b"Bootloader port listing\n\0" as *const u8 as *const libc::c_char);
            printf(
                b"-----------------------------\n\0" as *const u8 as *const libc::c_char,
            );
        }
        loop {
            pdevfile = readdir(pdev);
            if pdevfile.is_null() {
                break;
            }
            if (*pdevfile).d_type as libc::c_int == DT_CHR as libc::c_int {
                if strstr(
                    ((*pdevfile).d_name).as_mut_ptr(),
                    b"ttyACM\0" as *const u8 as *const libc::c_char,
                ) == ((*pdevfile).d_name).as_mut_ptr()
                {
                    let mut pathbuf: [libc::c_char; 262] = *::std::mem::transmute::<
                        &[u8; 262],
                        &mut [libc::c_char; 262],
                    >(
                        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                    );
                    sprintf(
                        pathbuf.as_mut_ptr(),
                        b"%s/%s\0" as *const u8 as *const libc::c_char,
                        devdir.as_mut_ptr(),
                        ((*pdevfile).d_name).as_mut_ptr(),
                    );
                    if test_port(pathbuf.as_mut_ptr(), 0 as libc::c_int as libc::c_char)
                        != 0
                    {
                        if test_mcu(1 as libc::c_int as libc::c_char) != 0 {
                            if !first_device.is_null() {
                                printf(b"\n\0" as *const u8 as *const libc::c_char);
                            }
                            printf(
                                b"Device port: %s (%s)\n\0" as *const u8
                                    as *const libc::c_char,
                                pathbuf.as_mut_ptr(),
                                ((*mcu).name).as_mut_ptr(),
                            );
                            if !first_device.is_null() {
                                close_port(1 as libc::c_int as libc::c_char);
                                strcpy(first_device, pathbuf.as_mut_ptr());
                                return;
                            }
                            portcount += 1;
                        }
                        close_port(1 as libc::c_int as libc::c_char);
                    }
                }
            }
        }
        closedir(pdev);
        if first_device.is_null() {
            if portcount == 0 as libc::c_int {
                printf(b"No devices found!\n\0" as *const u8 as *const libc::c_char);
            }
        }
    } else {
        printf(
            b"Error: Could not open dev directory (%s)\n\0" as *const u8
                as *const libc::c_char,
            strerror(*__errno_location()),
        );
    };
}
