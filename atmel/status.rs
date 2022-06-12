/* ----------------------------------------------------------------------------
 *         SAM Software Package License
 * ----------------------------------------------------------------------------
 * Copyright (c) 2011-2012, Atmel Corporation
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following condition is met:
 *
 * Redistributions of source code must retain the above copyright notice,
 * this list of conditions and the disclaimer below.
 *
 * Atmel's name may not be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * DISCLAIMER: THIS SOFTWARE IS PROVIDED BY ATMEL "AS IS" AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT ARE
 * DISCLAIMED. IN NO EVENT SHALL ATMEL BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA,
 * OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
 * EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 * ----------------------------------------------------------------------------
 */

pub type status_code = libc::c_uint;
pub const STATUS_ERR_PIN_MUX_INVALID: status_code = 80;
pub const STATUS_ERR_PROTOCOL: status_code = 66;
pub const STATUS_ERR_PACKET_COLLISION: status_code = 65;
pub const STATUS_ERR_BAUDRATE_UNAVAILABLE: status_code = 64;
pub const STATUS_ERR_RESOLUTION_UNAVAILABLE: status_code = 49;
pub const STATUS_ERR_SAMPLERATE_UNAVAILABLE: status_code = 48;
pub const STATUS_ERR_NOT_INITIALIZED: status_code = 31;
pub const STATUS_ERR_OVERFLOW: status_code = 30;
pub const STATUS_ERR_ALREADY_INITIALIZED: status_code = 29;
pub const STATUS_ERR_DENIED: status_code = 28;
pub const STATUS_ERR_BAD_FRQ: status_code = 27;
pub const STATUS_ERR_BAD_FORMAT: status_code = 26;
pub const STATUS_ERR_BAD_ADDRESS: status_code = 24;
pub const STATUS_ERR_INVALID_ARG: status_code = 23;
pub const STATUS_ERR_NO_MEMORY: status_code = 22;
pub const STATUS_ERR_UNSUPPORTED_DEV: status_code = 21;
pub const STATUS_ERR_NOT_FOUND: status_code = 20;
pub const STATUS_ERR_BAD_DATA: status_code = 19;
pub const STATUS_ERR_TIMEOUT: status_code = 18;
pub const STATUS_ERR_REQ_FLUSHED: status_code = 17;
pub const STATUS_ERR_IO: status_code = 16;
pub const STATUS_SUSPEND: status_code = 6;
pub const STATUS_BUSY: status_code = 5;
pub const STATUS_ABORTED: status_code = 4;
pub const STATUS_NO_CHANGE: status_code = 2;
pub const STATUS_VALID_DATA: status_code = 1;
pub const STATUS_OK: status_code = 0;
