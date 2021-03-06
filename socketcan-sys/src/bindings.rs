/* automatically generated by rust-bindgen 0.56.0 */

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub const fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
pub const CAN_EFF_FLAG: u32 = 2147483648;
pub const CAN_RTR_FLAG: u32 = 1073741824;
pub const CAN_ERR_FLAG: u32 = 536870912;
pub const CAN_SFF_MASK: u32 = 2047;
pub const CAN_EFF_MASK: u32 = 536870911;
pub const CAN_ERR_MASK: u32 = 536870911;
pub const CAN_SFF_ID_BITS: u32 = 11;
pub const CAN_EFF_ID_BITS: u32 = 29;
pub const CAN_MAX_DLC: u32 = 8;
pub const CAN_MAX_DLEN: u32 = 8;
pub const CANFD_MAX_DLC: u32 = 15;
pub const CANFD_MAX_DLEN: u32 = 64;
pub const CANFD_BRS: u32 = 1;
pub const CANFD_ESI: u32 = 2;
pub const CAN_RAW: u32 = 1;
pub const CAN_BCM: u32 = 2;
pub const CAN_TP16: u32 = 3;
pub const CAN_TP20: u32 = 4;
pub const CAN_MCNET: u32 = 5;
pub const CAN_ISOTP: u32 = 6;
pub const CAN_J1939: u32 = 7;
pub const CAN_NPROTO: u32 = 8;
pub const SOL_CAN_BASE: u32 = 100;
pub const CAN_INV_FILTER: u32 = 536870912;
pub const CAN_RAW_FILTER_MAX: u32 = 512;
pub const SETTIMER: u32 = 1;
pub const STARTTIMER: u32 = 2;
pub const TX_COUNTEVT: u32 = 4;
pub const TX_ANNOUNCE: u32 = 8;
pub const TX_CP_CAN_ID: u32 = 16;
pub const RX_FILTER_ID: u32 = 32;
pub const RX_CHECK_DLC: u32 = 64;
pub const RX_NO_AUTOTIMER: u32 = 128;
pub const RX_ANNOUNCE_RESUME: u32 = 256;
pub const TX_RESET_MULTI_IDX: u32 = 512;
pub const RX_RTR_FRAME: u32 = 1024;
pub const CAN_FD_FRAME: u32 = 2048;
pub const CAN_ERR_DLC: u32 = 8;
pub const CAN_ERR_TX_TIMEOUT: u32 = 1;
pub const CAN_ERR_LOSTARB: u32 = 2;
pub const CAN_ERR_CRTL: u32 = 4;
pub const CAN_ERR_PROT: u32 = 8;
pub const CAN_ERR_TRX: u32 = 16;
pub const CAN_ERR_ACK: u32 = 32;
pub const CAN_ERR_BUSOFF: u32 = 64;
pub const CAN_ERR_BUSERROR: u32 = 128;
pub const CAN_ERR_RESTARTED: u32 = 256;
pub const CAN_ERR_LOSTARB_UNSPEC: u32 = 0;
pub const CAN_ERR_CRTL_UNSPEC: u32 = 0;
pub const CAN_ERR_CRTL_RX_OVERFLOW: u32 = 1;
pub const CAN_ERR_CRTL_TX_OVERFLOW: u32 = 2;
pub const CAN_ERR_CRTL_RX_WARNING: u32 = 4;
pub const CAN_ERR_CRTL_TX_WARNING: u32 = 8;
pub const CAN_ERR_CRTL_RX_PASSIVE: u32 = 16;
pub const CAN_ERR_CRTL_TX_PASSIVE: u32 = 32;
pub const CAN_ERR_CRTL_ACTIVE: u32 = 64;
pub const CAN_ERR_PROT_UNSPEC: u32 = 0;
pub const CAN_ERR_PROT_BIT: u32 = 1;
pub const CAN_ERR_PROT_FORM: u32 = 2;
pub const CAN_ERR_PROT_STUFF: u32 = 4;
pub const CAN_ERR_PROT_BIT0: u32 = 8;
pub const CAN_ERR_PROT_BIT1: u32 = 16;
pub const CAN_ERR_PROT_OVERLOAD: u32 = 32;
pub const CAN_ERR_PROT_ACTIVE: u32 = 64;
pub const CAN_ERR_PROT_TX: u32 = 128;
pub const CAN_ERR_PROT_LOC_UNSPEC: u32 = 0;
pub const CAN_ERR_PROT_LOC_SOF: u32 = 3;
pub const CAN_ERR_PROT_LOC_ID28_21: u32 = 2;
pub const CAN_ERR_PROT_LOC_ID20_18: u32 = 6;
pub const CAN_ERR_PROT_LOC_SRTR: u32 = 4;
pub const CAN_ERR_PROT_LOC_IDE: u32 = 5;
pub const CAN_ERR_PROT_LOC_ID17_13: u32 = 7;
pub const CAN_ERR_PROT_LOC_ID12_05: u32 = 15;
pub const CAN_ERR_PROT_LOC_ID04_00: u32 = 14;
pub const CAN_ERR_PROT_LOC_RTR: u32 = 12;
pub const CAN_ERR_PROT_LOC_RES1: u32 = 13;
pub const CAN_ERR_PROT_LOC_RES0: u32 = 9;
pub const CAN_ERR_PROT_LOC_DLC: u32 = 11;
pub const CAN_ERR_PROT_LOC_DATA: u32 = 10;
pub const CAN_ERR_PROT_LOC_CRC_SEQ: u32 = 8;
pub const CAN_ERR_PROT_LOC_CRC_DEL: u32 = 24;
pub const CAN_ERR_PROT_LOC_ACK: u32 = 25;
pub const CAN_ERR_PROT_LOC_ACK_DEL: u32 = 27;
pub const CAN_ERR_PROT_LOC_EOF: u32 = 26;
pub const CAN_ERR_PROT_LOC_INTERM: u32 = 18;
pub const CAN_ERR_TRX_UNSPEC: u32 = 0;
pub const CAN_ERR_TRX_CANH_NO_WIRE: u32 = 4;
pub const CAN_ERR_TRX_CANH_SHORT_TO_BAT: u32 = 5;
pub const CAN_ERR_TRX_CANH_SHORT_TO_VCC: u32 = 6;
pub const CAN_ERR_TRX_CANH_SHORT_TO_GND: u32 = 7;
pub const CAN_ERR_TRX_CANL_NO_WIRE: u32 = 64;
pub const CAN_ERR_TRX_CANL_SHORT_TO_BAT: u32 = 80;
pub const CAN_ERR_TRX_CANL_SHORT_TO_VCC: u32 = 96;
pub const CAN_ERR_TRX_CANL_SHORT_TO_GND: u32 = 112;
pub const CAN_ERR_TRX_CANL_SHORT_TO_CANH: u32 = 128;
pub const J1939_MAX_UNICAST_ADDR: u32 = 253;
pub const J1939_IDLE_ADDR: u32 = 254;
pub const J1939_NO_ADDR: u32 = 255;
pub const J1939_NO_NAME: u32 = 0;
pub const J1939_PGN_REQUEST: u32 = 59904;
pub const J1939_PGN_ADDRESS_CLAIMED: u32 = 60928;
pub const J1939_PGN_ADDRESS_COMMANDED: u32 = 65240;
pub const J1939_PGN_PDU1_MAX: u32 = 261888;
pub const J1939_PGN_MAX: u32 = 262143;
pub const J1939_NO_PGN: u32 = 262144;
pub const SOL_CAN_J1939: u32 = 107;
pub const J1939_FILTER_MAX: u32 = 512;
pub const CAN_CTRLMODE_LOOPBACK: u32 = 1;
pub const CAN_CTRLMODE_LISTENONLY: u32 = 2;
pub const CAN_CTRLMODE_3_SAMPLES: u32 = 4;
pub const CAN_CTRLMODE_ONE_SHOT: u32 = 8;
pub const CAN_CTRLMODE_BERR_REPORTING: u32 = 16;
pub const CAN_CTRLMODE_FD: u32 = 32;
pub const CAN_CTRLMODE_PRESUME_ACK: u32 = 64;
pub const CAN_CTRLMODE_FD_NON_ISO: u32 = 128;
pub const CAN_TERMINATION_DISABLED: u32 = 0;
pub const SOL_CAN_RAW: u32 = 101;
pub type __kernel_sa_family_t = ::std::os::raw::c_ushort;
pub type canid_t = __u32;
pub type can_err_mask_t = __u32;
#[repr(C)]
#[repr(align(8))]
pub struct can_frame {
    pub can_id: canid_t,
    pub can_dlc: __u8,
    pub __pad: __u8,
    pub __res0: __u8,
    pub __res1: __u8,
    pub data: [__u8; 8usize],
}
#[test]
fn bindgen_test_layout_can_frame() {
    assert_eq!(
        ::std::mem::size_of::<can_frame>(),
        16usize,
        concat!("Size of: ", stringify!(can_frame))
    );
    assert_eq!(
        ::std::mem::align_of::<can_frame>(),
        8usize,
        concat!("Alignment of ", stringify!(can_frame))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_frame>())).can_id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(can_frame),
            "::",
            stringify!(can_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_frame>())).can_dlc as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(can_frame),
            "::",
            stringify!(can_dlc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_frame>())).__pad as *const _ as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(can_frame),
            "::",
            stringify!(__pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_frame>())).__res0 as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(can_frame),
            "::",
            stringify!(__res0)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_frame>())).__res1 as *const _ as usize },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(can_frame),
            "::",
            stringify!(__res1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_frame>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(can_frame),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
#[repr(align(8))]
pub struct canfd_frame {
    pub can_id: canid_t,
    pub len: __u8,
    pub flags: __u8,
    pub __res0: __u8,
    pub __res1: __u8,
    pub data: [__u8; 64usize],
}
#[test]
fn bindgen_test_layout_canfd_frame() {
    assert_eq!(
        ::std::mem::size_of::<canfd_frame>(),
        72usize,
        concat!("Size of: ", stringify!(canfd_frame))
    );
    assert_eq!(
        ::std::mem::align_of::<canfd_frame>(),
        8usize,
        concat!("Alignment of ", stringify!(canfd_frame))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<canfd_frame>())).can_id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(canfd_frame),
            "::",
            stringify!(can_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<canfd_frame>())).len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(canfd_frame),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<canfd_frame>())).flags as *const _ as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(canfd_frame),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<canfd_frame>())).__res0 as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(canfd_frame),
            "::",
            stringify!(__res0)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<canfd_frame>())).__res1 as *const _ as usize },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(canfd_frame),
            "::",
            stringify!(__res1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<canfd_frame>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(canfd_frame),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
pub struct sockaddr_can {
    pub can_family: __kernel_sa_family_t,
    pub can_ifindex: ::std::os::raw::c_int,
    pub can_addr: sockaddr_can__bindgen_ty_1,
}
#[repr(C)]
pub struct sockaddr_can__bindgen_ty_1 {
    pub tp: __BindgenUnionField<sockaddr_can__bindgen_ty_1__bindgen_ty_1>,
    pub j1939: __BindgenUnionField<sockaddr_can__bindgen_ty_1__bindgen_ty_2>,
    pub bindgen_union_field: [u64; 2usize],
}
#[repr(C)]
pub struct sockaddr_can__bindgen_ty_1__bindgen_ty_1 {
    pub rx_id: canid_t,
    pub tx_id: canid_t,
}
#[test]
fn bindgen_test_layout_sockaddr_can__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<sockaddr_can__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(sockaddr_can__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<sockaddr_can__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(sockaddr_can__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<sockaddr_can__bindgen_ty_1__bindgen_ty_1>())).rx_id as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_can__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(rx_id)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<sockaddr_can__bindgen_ty_1__bindgen_ty_1>())).tx_id as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_can__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(tx_id)
        )
    );
}
#[repr(C)]
pub struct sockaddr_can__bindgen_ty_1__bindgen_ty_2 {
    pub name: __u64,
    pub pgn: __u32,
    pub addr: __u8,
}
#[test]
fn bindgen_test_layout_sockaddr_can__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<sockaddr_can__bindgen_ty_1__bindgen_ty_2>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(sockaddr_can__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<sockaddr_can__bindgen_ty_1__bindgen_ty_2>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(sockaddr_can__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<sockaddr_can__bindgen_ty_1__bindgen_ty_2>())).name as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_can__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<sockaddr_can__bindgen_ty_1__bindgen_ty_2>())).pgn as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_can__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(pgn)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<sockaddr_can__bindgen_ty_1__bindgen_ty_2>())).addr as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_can__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(addr)
        )
    );
}
#[test]
fn bindgen_test_layout_sockaddr_can__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<sockaddr_can__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(sockaddr_can__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<sockaddr_can__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(sockaddr_can__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_can__bindgen_ty_1>())).tp as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_can__bindgen_ty_1),
            "::",
            stringify!(tp)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<sockaddr_can__bindgen_ty_1>())).j1939 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_can__bindgen_ty_1),
            "::",
            stringify!(j1939)
        )
    );
}
#[test]
fn bindgen_test_layout_sockaddr_can() {
    assert_eq!(
        ::std::mem::size_of::<sockaddr_can>(),
        24usize,
        concat!("Size of: ", stringify!(sockaddr_can))
    );
    assert_eq!(
        ::std::mem::align_of::<sockaddr_can>(),
        8usize,
        concat!("Alignment of ", stringify!(sockaddr_can))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_can>())).can_family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_can),
            "::",
            stringify!(can_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_can>())).can_ifindex as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_can),
            "::",
            stringify!(can_ifindex)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_can>())).can_addr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_can),
            "::",
            stringify!(can_addr)
        )
    );
}
#[repr(C)]
pub struct can_filter {
    pub can_id: canid_t,
    pub can_mask: canid_t,
}
#[test]
fn bindgen_test_layout_can_filter() {
    assert_eq!(
        ::std::mem::size_of::<can_filter>(),
        8usize,
        concat!("Size of: ", stringify!(can_filter))
    );
    assert_eq!(
        ::std::mem::align_of::<can_filter>(),
        4usize,
        concat!("Alignment of ", stringify!(can_filter))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_filter>())).can_id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(can_filter),
            "::",
            stringify!(can_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_filter>())).can_mask as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(can_filter),
            "::",
            stringify!(can_mask)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bcm_timeval {
    pub tv_sec: ::std::os::raw::c_long,
    pub tv_usec: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_bcm_timeval() {
    assert_eq!(
        ::std::mem::size_of::<bcm_timeval>(),
        16usize,
        concat!("Size of: ", stringify!(bcm_timeval))
    );
    assert_eq!(
        ::std::mem::align_of::<bcm_timeval>(),
        8usize,
        concat!("Alignment of ", stringify!(bcm_timeval))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bcm_timeval>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bcm_timeval),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bcm_timeval>())).tv_usec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bcm_timeval),
            "::",
            stringify!(tv_usec)
        )
    );
}
#[repr(C)]
pub struct bcm_msg_head {
    pub opcode: __u32,
    pub flags: __u32,
    pub count: __u32,
    pub ival1: bcm_timeval,
    pub ival2: bcm_timeval,
    pub can_id: canid_t,
    pub nframes: __u32,
    pub frames: __IncompleteArrayField<can_frame>,
}
#[test]
fn bindgen_test_layout_bcm_msg_head() {
    assert_eq!(
        ::std::mem::size_of::<bcm_msg_head>(),
        56usize,
        concat!("Size of: ", stringify!(bcm_msg_head))
    );
    assert_eq!(
        ::std::mem::align_of::<bcm_msg_head>(),
        8usize,
        concat!("Alignment of ", stringify!(bcm_msg_head))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bcm_msg_head>())).opcode as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bcm_msg_head),
            "::",
            stringify!(opcode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bcm_msg_head>())).flags as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bcm_msg_head),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bcm_msg_head>())).count as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bcm_msg_head),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bcm_msg_head>())).ival1 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(bcm_msg_head),
            "::",
            stringify!(ival1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bcm_msg_head>())).ival2 as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(bcm_msg_head),
            "::",
            stringify!(ival2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bcm_msg_head>())).can_id as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(bcm_msg_head),
            "::",
            stringify!(can_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bcm_msg_head>())).nframes as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(bcm_msg_head),
            "::",
            stringify!(nframes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bcm_msg_head>())).frames as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(bcm_msg_head),
            "::",
            stringify!(frames)
        )
    );
}
pub const TX_SETUP: ::std::os::raw::c_uint = 1;
pub const TX_DELETE: ::std::os::raw::c_uint = 2;
pub const TX_READ: ::std::os::raw::c_uint = 3;
pub const TX_SEND: ::std::os::raw::c_uint = 4;
pub const RX_SETUP: ::std::os::raw::c_uint = 5;
pub const RX_DELETE: ::std::os::raw::c_uint = 6;
pub const RX_READ: ::std::os::raw::c_uint = 7;
pub const TX_STATUS: ::std::os::raw::c_uint = 8;
pub const TX_EXPIRED: ::std::os::raw::c_uint = 9;
pub const RX_STATUS: ::std::os::raw::c_uint = 10;
pub const RX_TIMEOUT: ::std::os::raw::c_uint = 11;
pub const RX_CHANGED: ::std::os::raw::c_uint = 12;
pub type _bindgen_ty_1 = ::std::os::raw::c_uint;
pub type pgn_t = __u32;
pub type priority_t = __u8;
pub type name_t = __u64;
pub const SO_J1939_FILTER: ::std::os::raw::c_uint = 1;
pub const SO_J1939_PROMISC: ::std::os::raw::c_uint = 2;
pub const SO_J1939_SEND_PRIO: ::std::os::raw::c_uint = 3;
pub const SO_J1939_ERRQUEUE: ::std::os::raw::c_uint = 4;
pub type _bindgen_ty_2 = ::std::os::raw::c_uint;
pub const SCM_J1939_DEST_ADDR: ::std::os::raw::c_uint = 1;
pub const SCM_J1939_DEST_NAME: ::std::os::raw::c_uint = 2;
pub const SCM_J1939_PRIO: ::std::os::raw::c_uint = 3;
pub const SCM_J1939_ERRQUEUE: ::std::os::raw::c_uint = 4;
pub type _bindgen_ty_3 = ::std::os::raw::c_uint;
pub const J1939_NLA_PAD: ::std::os::raw::c_uint = 0;
pub const J1939_NLA_BYTES_ACKED: ::std::os::raw::c_uint = 1;
pub type _bindgen_ty_4 = ::std::os::raw::c_uint;
pub const J1939_EE_INFO_NONE: ::std::os::raw::c_uint = 0;
pub const J1939_EE_INFO_TX_ABORT: ::std::os::raw::c_uint = 1;
pub type _bindgen_ty_5 = ::std::os::raw::c_uint;
#[repr(C)]
pub struct j1939_filter {
    pub name: name_t,
    pub name_mask: name_t,
    pub pgn: pgn_t,
    pub pgn_mask: pgn_t,
    pub addr: __u8,
    pub addr_mask: __u8,
}
#[test]
fn bindgen_test_layout_j1939_filter() {
    assert_eq!(
        ::std::mem::size_of::<j1939_filter>(),
        32usize,
        concat!("Size of: ", stringify!(j1939_filter))
    );
    assert_eq!(
        ::std::mem::align_of::<j1939_filter>(),
        8usize,
        concat!("Alignment of ", stringify!(j1939_filter))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<j1939_filter>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(j1939_filter),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<j1939_filter>())).name_mask as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(j1939_filter),
            "::",
            stringify!(name_mask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<j1939_filter>())).pgn as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(j1939_filter),
            "::",
            stringify!(pgn)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<j1939_filter>())).pgn_mask as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(j1939_filter),
            "::",
            stringify!(pgn_mask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<j1939_filter>())).addr as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(j1939_filter),
            "::",
            stringify!(addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<j1939_filter>())).addr_mask as *const _ as usize },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(j1939_filter),
            "::",
            stringify!(addr_mask)
        )
    );
}
#[repr(C)]
pub struct can_bittiming {
    pub bitrate: __u32,
    pub sample_point: __u32,
    pub tq: __u32,
    pub prop_seg: __u32,
    pub phase_seg1: __u32,
    pub phase_seg2: __u32,
    pub sjw: __u32,
    pub brp: __u32,
}
#[test]
fn bindgen_test_layout_can_bittiming() {
    assert_eq!(
        ::std::mem::size_of::<can_bittiming>(),
        32usize,
        concat!("Size of: ", stringify!(can_bittiming))
    );
    assert_eq!(
        ::std::mem::align_of::<can_bittiming>(),
        4usize,
        concat!("Alignment of ", stringify!(can_bittiming))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_bittiming>())).bitrate as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(can_bittiming),
            "::",
            stringify!(bitrate)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_bittiming>())).sample_point as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(can_bittiming),
            "::",
            stringify!(sample_point)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_bittiming>())).tq as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(can_bittiming),
            "::",
            stringify!(tq)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_bittiming>())).prop_seg as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(can_bittiming),
            "::",
            stringify!(prop_seg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_bittiming>())).phase_seg1 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(can_bittiming),
            "::",
            stringify!(phase_seg1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_bittiming>())).phase_seg2 as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(can_bittiming),
            "::",
            stringify!(phase_seg2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_bittiming>())).sjw as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(can_bittiming),
            "::",
            stringify!(sjw)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_bittiming>())).brp as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(can_bittiming),
            "::",
            stringify!(brp)
        )
    );
}
#[repr(C)]
pub struct can_bittiming_const {
    pub name: [::std::os::raw::c_char; 16usize],
    pub tseg1_min: __u32,
    pub tseg1_max: __u32,
    pub tseg2_min: __u32,
    pub tseg2_max: __u32,
    pub sjw_max: __u32,
    pub brp_min: __u32,
    pub brp_max: __u32,
    pub brp_inc: __u32,
}
#[test]
fn bindgen_test_layout_can_bittiming_const() {
    assert_eq!(
        ::std::mem::size_of::<can_bittiming_const>(),
        48usize,
        concat!("Size of: ", stringify!(can_bittiming_const))
    );
    assert_eq!(
        ::std::mem::align_of::<can_bittiming_const>(),
        4usize,
        concat!("Alignment of ", stringify!(can_bittiming_const))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_bittiming_const>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(can_bittiming_const),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_bittiming_const>())).tseg1_min as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(can_bittiming_const),
            "::",
            stringify!(tseg1_min)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_bittiming_const>())).tseg1_max as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(can_bittiming_const),
            "::",
            stringify!(tseg1_max)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_bittiming_const>())).tseg2_min as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(can_bittiming_const),
            "::",
            stringify!(tseg2_min)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_bittiming_const>())).tseg2_max as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(can_bittiming_const),
            "::",
            stringify!(tseg2_max)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_bittiming_const>())).sjw_max as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(can_bittiming_const),
            "::",
            stringify!(sjw_max)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_bittiming_const>())).brp_min as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(can_bittiming_const),
            "::",
            stringify!(brp_min)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_bittiming_const>())).brp_max as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(can_bittiming_const),
            "::",
            stringify!(brp_max)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_bittiming_const>())).brp_inc as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(can_bittiming_const),
            "::",
            stringify!(brp_inc)
        )
    );
}
#[repr(C)]
pub struct can_clock {
    pub freq: __u32,
}
#[test]
fn bindgen_test_layout_can_clock() {
    assert_eq!(
        ::std::mem::size_of::<can_clock>(),
        4usize,
        concat!("Size of: ", stringify!(can_clock))
    );
    assert_eq!(
        ::std::mem::align_of::<can_clock>(),
        4usize,
        concat!("Alignment of ", stringify!(can_clock))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_clock>())).freq as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(can_clock),
            "::",
            stringify!(freq)
        )
    );
}
pub const can_state_CAN_STATE_ERROR_ACTIVE: can_state = 0;
pub const can_state_CAN_STATE_ERROR_WARNING: can_state = 1;
pub const can_state_CAN_STATE_ERROR_PASSIVE: can_state = 2;
pub const can_state_CAN_STATE_BUS_OFF: can_state = 3;
pub const can_state_CAN_STATE_STOPPED: can_state = 4;
pub const can_state_CAN_STATE_SLEEPING: can_state = 5;
pub const can_state_CAN_STATE_MAX: can_state = 6;
pub type can_state = ::std::os::raw::c_uint;
#[repr(C)]
pub struct can_berr_counter {
    pub txerr: __u16,
    pub rxerr: __u16,
}
#[test]
fn bindgen_test_layout_can_berr_counter() {
    assert_eq!(
        ::std::mem::size_of::<can_berr_counter>(),
        4usize,
        concat!("Size of: ", stringify!(can_berr_counter))
    );
    assert_eq!(
        ::std::mem::align_of::<can_berr_counter>(),
        2usize,
        concat!("Alignment of ", stringify!(can_berr_counter))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_berr_counter>())).txerr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(can_berr_counter),
            "::",
            stringify!(txerr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_berr_counter>())).rxerr as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(can_berr_counter),
            "::",
            stringify!(rxerr)
        )
    );
}
#[repr(C)]
pub struct can_ctrlmode {
    pub mask: __u32,
    pub flags: __u32,
}
#[test]
fn bindgen_test_layout_can_ctrlmode() {
    assert_eq!(
        ::std::mem::size_of::<can_ctrlmode>(),
        8usize,
        concat!("Size of: ", stringify!(can_ctrlmode))
    );
    assert_eq!(
        ::std::mem::align_of::<can_ctrlmode>(),
        4usize,
        concat!("Alignment of ", stringify!(can_ctrlmode))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_ctrlmode>())).mask as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(can_ctrlmode),
            "::",
            stringify!(mask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_ctrlmode>())).flags as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(can_ctrlmode),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
pub struct can_device_stats {
    pub bus_error: __u32,
    pub error_warning: __u32,
    pub error_passive: __u32,
    pub bus_off: __u32,
    pub arbitration_lost: __u32,
    pub restarts: __u32,
}
#[test]
fn bindgen_test_layout_can_device_stats() {
    assert_eq!(
        ::std::mem::size_of::<can_device_stats>(),
        24usize,
        concat!("Size of: ", stringify!(can_device_stats))
    );
    assert_eq!(
        ::std::mem::align_of::<can_device_stats>(),
        4usize,
        concat!("Alignment of ", stringify!(can_device_stats))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_device_stats>())).bus_error as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(can_device_stats),
            "::",
            stringify!(bus_error)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_device_stats>())).error_warning as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(can_device_stats),
            "::",
            stringify!(error_warning)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_device_stats>())).error_passive as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(can_device_stats),
            "::",
            stringify!(error_passive)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_device_stats>())).bus_off as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(can_device_stats),
            "::",
            stringify!(bus_off)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<can_device_stats>())).arbitration_lost as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(can_device_stats),
            "::",
            stringify!(arbitration_lost)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<can_device_stats>())).restarts as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(can_device_stats),
            "::",
            stringify!(restarts)
        )
    );
}
pub const IFLA_CAN_UNSPEC: ::std::os::raw::c_uint = 0;
pub const IFLA_CAN_BITTIMING: ::std::os::raw::c_uint = 1;
pub const IFLA_CAN_BITTIMING_CONST: ::std::os::raw::c_uint = 2;
pub const IFLA_CAN_CLOCK: ::std::os::raw::c_uint = 3;
pub const IFLA_CAN_STATE: ::std::os::raw::c_uint = 4;
pub const IFLA_CAN_CTRLMODE: ::std::os::raw::c_uint = 5;
pub const IFLA_CAN_RESTART_MS: ::std::os::raw::c_uint = 6;
pub const IFLA_CAN_RESTART: ::std::os::raw::c_uint = 7;
pub const IFLA_CAN_BERR_COUNTER: ::std::os::raw::c_uint = 8;
pub const IFLA_CAN_DATA_BITTIMING: ::std::os::raw::c_uint = 9;
pub const IFLA_CAN_DATA_BITTIMING_CONST: ::std::os::raw::c_uint = 10;
pub const IFLA_CAN_TERMINATION: ::std::os::raw::c_uint = 11;
pub const IFLA_CAN_TERMINATION_CONST: ::std::os::raw::c_uint = 12;
pub const IFLA_CAN_BITRATE_CONST: ::std::os::raw::c_uint = 13;
pub const IFLA_CAN_DATA_BITRATE_CONST: ::std::os::raw::c_uint = 14;
pub const IFLA_CAN_BITRATE_MAX: ::std::os::raw::c_uint = 15;
pub const __IFLA_CAN_MAX: ::std::os::raw::c_uint = 16;
pub type _bindgen_ty_6 = ::std::os::raw::c_uint;
pub const CAN_RAW_FILTER: ::std::os::raw::c_uint = 1;
pub const CAN_RAW_ERR_FILTER: ::std::os::raw::c_uint = 2;
pub const CAN_RAW_LOOPBACK: ::std::os::raw::c_uint = 3;
pub const CAN_RAW_RECV_OWN_MSGS: ::std::os::raw::c_uint = 4;
pub const CAN_RAW_FD_FRAMES: ::std::os::raw::c_uint = 5;
pub const CAN_RAW_JOIN_FILTERS: ::std::os::raw::c_uint = 6;
pub type _bindgen_ty_7 = ::std::os::raw::c_uint;
pub const VXCAN_INFO_UNSPEC: ::std::os::raw::c_uint = 0;
pub const VXCAN_INFO_PEER: ::std::os::raw::c_uint = 1;
pub const __VXCAN_INFO_MAX: ::std::os::raw::c_uint = 2;
pub type _bindgen_ty_8 = ::std::os::raw::c_uint;
