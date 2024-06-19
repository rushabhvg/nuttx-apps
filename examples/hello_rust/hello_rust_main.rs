/****************************************************************************
 * apps/examples/hello_rust/hello_rust_main.rs
 *
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.  The
 * ASF licenses this file to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance with the
 * License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
 * WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.  See the
 * License for the specific language governing permissions and limitations
 * under the License.
 *
 ****************************************************************************/

/****************************************************************************
 * Attributes
 ****************************************************************************/

 #![no_main]
 #![no_std]
 
 /****************************************************************************
  * Uses
  ****************************************************************************/
 
 use core::panic::PanicInfo;
 use core::ffi::{ c_char, c_int, c_void };
 
 /****************************************************************************
  * Externs
  ****************************************************************************/
 
 extern "C" {
     pub fn printf(format: *const u8, ...) -> i32;
     pub fn puts(s: *const c_char) -> c_int;
     pub fn fgets(
         buf: *mut c_char,
         n: c_int,
         stream: *mut c_void
     ) -> *mut c_char;
     pub fn lib_get_stream(fd: c_int) -> *mut c_void;
 
     // Functions for Rust LED Blinky example
     pub fn open(path: *const u8, oflag: i32, ...) -> i32;
     pub fn read(fd: i32, buf: *mut u8, count: u32) -> i32;
     pub fn write(fd: i32, buf: *const u8, count: u32) -> i32;
     pub fn close(fd: i32) -> i32;
     pub fn ioctl(fd: i32, request: i32, ...) -> i32;  //  On NuttX: request is i32, not u64 like Linux
     pub fn sleep(secs: u32) -> u32;
     pub fn usleep(usec: u32) -> u32;
     pub fn exit(status: u32) -> !;
 }
 
 /****************************************************************************
  * Rust Constants
  ****************************************************************************/
 pub const O_WRONLY: i32 = 1 << 1;
 pub const ULEDIOC_SETALL: u16 = 0x1d03;
 
 /****************************************************************************
  * Private functions
  ****************************************************************************/
 
 /****************************************************************************
  * Panic handler (needed for [no_std] compilation)
  ****************************************************************************/
 
 #[panic_handler]
 fn panic(_panic: &PanicInfo<'_>) -> ! {
     loop {}
 }
 
 /****************************************************************************
  * Public functions
  ****************************************************************************/
 
 /****************************************************************************
  * hello_rust_main
  ****************************************************************************/
 
 #[no_mangle]
 pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
     /* "Hello, Rust!!" using printf() from libc */
 
     unsafe {
         printf(b"Hello, Rust!!\n\0" as *const u8);
 
         // Open the LED Driver
         // printf(b"Opening /dev/userleds\n\0");
         let format = b"Opening /dev/userleds\n\0";
         let format_ptr = format.as_ptr();
         printf(format_ptr);
     
         let fd = open(b"/dev/userleds\0" as *const u8, O_WRONLY);
 
         if (fd < 0) {
             let errcode = 0;
             // printf(b"ERROR: Failed to open /dev/userleds: %d\n\0", errcode);
             let format = b"ERROR: Failed to open /dev/userleds: %d\n\0";
             let format_ptr = format.as_ptr();
             printf(format_ptr, errcode);
 
             return 1;   // In stdlib.h: #define EXIT_FAILURE 1
         }
     
         // puts(b"Set LED 0 to 1\0" as *const i8);
         let message = b"Set LED 0 to 1\0";
         let message_ptr = message.as_ptr() as *const i8;
         puts(message_ptr);
         let ret = ioctl(fd, ULEDIOC_SETALL.into(), 1);      // ULEDIOC_SETALL is u16, so we need to convert it to i32
         if (ret < 0) {
             let errcode = 0;
             printf(b"ERROR: ioctl(ULEDIOC_SUPPORTED) failed: %d\n\0" as *const u8, errcode);
             return 1;   // In stdlib.h: #define EXIT_FAILURE 1
         }
 
         // Sleep a while
         // puts(b"Waiting...\0" as *const c_char);
         let message = b"Waiting...\0";
         let message_ptr = message.as_ptr() as *const i8;
         puts(message_ptr);
         // In no_std, we can't use Rust's usleep() function, so we use unsafe instead
         usleep(500 * 1000);    // 500 ms
 
         // Turn off LED
         // puts(b"Set LED 0 to 0\0" as *const u8);
         let message = b"Set LED 0 to 0\0";
         let message_ptr = message.as_ptr() as *const i8;
         puts(message_ptr);
         let ret = ioctl(fd, ULEDIOC_SETALL.into(), 0);
         if (ret < 0) {
             let errcode = 0;
             printf(b"ERROR: ioctl(ULEDIOC_SUPPORTED) failed: %d\n\0" as *const u8, errcode);
             return 1;   // In stdlib.h: #define EXIT_FAILURE 1
         }
 
         // Close the LED Driver
         close(fd);
     }
 
     /* exit with status 0 */
 
     0
 }