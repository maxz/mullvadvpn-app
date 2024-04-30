/* automatically generated by rust-bindgen 0.69.2 */

pub const PTH_FLAG_DIR_OUT: u32 = 2;
pub const PCAP_ERRBUF_SIZE: u32 = 256;
pub type __int32_t = ::std::os::raw::c_int;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_uuid_t = [::std::os::raw::c_uchar; 16usize];
pub type u_int = ::std::os::raw::c_uint;
pub type pid_t = __darwin_pid_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval32 {
    pub tv_sec: __int32_t,
    pub tv_usec: __int32_t,
}
#[test]
fn bindgen_test_layout_timeval32() {
    const UNINIT: ::std::mem::MaybeUninit<timeval32> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<timeval32>(),
        8usize,
        concat!("Size of: ", stringify!(timeval32))
    );
    assert_eq!(
        ::std::mem::align_of::<timeval32>(),
        4usize,
        concat!("Alignment of ", stringify!(timeval32))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tv_sec) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval32),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tv_usec) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval32),
            "::",
            stringify!(tv_usec)
        )
    );
}
pub type uuid_t = __darwin_uuid_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pktap_header {
    pub pth_length: u32,
    pub pth_type_next: u32,
    pub pth_dlt: u32,
    pub pth_ifname: [::std::os::raw::c_char; 24usize],
    pub pth_flags: u32,
    pub pth_protocol_family: u32,
    pub pth_frame_pre_length: u32,
    pub pth_frame_post_length: u32,
    pub pth_pid: pid_t,
    pub pth_comm: [::std::os::raw::c_char; 17usize],
    pub pth_svc: u32,
    pub pth_iftype: u16,
    pub pth_ifunit: u16,
    pub pth_epid: pid_t,
    pub pth_ecomm: [::std::os::raw::c_char; 17usize],
    pub pth_flowid: u32,
    pub pth_ipproto: u32,
    pub pth_tstamp: timeval32,
    pub pth_uuid: uuid_t,
    pub pth_euuid: uuid_t,
}
#[test]
fn bindgen_test_layout_pktap_header() {
    const UNINIT: ::std::mem::MaybeUninit<pktap_header> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<pktap_header>(),
        156usize,
        concat!("Size of: ", stringify!(pktap_header))
    );
    assert_eq!(
        ::std::mem::align_of::<pktap_header>(),
        4usize,
        concat!("Alignment of ", stringify!(pktap_header))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_length) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_type_next) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_type_next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_dlt) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_dlt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_ifname) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_ifname)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_flags) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_protocol_family) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_protocol_family)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_frame_pre_length) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_frame_pre_length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_frame_post_length) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_frame_post_length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_pid) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_pid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_comm) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_comm)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_svc) as usize - ptr as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_svc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_iftype) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_iftype)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_ifunit) as usize - ptr as usize },
        82usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_ifunit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_epid) as usize - ptr as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_epid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_ecomm) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_ecomm)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_flowid) as usize - ptr as usize },
        108usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_flowid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_ipproto) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_ipproto)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_tstamp) as usize - ptr as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_tstamp)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_uuid) as usize - ptr as usize },
        124usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_uuid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pth_euuid) as usize - ptr as usize },
        140usize,
        concat!(
            "Offset of field: ",
            stringify!(pktap_header),
            "::",
            stringify!(pth_euuid)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_stat {
    pub bs_recv: u_int,
    pub bs_drop: u_int,
}
#[test]
fn bindgen_test_layout_bpf_stat() {
    const UNINIT: ::std::mem::MaybeUninit<bpf_stat> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<bpf_stat>(),
        8usize,
        concat!("Size of: ", stringify!(bpf_stat))
    );
    assert_eq!(
        ::std::mem::align_of::<bpf_stat>(),
        4usize,
        concat!("Alignment of ", stringify!(bpf_stat))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bs_recv) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_stat),
            "::",
            stringify!(bs_recv)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bs_drop) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_stat),
            "::",
            stringify!(bs_drop)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcap {
    _unused: [u8; 0],
}
pub type pcap_t = pcap;
extern "C" {
    pub fn pcap_create(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut ::std::os::raw::c_char,
    ) -> *mut pcap_t;
}
extern "C" {
    pub fn pcap_set_want_pktap(
        arg1: *mut pcap_t,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub const BIOCSWANTPKTAP: u64 = 3221504639;
