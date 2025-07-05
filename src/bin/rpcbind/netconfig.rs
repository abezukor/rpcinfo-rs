use std::{fs, str::SplitWhitespace, sync::LazyLock};

#[allow(dead_code)]
pub struct NetConfigEntry {
    pub network_id: String,
    pub semantics: String,
    pub flags: Option<String>,
    pub protofamily: String,
    pub protoname: String,
    pub device: Option<String>,
    pub nametoaddr_libs: Option<String>,
}

pub static NET_CONFIG: LazyLock<Box<[NetConfigEntry]>> = LazyLock::new(|| {
    const FILE_PATH: &str = "/etc/netconfig";
    let netconfig_content = fs::read_to_string(FILE_PATH).unwrap();
    let mut entries = Vec::new();
    for line in netconfig_content.lines() {
        if line.trim().starts_with('#') {
            continue;
        }
        let mut netconfig = line.split_whitespace();
        let entry = NetConfigEntry {
            network_id: netconfig.next().unwrap().to_owned(),
            semantics: netconfig.next().unwrap().to_owned(),
            flags: netconfig_optional(&mut netconfig),
            protofamily: netconfig.next().unwrap().to_owned(),
            protoname: netconfig.next().unwrap().to_owned(),
            device: netconfig_optional(&mut netconfig),
            nametoaddr_libs: netconfig_optional(&mut netconfig),
        };
        entries.push(entry);
    }
    entries.into_boxed_slice()
});

fn netconfig_optional(netconfig: &mut SplitWhitespace) -> Option<String> {
    let item = netconfig.next().unwrap();
    if item == "-" {
        None
    } else {
        Some(item.to_owned())
    }
}
