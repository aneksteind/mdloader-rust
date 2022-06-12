use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn filesize(fname: *mut libc::c_char) -> libc::c_int;
    static mut bootloader_length: uint32_t;
    static mut mcu: *mut mcu_t;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type uint8_t = __uint8_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_s {
    pub addr: uint32_t,
    pub size: uint32_t,
    pub data: *mut libc::c_char,
}
pub type data_t = data_s;
pub type hex_record_t = hex_record_s;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct hex_record_s {
    pub start_code: libc::c_char,
    pub byte_count: [libc::c_char; 2],
    pub address: C2RustUnnamed,
    pub record_type: [libc::c_char; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub i: uint32_t,
    pub c: [uint8_t; 4],
}
#[no_mangle]
pub unsafe extern "C" fn free_data(mut data: *mut data_t) {
    if data.is_null() {
        printf(
            b"Error: Parser: Attempt to free NULL data!\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if !((*data).data).is_null() {
        free((*data).data as *mut libc::c_void);
        let ref mut fresh0 = (*data).data;
        *fresh0 = 0 as *mut libc::c_char;
    }
    free(data as *mut libc::c_void);
    data = 0 as *mut data_t;
}
#[no_mangle]
pub unsafe extern "C" fn create_data(mut data_length: uint32_t) -> *mut data_t {
    let mut data: *mut data_t = malloc(::std::mem::size_of::<data_t>() as libc::c_ulong)
        as *mut data_t;
    if data.is_null() {
        printf(
            b"ERROR: Parser: Could not allocate parser memory!\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut data_t;
    }
    let ref mut fresh1 = (*data).data;
    *fresh1 = malloc(data_length as libc::c_ulong) as *mut libc::c_char;
    if ((*data).data).is_null() {
        printf(
            b"ERROR: Parser: Could not allocate parser data memory!\n\0" as *const u8
                as *const libc::c_char,
        );
        free_data(data);
        return 0 as *mut data_t;
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn parse_bin(
    mut rawdata: *mut libc::c_char,
    mut rawlength: uint32_t,
) -> *mut data_t {
    if rawdata.is_null() {
        printf(
            b"ERROR: Parser: Bin: Raw data null!\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut data_t;
    }
    if rawlength == 0 {
        printf(
            b"ERROR: Parser: Bin: Raw data length zero!\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut data_t;
    }
    let mut data: *mut data_t = create_data(rawlength);
    if !data.is_null() {
        memcpy(
            (*data).data as *mut libc::c_void,
            rawdata as *const libc::c_void,
            rawlength as libc::c_ulong,
        );
        (*data).size = rawlength;
        (*data)
            .addr = ((*mcu).flash_addr as libc::c_uint).wrapping_add(bootloader_length);
    } else {
        printf(
            b"Error: Parser: Bin: Error creating parser storage!\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return data;
}
#[no_mangle]
pub static mut hex_conv_error: libc::c_char = 0;
#[no_mangle]
pub unsafe extern "C" fn ascii_to_hex(
    mut bh: libc::c_char,
    mut bl: libc::c_char,
) -> libc::c_char {
    hex_conv_error = 0 as libc::c_int as libc::c_char;
    if bh as libc::c_int >= '0' as i32 && bh as libc::c_int <= '9' as i32 {
        bh = (bh as libc::c_int - '0' as i32) as libc::c_char;
    } else if bh as libc::c_int >= 'A' as i32 && bh as libc::c_int <= 'F' as i32 {
        bh = (bh as libc::c_int - ('A' as i32 - 0xa as libc::c_int)) as libc::c_char;
    } else if bh as libc::c_int >= 'a' as i32 && bh as libc::c_int <= 'a' as i32 {
        bh = (bh as libc::c_int - ('a' as i32 - 0xa as libc::c_int)) as libc::c_char;
    } else {
        hex_conv_error = 1 as libc::c_int as libc::c_char;
        return 0 as libc::c_int as libc::c_char;
    }
    if bl as libc::c_int >= '0' as i32 && bl as libc::c_int <= '9' as i32 {
        bl = (bl as libc::c_int - '0' as i32) as libc::c_char;
    } else if bl as libc::c_int >= 'A' as i32 && bl as libc::c_int <= 'F' as i32 {
        bl = (bl as libc::c_int - ('A' as i32 - 0xa as libc::c_int)) as libc::c_char;
    } else if bl as libc::c_int >= 'a' as i32 && bl as libc::c_int <= 'a' as i32 {
        bl = (bl as libc::c_int - ('a' as i32 - 0xa as libc::c_int)) as libc::c_char;
    } else {
        hex_conv_error = 1 as libc::c_int as libc::c_char;
        return 0 as libc::c_int as libc::c_char;
    }
    return ((bh as libc::c_int) << 4 as libc::c_int | bl as libc::c_int) as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn parse_hex(
    mut rawdata: *mut libc::c_char,
    mut rawlength: uint32_t,
) -> *mut data_t {
    if rawdata.is_null() {
        printf(
            b"ERROR: Parser: Hex: Raw data null!\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut data_t;
    }
    if rawlength == 0 {
        printf(
            b"ERROR: Parser: Hex: Raw data length zero!\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut data_t;
    }
    let mut first_address_set: uint8_t = 0 as libc::c_int as uint8_t;
    let mut base_address: uint32_t = 0 as libc::c_int as uint32_t;
    let mut start_offset: uint32_t = 0;
    let mut rds: *mut uint8_t = rawdata as *mut uint8_t;
    let mut rde: *mut uint8_t = (rawdata as *mut uint8_t).offset(rawlength as isize);
    let mut bindata: *mut uint8_t = rawdata as *mut uint8_t;
    let mut hex: *mut hex_record_t = 0 as *mut hex_record_t;
    let mut hex_data: *mut uint8_t = 0 as *mut uint8_t;
    let mut checksum: uint8_t = 0;
    let mut checksum_given: uint8_t = 0;
    let mut line: uint16_t = 0 as libc::c_int as uint16_t;
    let mut binlength: uint32_t = 0 as libc::c_int as uint32_t;
    let mut byte_count: uint8_t = 0;
    let mut next_address: uint32_t = 0 as libc::c_int as uint32_t;
    let mut start_segment_address: uint32_t = 0 as libc::c_int as uint32_t;
    let mut start_segment_address_set: uint8_t = 0 as libc::c_int as uint8_t;
    while rds < rde {
        line = line.wrapping_add(1);
        if (rde.offset_from(rds) as libc::c_long as libc::c_ulong)
            < ::std::mem::size_of::<hex_record_t>() as libc::c_ulong
        {
            printf(
                b"Error: Parser: Hex: Unexpected end of header! (Line %i)\n\0"
                    as *const u8 as *const libc::c_char,
                line as libc::c_int,
            );
            return 0 as *mut data_t;
        }
        hex = rds as *mut hex_record_t;
        if (*hex).start_code as libc::c_int != ':' as i32 {
            printf(
                b"Error: Parser: Hex: Invalid start code! (Line %i)\n\0" as *const u8
                    as *const libc::c_char,
                line as libc::c_int,
            );
            return 0 as *mut data_t;
        }
        checksum = 0 as libc::c_int as uint8_t;
        byte_count = ascii_to_hex(
            *((*hex).byte_count).as_mut_ptr(),
            *((*hex).byte_count).as_mut_ptr().offset(1 as libc::c_int as isize),
        ) as uint8_t;
        if hex_conv_error != 0 {
            printf(
                b"Error: Parser: Hex: Unexpected ASCII in byte count! (Line %i)\n\0"
                    as *const u8 as *const libc::c_char,
                line as libc::c_int,
            );
            return 0 as *mut data_t;
        }
        checksum = (checksum as libc::c_int + byte_count as libc::c_int) as uint8_t;
        *((*hex).record_type)
            .as_mut_ptr() = ascii_to_hex(
            *((*hex).record_type).as_mut_ptr(),
            *((*hex).record_type).as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        if hex_conv_error != 0 {
            printf(
                b"Error: Parser: Hex: Unexpected ASCII in record type! (Line %i)\n\0"
                    as *const u8 as *const libc::c_char,
                line as libc::c_int,
            );
            return 0 as *mut data_t;
        }
        checksum = (checksum as libc::c_int
            + *((*hex).record_type).as_mut_ptr() as libc::c_int) as uint8_t;
        *((*hex).address.c)
            .as_mut_ptr() = ascii_to_hex(
            *((*hex).address.c).as_mut_ptr().offset(0 as libc::c_int as isize)
                as libc::c_char,
            *((*hex).address.c).as_mut_ptr().offset(1 as libc::c_int as isize)
                as libc::c_char,
        ) as uint8_t;
        if hex_conv_error != 0 {
            printf(
                b"Error: Parser: Hex: Unexpected ASCII in address byte 1! (Line %i)\n\0"
                    as *const u8 as *const libc::c_char,
                line as libc::c_int,
            );
            return 0 as *mut data_t;
        }
        checksum = (checksum as libc::c_int
            + *((*hex).address.c).as_mut_ptr() as libc::c_int) as uint8_t;
        *((*hex).address.c)
            .as_mut_ptr()
            .offset(
                1 as libc::c_int as isize,
            ) = ascii_to_hex(
            *((*hex).address.c).as_mut_ptr().offset(2 as libc::c_int as isize)
                as libc::c_char,
            *((*hex).address.c).as_mut_ptr().offset(3 as libc::c_int as isize)
                as libc::c_char,
        ) as uint8_t;
        if hex_conv_error != 0 {
            printf(
                b"Error: Parser: Hex: Unexpected ASCII in address byte 2! (Line %i)\n\0"
                    as *const u8 as *const libc::c_char,
                line as libc::c_int,
            );
            return 0 as *mut data_t;
        }
        checksum = (checksum as libc::c_int
            + *((*hex).address.c).as_mut_ptr().offset(1 as libc::c_int as isize)
                as libc::c_int) as uint8_t;
        (*hex)
            .address
            .i = (((*((*hex).address.c).as_mut_ptr() as libc::c_int) << 8 as libc::c_int)
            + *((*hex).address.c).as_mut_ptr().offset(1 as libc::c_int as isize)
                as libc::c_int) as uint32_t;
        if (rde.offset_from(rds) as libc::c_long as libc::c_ulong)
            < (::std::mem::size_of::<hex_record_t>() as libc::c_ulong)
                .wrapping_add(
                    (byte_count as libc::c_int * 2 as libc::c_int) as libc::c_ulong,
                )
                .wrapping_add(2 as libc::c_int as libc::c_ulong)
        {
            printf(
                b"Error: Parser: Hex: Malformed data! (Line %i)\n\0" as *const u8
                    as *const libc::c_char,
                line as libc::c_int,
            );
            return 0 as *mut data_t;
        }
        hex_data = rds
            .offset(::std::mem::size_of::<hex_record_t>() as libc::c_ulong as isize);
        start_offset = 0 as libc::c_int as uint32_t;
        while start_offset
            < (byte_count as libc::c_int * 2 as libc::c_int) as libc::c_uint
        {
            *hex_data
                .offset(
                    start_offset.wrapping_div(2 as libc::c_int as libc::c_uint) as isize,
                ) = ascii_to_hex(
                *hex_data.offset(start_offset as isize) as libc::c_char,
                *hex_data.offset(start_offset as isize).offset(1 as libc::c_int as isize)
                    as libc::c_char,
            ) as uint8_t;
            if hex_conv_error != 0 {
                printf(
                    b"Error: Parser: Hex: Unexpected ASCII in data byte! (Line %i)\n\0"
                        as *const u8 as *const libc::c_char,
                    line as libc::c_int,
                );
                return 0 as *mut data_t;
            }
            checksum = (checksum as libc::c_int
                + *hex_data
                    .offset(
                        start_offset.wrapping_div(2 as libc::c_int as libc::c_uint)
                            as isize,
                    ) as libc::c_int) as uint8_t;
            start_offset = (start_offset as libc::c_uint)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
        }
        checksum = (checksum as libc::c_int * -(1 as libc::c_int)) as uint8_t;
        checksum_given = ascii_to_hex(
            *hex_data.offset(start_offset as isize) as libc::c_char,
            *hex_data.offset(start_offset as isize).offset(1 as libc::c_int as isize)
                as libc::c_char,
        ) as uint8_t;
        if hex_conv_error != 0 {
            printf(
                b"Error: Parser: Hex: Unexpected ASCII in checksum byte! (Line %i)\n\0"
                    as *const u8 as *const libc::c_char,
                line as libc::c_int,
            );
            return 0 as *mut data_t;
        }
        if checksum as libc::c_int != checksum_given as libc::c_int {
            printf(
                b"Error: Parser: Hex: Checksum mismatch! (Line %i)\n\0" as *const u8
                    as *const libc::c_char,
                line as libc::c_int,
            );
            return 0 as *mut data_t;
        }
        if *((*hex).record_type).as_mut_ptr() as libc::c_int == 0 as libc::c_int {
            if first_address_set == 0 {
                first_address_set = 1 as libc::c_int as uint8_t;
                next_address = base_address.wrapping_add((*hex).address.i);
            }
            if ((*hex).address.i).wrapping_add(base_address) != next_address {
                printf(
                    b"Error: Parser: Hex: Address not contiguous! (Line %i)\n\0"
                        as *const u8 as *const libc::c_char,
                    line as libc::c_int,
                );
                return 0 as *mut data_t;
            }
            binlength = (binlength as libc::c_uint)
                .wrapping_add(byte_count as libc::c_uint) as uint32_t as uint32_t;
            start_offset = 0 as libc::c_int as uint32_t;
            while start_offset < byte_count as libc::c_uint {
                *bindata = *hex_data.offset(start_offset as isize);
                bindata = bindata.offset(1);
                start_offset = start_offset.wrapping_add(1);
            }
            next_address = base_address
                .wrapping_add(
                    next_address.wrapping_add(byte_count as libc::c_uint)
                        & 0xffff as libc::c_int as libc::c_uint,
                );
        } else {
            if *((*hex).record_type).as_mut_ptr() as libc::c_int == 1 as libc::c_int {
                break;
            }
            if *((*hex).record_type).as_mut_ptr() as libc::c_int == 2 as libc::c_int {
                base_address = ((((*hex_data as libc::c_int) << 8 as libc::c_int)
                    + *hex_data.offset(1 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int) as uint32_t;
                next_address = (next_address as libc::c_uint).wrapping_add(base_address)
                    as uint32_t as uint32_t;
            } else if *((*hex).record_type).as_mut_ptr() as libc::c_int
                    == 3 as libc::c_int
                {
                start_segment_address = (((*hex_data as libc::c_int)
                    << 24 as libc::c_int)
                    + ((*hex_data.offset(1 as libc::c_int as isize) as libc::c_int)
                        << 16 as libc::c_int)
                    + ((*hex_data.offset(2 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int)
                    + *hex_data.offset(3 as libc::c_int as isize) as libc::c_int)
                    as uint32_t;
                start_segment_address_set = 1 as libc::c_int as uint8_t;
            } else if *((*hex).record_type).as_mut_ptr() as libc::c_int
                    == 4 as libc::c_int
                {
                printf(
                    b"Error: Parser: Hex: 32-bit addressing is not supported! (Line %i)\n\0"
                        as *const u8 as *const libc::c_char,
                    line as libc::c_int,
                );
                return 0 as *mut data_t;
            } else if *((*hex).record_type).as_mut_ptr() as libc::c_int
                    == 5 as libc::c_int
                {
                printf(
                    b"Error: Parser: Hex: Start linear address is not supported! (Line %i)\n\0"
                        as *const u8 as *const libc::c_char,
                    line as libc::c_int,
                );
                return 0 as *mut data_t;
            } else {
                printf(
                    b"Error: Parser: Hex: Unknown record type! (Line %i)\n\0"
                        as *const u8 as *const libc::c_char,
                    line as libc::c_int,
                );
                return 0 as *mut data_t;
            }
        }
        rds = rds
            .offset(
                (::std::mem::size_of::<hex_record_t>() as libc::c_ulong)
                    .wrapping_add(
                        (byte_count as libc::c_int * 2 as libc::c_int) as libc::c_ulong,
                    )
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
            );
        while rds < rde
            && (*rds as libc::c_int == '\r' as i32 || *rds as libc::c_int == '\n' as i32)
        {
            rds = rds.offset(1);
        }
    }
    if start_segment_address_set == 0 {
        printf(
            b"Error: Parser: Hex: Missing start segment address!\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut data_t;
    }
    let mut data: *mut data_t = create_data(binlength);
    if !data.is_null() {
        memcpy(
            (*data).data as *mut libc::c_void,
            rawdata as *const libc::c_void,
            binlength as libc::c_ulong,
        );
        (*data).size = binlength;
        (*data).addr = start_segment_address;
    } else {
        printf(
            b"Error: Parser: Hex: Error creating parser storage!\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn get_type_by_ext(mut fname: *mut libc::c_char) -> libc::c_char {
    let mut pext: *mut libc::c_char = fname
        .offset(
            (strlen(fname)).wrapping_sub(4 as libc::c_int as libc::c_ulong) as isize,
        );
    if strcmp(pext, b".hex\0" as *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int as libc::c_char
    } else if strcmp(pext, b".bin\0" as *const u8 as *const libc::c_char) == 0 {
        return 2 as libc::c_int as libc::c_char
    } else {
        return 0 as libc::c_int as libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn load_file(mut fname: *mut libc::c_char) -> *mut data_t {
    if fname.is_null() {
        printf(b"ERROR: Parser: No file given!\n\0" as *const u8 as *const libc::c_char);
        return 0 as *mut data_t;
    }
    if strlen(fname) < 5 as libc::c_int as libc::c_ulong {
        printf(
            b"ERROR: Parser: File name must end in %s or %s!\n\0" as *const u8
                as *const libc::c_char,
            b".hex\0" as *const u8 as *const libc::c_char,
            b".bin\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut data_t;
    }
    let mut ftype: libc::c_char = get_type_by_ext(fname);
    if ftype as libc::c_int == 0 as libc::c_int {
        printf(
            b"ERROR: Parser: File name must end in %s or %s!\n\0" as *const u8
                as *const libc::c_char,
            b".hex\0" as *const u8 as *const libc::c_char,
            b".bin\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut data_t;
    }
    let mut fsize: uint32_t = filesize(fname) as uint32_t;
    if fsize == 0 as libc::c_int as libc::c_uint {
        printf(b"ERROR: Parser: File is empty!\n\0" as *const u8 as *const libc::c_char);
        return 0 as *mut data_t;
    }
    let mut fIn: *mut FILE = fopen(fname, b"rb\0" as *const u8 as *const libc::c_char);
    if fIn.is_null() {
        printf(
            b"ERROR: Parser: Could not open file for read!\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut data_t;
    }
    let mut rawdata: *mut libc::c_char = malloc(fsize as libc::c_ulong)
        as *mut libc::c_char;
    if rawdata.is_null() {
        printf(
            b"ERROR: Parser: Could no allocated file memory buffer!\n\0" as *const u8
                as *const libc::c_char,
        );
        fclose(fIn);
        return 0 as *mut data_t;
    }
    let mut readbytes: uint32_t = fread(
        rawdata as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        fsize as libc::c_ulong,
        fIn,
    ) as uint32_t;
    fclose(fIn);
    if readbytes != fsize {
        printf(
            b"ERROR: Parser: File read size mismatch!\n\0" as *const u8
                as *const libc::c_char,
        );
        free(rawdata as *mut libc::c_void);
        return 0 as *mut data_t;
    }
    let mut ret: *mut data_t = 0 as *mut data_t;
    if ftype as libc::c_int == 1 as libc::c_int {
        ret = parse_hex(rawdata, readbytes);
    } else if ftype as libc::c_int == 2 as libc::c_int {
        ret = parse_bin(rawdata, readbytes);
    } else {
        printf(
            b"ERROR: Parser: Unknown file type!\n\0" as *const u8 as *const libc::c_char,
        );
    }
    free(rawdata as *mut libc::c_void);
    return ret;
}
