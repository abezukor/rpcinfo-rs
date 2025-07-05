pub const STAT_HIGHPROC: u32 = 13;
pub const VERS_2_STAT: u32 = 0;
pub const VERS_3_STAT: u32 = 1;
pub const VERS_4_STAT: u32 = 2;
pub const VERS_STAT: u32 = 3;
pub const _PORT: u32 = 111;
pub const HIGHPROC_2: u32 = 5;
pub const HIGHPROC_3: u32 = 8;
pub const HIGHPROC_4: u32 = 12;

#[derive(Debug, PartialEq, facet::Facet)]
pub struct NetBuf {
    pub maxlen: u32,
    pub buf: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, facet::Facet)]
pub struct RPList {
    pub rpcb_map: RPCB,
    pub rpcb_next: Vec<Box<RPList>>,
}

#[derive(Debug, PartialEq, Clone, facet::Facet)]
pub struct RPCB {
    pub r_prog: u32,
    pub r_vers: u32,
    pub r_netid: String,
    pub r_addr: String,
    pub r_owner: String,
}

#[derive(Debug, PartialEq)]
pub struct Entry {
    pub r_maddr: String,
    pub r_nc_netid: String,
    pub r_nc_semantics: u32,
    pub r_nc_protofmly: String,
    pub r_nc_proto: String,
}
#[derive(Debug, PartialEq)]
pub struct EntryList {
    pub rpcb_entry_map: Entry,
    pub rpcb_entry_next: Vec<Box<EntryList>>,
}
#[derive(Debug, PartialEq, facet::Facet)]
pub struct RmtCallArgs {
    pub prog: u32,
    pub vers: u32,
    pub proc: u32,
    pub args: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct RmtCallRes {
    pub addr: String,
    pub results: Vec<u8>,
}
#[derive(Debug, PartialEq)]
pub struct Stat {
    pub info: Proc,
    pub setinfo: i32,
    pub unsetinfo: i32,
    pub addrinfo: Option<Box<AddrList>>,
    pub rmtinfo: Option<Box<RmtCallList>>,
}
#[derive(Debug, PartialEq)]
pub struct StatByVers(pub [Stat; VERS_STAT as usize]);

#[derive(Debug, PartialEq)]
pub struct AddrList {
    pub prog: u32,
    pub vers: u32,
    pub success: i32,
    pub failure: i32,
    pub netid: String,
    pub next: Vec<Box<AddrList>>,
}

#[derive(Debug, PartialEq)]
pub struct Proc(pub [i32; STAT_HIGHPROC as usize]);

#[derive(Debug, PartialEq)]
pub struct RmtCallList {
    pub prog: u32,
    pub vers: u32,
    pub proc: u32,
    pub success: i32,
    pub failure: i32,
    pub indirect: i32,
    pub netid: String,
    pub next: Vec<Box<RmtCallList>>,
}
