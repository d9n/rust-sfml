/* automatically generated by rust-bindgen */

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

// -- Manual additions --
pub const sfFalse: sfBool = 0;
pub const sfTrue: sfBool = 1;
// -- End of manual additions --

pub type sfBool = ::std::os::raw::c_int;
pub type sfInt8 = ::std::os::raw::c_char;
pub type sfUint8 = ::std::os::raw::c_uchar;
pub type sfInt16 = ::std::os::raw::c_short;
pub type sfUint16 = ::std::os::raw::c_ushort;
pub type sfInt32 = ::std::os::raw::c_int;
pub type sfUint32 = ::std::os::raw::c_uint;
pub type sfInt64 = ::std::os::raw::c_longlong;
pub type sfUint64 = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfTime {
    pub microseconds: sfInt64,
}
impl ::std::default::Default for sfTime {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum sfClock { }
pub enum sfMutex { }
pub enum sfThread { }
pub type sfInputStreamReadFunc =
    ::std::option::Option<unsafe extern "C" fn(data:
                                                   *mut ::std::os::raw::c_void,
                                               size: sfInt64,
                                               userData:
                                                   *mut ::std::os::raw::c_void)
                              -> sfInt64>;
pub type sfInputStreamSeekFunc =
    ::std::option::Option<unsafe extern "C" fn(position: sfInt64,
                                               userData:
                                                   *mut ::std::os::raw::c_void)
                              -> sfInt64>;
pub type sfInputStreamTellFunc =
    ::std::option::Option<unsafe extern "C" fn(userData:
                                                   *mut ::std::os::raw::c_void)
                              -> sfInt64>;
pub type sfInputStreamGetSizeFunc =
    ::std::option::Option<unsafe extern "C" fn(userData:
                                                   *mut ::std::os::raw::c_void)
                              -> sfInt64>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfInputStream {
    pub read: sfInputStreamReadFunc,
    pub seek: sfInputStreamSeekFunc,
    pub tell: sfInputStreamTellFunc,
    pub getSize: sfInputStreamGetSizeFunc,
    pub userData: *mut ::std::os::raw::c_void,
}
impl ::std::default::Default for sfInputStream {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfVector2i {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
impl ::std::default::Default for sfVector2i {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfVector2u {
    pub x: ::std::os::raw::c_uint,
    pub y: ::std::os::raw::c_uint,
}
impl ::std::default::Default for sfVector2u {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct sfVector2f {
    pub x: ::std::os::raw::c_float,
    pub y: ::std::os::raw::c_float,
}
impl ::std::default::Default for sfVector2f {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfVector3f {
    pub x: ::std::os::raw::c_float,
    pub y: ::std::os::raw::c_float,
    pub z: ::std::os::raw::c_float,
}
impl ::std::default::Default for sfVector3f {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
extern "C" {
    pub static mut sfTime_Zero: sfTime;
}
extern "C" {
    pub fn sfTime_asSeconds(time: sfTime) -> ::std::os::raw::c_float;
    pub fn sfTime_asMilliseconds(time: sfTime) -> sfInt32;
    pub fn sfTime_asMicroseconds(time: sfTime) -> sfInt64;
    pub fn sfSeconds(amount: ::std::os::raw::c_float) -> sfTime;
    pub fn sfMilliseconds(amount: sfInt32) -> sfTime;
    pub fn sfMicroseconds(amount: sfInt64) -> sfTime;
    pub fn sfClock_create() -> *mut sfClock;
    pub fn sfClock_copy(clock: *const sfClock) -> *mut sfClock;
    pub fn sfClock_destroy(clock: *mut sfClock);
    pub fn sfClock_getElapsedTime(clock: *const sfClock) -> sfTime;
    pub fn sfClock_restart(clock: *mut sfClock) -> sfTime;
    pub fn sfMutex_create() -> *mut sfMutex;
    pub fn sfMutex_destroy(mutex: *mut sfMutex);
    pub fn sfMutex_lock(mutex: *mut sfMutex);
    pub fn sfMutex_unlock(mutex: *mut sfMutex);
    pub fn sfSleep(duration: sfTime);
    pub fn sfThread_create(function:
                               ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                              *mut ::std::os::raw::c_void)>,
                           userData: *mut ::std::os::raw::c_void)
     -> *mut sfThread;
    pub fn sfThread_destroy(thread: *mut sfThread);
    pub fn sfThread_launch(thread: *mut sfThread);
    pub fn sfThread_wait(thread: *mut sfThread);
    pub fn sfThread_terminate(thread: *mut sfThread);
}
