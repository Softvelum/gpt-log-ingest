use serde::Deserialize;
use std::net::Ipv4Addr;

#[derive(Deserialize)]
struct RawStream {
    id: Option<String>,
    protocol: Option<String>,
    ip: Option<String>,
    port: Option<u16>,
    cam_alias: Option<String>,
    description: Option<String>,
    mip: Option<String>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Stream {
    pub id: String,
    pub name: String,
    pub alias: String,
    pub group: Ipv4Addr,
    pub port: u16,
    pub mip: Option<Ipv4Addr>,
}

impl Stream {
    pub fn key(&self) -> (u32, u16) {
        (u32::from(self.group), self.port)
    }
    pub fn label(&self) -> String {
        format!("{}:{}", self.group, self.port)
    }
}

/// Load Nimble's stream list JSON and keep only UDP multicast inputs.
pub fn load(path: &str) -> Result<(Vec<Stream>, usize), String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("cannot read {}: {}", path, e))?;
    let raw: Vec<RawStream> =
        serde_json::from_str(&text).map_err(|e| format!("cannot parse {}: {}", path, e))?;
    let total = raw.len();
    let mut out = Vec::new();
    for r in raw {
        if r.protocol.as_deref().map(|p| p.eq_ignore_ascii_case("udp")) != Some(true) {
            continue;
        }
        let group: Ipv4Addr = match r.ip.as_deref().and_then(|s| s.trim().parse().ok()) {
            Some(g) => g,
            None => continue,
        };
        if !group.is_multicast() {
            continue;
        }
        let port = match r.port {
            Some(p) => p,
            None => continue,
        };
        let mip = r.mip.as_deref().and_then(|s| s.trim().parse().ok());
        let desc = r.description.unwrap_or_default();
        let alias = r.cam_alias.unwrap_or_default();
        let name = if !desc.trim().is_empty() {
            desc.trim().to_string()
        } else if !alias.trim().is_empty() {
            alias.trim().to_string()
        } else {
            format!("{}:{}", group, port)
        };
        out.push(Stream { id: r.id.unwrap_or_default(), name, alias, group, port, mip });
    }
    Ok((out, total))
}
