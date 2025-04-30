use nix::libc;

const NCCS: usize = 19;

#[repr(C)]
pub struct termios2 {
    pub c_iflag: libc::tcflag_t,
    pub c_oflag: libc::tcflag_t,
    pub c_cflag: libc::tcflag_t,
    pub c_lflag: libc::tcflag_t,
    pub c_line: libc::cc_t,
    pub c_cc: [libc::cc_t; NCCS],
    pub c_ispeed: libc::speed_t,
    pub c_ospeed: libc::speed_t,
}
