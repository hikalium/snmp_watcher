use anyhow::Result;
use std::collections::HashMap;
use std::env;
use std::process::Command;

fn snmpwalk(community: &str, host: &str, oid: &str) -> Result<Vec<(String, String)>> {
    let cmd = format!("snmpwalk -v2c -O afnq -c {community} {host} {oid}");
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");
    Ok(String::from_utf8_lossy(&output.stdout)
        .trim()
        .split('\n')
        .map(|s| s.split(' ').map(|s| s.to_string()).collect::<Vec<String>>())
        .map(|v| {
            (
                v[0].replace(oid, "").clone(),
                v[1].as_str().trim_matches('"').to_string().clone(),
            )
        })
        .collect::<Vec<(String, String)>>())
}

// 2: DOWN, 1: UP
const OID_ARUBA_AP_STAT: &'static str = ".1.3.6.1.4.1.14823.2.2.1.5.2.1.4.1.19";
const OID_ARUBA_AP_ADDR: &'static str = ".1.3.6.1.4.1.14823.2.2.1.5.2.1.4.1.2";
const OID_ARUBA_AP_NAME: &'static str = ".1.3.6.1.4.1.14823.2.2.1.5.2.1.4.1.3";

fn main() -> Result<()> {
    let snmp_community = &env::var("SNMP_COMMUNITY")?;
    let aruba_mc_host = &env::var("ARUBA_MC_HOST")?;

    let name: HashMap<String, String> = snmpwalk(snmp_community, aruba_mc_host, OID_ARUBA_AP_NAME)?
        .into_iter()
        .collect();
    let addr: HashMap<String, String> = snmpwalk(snmp_community, aruba_mc_host, OID_ARUBA_AP_ADDR)?
        .into_iter()
        .collect();
    let stat: HashMap<String, String> = snmpwalk(snmp_community, aruba_mc_host, OID_ARUBA_AP_STAT)?
        .into_iter()
        .collect();
    for k in name.keys() {
        if let (Some(name), Some(addr), Some(status)) = (name.get(k), addr.get(k), stat.get(k)) {
            println!(
                "{:20} {:10} {:10}",
                name,
                addr,
                if status == "1" { "UP" } else { "DOWN" }
            );
        }
    }
    Ok(())
}
