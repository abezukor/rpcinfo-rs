#[derive(Debug, PartialEq, facet::Facet)]
pub struct Mapping {
    pub prog: u32,
    pub vers: u32,
    pub prot: u32,
    pub port: u32,
}

#[derive(Debug, PartialEq, facet::Facet)]
pub struct PMapList {
    pub map: Mapping,
    pub next: Vec<Box<PMapList>>,
}

#[derive(Debug, PartialEq, facet::Facet)]
pub struct CallArgs {
    pub prog: u32,
    pub vers: u32,
    pub proc: u32,
    pub args: Vec<u8>,
}

#[derive(Debug, PartialEq, facet::Facet)]
pub struct CallResult {
    pub port: u32,
    pub res: Vec<u8>,
}
