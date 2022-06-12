/*

Copyright (C) 2012-2015 Atmel Corporation. All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice,
   this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. The name of Atmel may not be used to endorse or promote products derived
   from this software without specific prior written permission.

4. This software may only be redistributed and used in connection with an
   Atmel microcontroller product.

THIS SOFTWARE IS PROVIDED BY ATMEL "AS IS" AND ANY EXPRESS OR IMPLIED
WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT ARE
EXPRESSLY AND SPECIFICALLY DISCLAIMED. IN NO EVENT SHALL ATMEL BE LIABLE FOR
ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.

Support and FAQ: visit http://www.atmel.com/design-support/

*/

type __int8_t = libc::c_schar;
type __uint8_t = libc::c_uchar;
type __uint16_t = libc::c_ushort;
type __uint32_t = libc::c_uint;
type __dev_t = libc::c_ulong;
type __uid_t = libc::c_uint;
type __gid_t = libc::c_uint;
type __ino_t = libc::c_ulong;
type __mode_t = libc::c_uint;
type __nlink_t = libc::c_ulong;
type __off_t = libc::c_long;
type __off64_t = libc::c_long;
type __time_t = libc::c_long;
type __useconds_t = libc::c_uint;
type __blksize_t = libc::c_long;
type __blkcnt_t = libc::c_long;
type __syscall_slong_t = libc::c_long;
type uint8_t = __uint8_t;
type uint16_t = __uint16_t;
type uint32_t = __uint32_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub inputInit: C2RustUnnamed_15,
    pub outputInit: C2RustUnnamed_13,
    pub inputWrite: C2RustUnnamed_10,
    pub outputWrite: C2RustUnnamed_11,
    pub inputRead: C2RustUnnamed_10,
    pub outputRead: C2RustUnnamed_9,
    pub inputLock: C2RustUnnamed_8,
    pub inputUnlock: C2RustUnnamed_7,
    pub outputReadLocks: C2RustUnnamed_6,
    pub outputReadFuses: C2RustUnnamed_5,
    pub inputReadUniqueID: C2RustUnnamed_4,
    pub inputSecurity: C2RustUnnamed_3,
    pub inputEraseRow: C2RustUnnamed_2,
    pub inputEraseApp: C2RustUnnamed_1,
    pub outputInfo: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub version_number: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub start_row: uint32_t,
    pub end_row: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub row: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub action: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub bufferAddr: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub bufferAddr: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub bufferAddr: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub row: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub row: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub bytesRead: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub bufferAddr: uint32_t,
    pub bufferSize: uint32_t,
    pub memoryOffset: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub bytesWritten: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub memorySize: uint32_t,
    pub bufferAddress: uint32_t,
    pub bufferSize: uint32_t,
    pub memoryInfo: C2RustUnnamed_14,
    pub pageSize: uint32_t,
    pub nbPages: uint32_t,
    pub appStartPage: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub lockRegionSize: uint16_t,
    pub numbersLockBits: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub comType: uint32_t,
    pub traceLevel: uint32_t,
    pub bank: uint32_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailbox_s {
    pub command: uint32_t,
    pub status: uint32_t,
    pub argument: C2RustUnnamed,
}

pub type mailbox_t = mailbox_s;