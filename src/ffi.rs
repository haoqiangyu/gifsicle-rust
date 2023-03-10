/* bindgen vendor/include/lcdfgif/gif.h --size_t-is-usize --whitelist-type='[gG]if.*' --whitelist-function='[gG]if.*' --no-layout-tests --opaque-type=FILE -- -Ivendor/include  */

use libc::FILE;
pub type Gif_Code = u16;
#[doc = " GIF_STREAM"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gif_Stream {
    pub images: *mut *mut Gif_Image,
    pub nimages: ::std::os::raw::c_int,
    pub imagescap: ::std::os::raw::c_int,
    pub global: *mut Gif_Colormap,
    pub background: u16,
    pub screen_width: u16,
    pub screen_height: u16,
    pub loopcount: ::std::os::raw::c_long,
    pub end_comment: *mut Gif_Comment,
    pub end_extension_list: *mut Gif_Extension,
    pub errors: ::std::os::raw::c_uint,
    pub user_flags: u32,
    pub landmark: *const ::std::os::raw::c_char,
    pub refcount: ::std::os::raw::c_int,
}
extern "C" {
    pub fn Gif_NewStream() -> *mut Gif_Stream;
}
extern "C" {
    pub fn Gif_DeleteStream(arg1: *mut Gif_Stream);
}
extern "C" {
    pub fn Gif_CopyStreamSkeleton(arg1: *mut Gif_Stream) -> *mut Gif_Stream;
}
extern "C" {
    pub fn Gif_CopyStreamImages(arg1: *mut Gif_Stream) -> *mut Gif_Stream;
}
extern "C" {
    pub fn Gif_CalculateScreenSize(arg1: *mut Gif_Stream, force: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Gif_Unoptimize(arg1: *mut Gif_Stream) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Gif_FullUnoptimize(arg1: *mut Gif_Stream, flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
#[doc = " GIF_IMAGE"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gif_Image {
    pub img: *mut *mut u8,
    pub image_data: *mut u8,
    pub width: u16,
    pub height: u16,
    pub left: u16,
    pub top: u16,
    pub delay: u16,
    pub disposal: u8,
    pub interlace: u8,
    pub transparent: ::std::os::raw::c_short,
    pub local: *mut Gif_Colormap,
    pub identifier: *mut ::std::os::raw::c_char,
    pub comment: *mut Gif_Comment,
    pub extension_list: *mut Gif_Extension,
    pub free_image_data: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub compressed_len: u32,
    pub compressed_errors: u32,
    pub compressed: *mut u8,
    pub free_compressed: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub user_flags: u32,
    pub user_data: *mut ::std::os::raw::c_void,
    pub free_user_data: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub refcount: ::std::os::raw::c_int,
}
extern "C" {
    pub fn Gif_NewImage() -> *mut Gif_Image;
}
extern "C" {
    pub fn Gif_DeleteImage(gfi: *mut Gif_Image);
}
extern "C" {
    pub fn Gif_AddImage(gfs: *mut Gif_Stream, gfi: *mut Gif_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Gif_RemoveImage(gfs: *mut Gif_Stream, i: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Gif_CopyImage(gfi: *mut Gif_Image) -> *mut Gif_Image;
}
extern "C" {
    pub fn Gif_MakeImageEmpty(gfi: *mut Gif_Image);
}
extern "C" {
    pub fn Gif_GetImage(gfs: *mut Gif_Stream, i: ::std::os::raw::c_int) -> *mut Gif_Image;
}
extern "C" {
    pub fn Gif_GetNamedImage(gfs: *mut Gif_Stream, name: *const ::std::os::raw::c_char) -> *mut Gif_Image;
}
extern "C" {
    pub fn Gif_ImageNumber(gfs: *mut Gif_Stream, gfi: *mut Gif_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Gif_ImageColorBound(gfi: *const Gif_Image) -> ::std::os::raw::c_int;
}
pub type Gif_ReadErrorHandler = ::std::option::Option<unsafe extern "C" fn(gfs: *mut Gif_Stream, gfi: *mut Gif_Image, is_error: ::std::os::raw::c_int, error_text: *const ::std::os::raw::c_char)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gif_CompressInfo {
    pub flags: ::std::os::raw::c_int,
    pub loss: ::std::os::raw::c_int,
    pub padding: [*mut ::std::os::raw::c_void; 7usize],
}
extern "C" {
    pub fn Gif_FullUncompressImage(gfs: *mut Gif_Stream, gfi: *mut Gif_Image, handler: Gif_ReadErrorHandler) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Gif_CompressImage(gfs: *mut Gif_Stream, gfi: *mut Gif_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Gif_FullCompressImage(gfs: *mut Gif_Stream, gfi: *mut Gif_Image, gcinfo: *const Gif_CompressInfo) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Gif_ReleaseUncompressedImage(gfi: *mut Gif_Image);
}
extern "C" {
    pub fn Gif_ReleaseCompressedImage(gfi: *mut Gif_Image);
}
extern "C" {
    pub fn Gif_SetUncompressedImage(
        gfi: *mut Gif_Image,
        data: *mut u8,
        free_data: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        data_interlaced: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Gif_CreateUncompressedImage(gfi: *mut Gif_Image, data_interlaced: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Gif_ClipImage(gfi: *mut Gif_Image, l: ::std::os::raw::c_int, t: ::std::os::raw::c_int, w: ::std::os::raw::c_int, h: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Gif_InitCompressInfo(gcinfo: *mut Gif_CompressInfo);
}
#[doc = " GIF_COLORMAP"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gif_Color {
    pub haspixel: u8,
    pub gfc_red: u8,
    pub gfc_green: u8,
    pub gfc_blue: u8,
    pub pixel: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gif_Colormap {
    pub ncol: ::std::os::raw::c_int,
    pub capacity: ::std::os::raw::c_int,
    pub user_flags: u32,
    pub refcount: ::std::os::raw::c_int,
    pub col: *mut Gif_Color,
}
extern "C" {
    pub fn Gif_NewColormap() -> *mut Gif_Colormap;
}
extern "C" {
    pub fn Gif_NewFullColormap(count: ::std::os::raw::c_int, capacity: ::std::os::raw::c_int) -> *mut Gif_Colormap;
}
extern "C" {
    pub fn Gif_DeleteColormap(arg1: *mut Gif_Colormap);
}
extern "C" {
    pub fn Gif_CopyColormap(arg1: *mut Gif_Colormap) -> *mut Gif_Colormap;
}
extern "C" {
    pub fn Gif_ColorEq(arg1: *mut Gif_Color, arg2: *mut Gif_Color) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Gif_FindColor(arg1: *mut Gif_Colormap, arg2: *mut Gif_Color) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Gif_AddColor(arg1: *mut Gif_Colormap, arg2: *mut Gif_Color, look_from: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
#[doc = " GIF_COMMENT"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gif_Comment {
    pub str_: *mut *mut ::std::os::raw::c_char,
    pub len: *mut ::std::os::raw::c_int,
    pub count: ::std::os::raw::c_int,
    pub cap: ::std::os::raw::c_int,
}
extern "C" {
    pub fn Gif_NewComment() -> *mut Gif_Comment;
}
extern "C" {
    pub fn Gif_DeleteComment(arg1: *mut Gif_Comment);
}
extern "C" {
    pub fn Gif_AddCommentTake(arg1: *mut Gif_Comment, arg2: *mut ::std::os::raw::c_char, arg3: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Gif_AddComment(arg1: *mut Gif_Comment, arg2: *const ::std::os::raw::c_char, arg3: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
#[doc = " GIF_EXTENSION"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gif_Extension {
    pub kind: ::std::os::raw::c_int,
    pub appname: *mut ::std::os::raw::c_char,
    pub applength: ::std::os::raw::c_int,
    pub data: *mut u8,
    pub length: u32,
    pub packetized: ::std::os::raw::c_int,
    pub stream: *mut Gif_Stream,
    pub image: *mut Gif_Image,
    pub next: *mut Gif_Extension,
    pub free_data: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
}
extern "C" {
    pub fn Gif_NewExtension(kind: ::std::os::raw::c_int, appname: *const ::std::os::raw::c_char, applength: ::std::os::raw::c_int) -> *mut Gif_Extension;
}
extern "C" {
    pub fn Gif_DeleteExtension(gfex: *mut Gif_Extension);
}
extern "C" {
    pub fn Gif_CopyExtension(gfex: *mut Gif_Extension) -> *mut Gif_Extension;
}
extern "C" {
    pub fn Gif_AddExtension(gfs: *mut Gif_Stream, gfi: *mut Gif_Image, gfex: *mut Gif_Extension) -> ::std::os::raw::c_int;
}
#[doc = " READING AND WRITING"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gif_Record {
    pub data: *const ::std::os::raw::c_uchar,
    pub length: u32,
}
extern "C" {
    pub fn Gif_SetErrorHandler(handler: Gif_ReadErrorHandler);
}
extern "C" {
    pub fn Gif_ReadFile(f: *mut FILE) -> *mut Gif_Stream;
}
extern "C" {
    pub fn Gif_FullReadFile(f: *mut FILE, flags: ::std::os::raw::c_int, landmark: *const ::std::os::raw::c_char, handler: Gif_ReadErrorHandler) -> *mut Gif_Stream;
}
extern "C" {
    pub fn Gif_ReadRecord(record: *const Gif_Record) -> *mut Gif_Stream;
}
extern "C" {
    pub fn Gif_FullReadRecord(record: *const Gif_Record, flags: ::std::os::raw::c_int, landmark: *const ::std::os::raw::c_char, handler: Gif_ReadErrorHandler) -> *mut Gif_Stream;
}
extern "C" {
    pub fn Gif_WriteFile(gfs: *mut Gif_Stream, f: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Gif_FullWriteFile(gfs: *mut Gif_Stream, gcinfo: *const Gif_CompressInfo, f: *mut FILE) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gif_Writer {
    pub f: *mut ::std::os::raw::c_void,
    pub v: *mut u8,
    pub pos: u32,
    pub cap: u32,
    pub gcinfo: Gif_CompressInfo,
    pub global_size: ::std::os::raw::c_int,
    pub local_size: ::std::os::raw::c_int,
    pub errors: ::std::os::raw::c_int,
    pub cleared: ::std::os::raw::c_int,
    pub private: [::std::os::raw::c_ulong; 16usize],
}
extern "C" {
    pub fn Gif_IncrementalWriteFileInit(gfs: *mut Gif_Stream, gcinfo: *const Gif_CompressInfo, f: *mut FILE) -> *mut Gif_Writer;
}
extern "C" {
    pub fn Gif_IncrementalWriteImage(grr: *mut Gif_Writer, gfs: *mut Gif_Stream, gfi: *mut Gif_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Gif_IncrementalWriteComplete(grr: *mut Gif_Writer, gfs: *mut Gif_Stream) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " HOOKS AND MISCELLANEOUS"]
    pub fn Gif_InterlaceLine(y: ::std::os::raw::c_int, height: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Gif_CopyString(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
pub type Gif_DeletionHookFunc = ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int, arg2: *mut ::std::os::raw::c_void, arg3: *mut ::std::os::raw::c_void)>;
extern "C" {
    pub fn Gif_AddDeletionHook(arg1: ::std::os::raw::c_int, arg2: Gif_DeletionHookFunc, arg3: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Gif_RemoveDeletionHook(arg1: ::std::os::raw::c_int, arg2: Gif_DeletionHookFunc, arg3: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Gif_Realloc(p: *mut ::std::os::raw::c_void, s: usize, n: usize, file: *const ::std::os::raw::c_char, line: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Gif_Free(p: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub static mut gif_write_info: Gif_CompressInfo;
}
