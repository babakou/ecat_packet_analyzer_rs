pub struct EtherCATRawDatagram<'a> {
    cmd: u8,
    index: u8,
    slave_addr: u16,
    offset_addr: u16,
    data_len: u16,
    round_trip: bool,
    last_ind: bool,
    irq: u16,
    data: &'a [u8],
    wkc: u16,
}

pub struct EtherCATLogicalDatagram<'a> {
    cmd: u8,
    index: u8,
    logical_addr: u32,
    data_len: u16,
    round_trip: bool,
    last_ind: bool,
    irq: u16,
    data: &'a [u8],
    wkc: u16,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DLInfo {
    addr: u16,
    cmd: &'static str,
    pdiwdt: &'static str,
    port0_str: &'static str,
    port1_str: &'static str,
    port2_str: &'static str,
    port3_str: &'static str,
}

fn split_str_to_byte_array(bytes: &str) -> Result<Vec<u8>, String> {
    let mut splitted = Vec::<u8>::new();
    let mut i = 0;
    let len = bytes.len();
    println!("len = {}", len);

    while i < len {
        let end = usize::min(i + 2, len);
        let chunk_num = u8::from_str_radix(&bytes[i..end], 16).map_err(|err| err.to_string())?;
        splitted.push(chunk_num);
        i += 2;
    }

    Ok(splitted)
}

#[derive(PartialEq)]
enum LastIndicator {
    LastDatagram,
    MoreDatagramFollow,
}

enum RoundTrip {
    NotCirculating,
    Circulating
}

pub fn parse_byte_array(bytes: &str) -> Vec<DLInfo> {
    let byte_array = split_str_to_byte_array(bytes).unwrap();
    let mut dlinfos = Vec::<DLInfo>::new();
    let mut offset = 0;

    loop {
        let (dlinfo, datagram_len, last) = parse_one_datagram(&byte_array[offset..]);
        if let Some(dlinfo) = dlinfo {
            dlinfos.push(dlinfo);
        }

        if last == LastIndicator::LastDatagram {
            break;
        }

        offset += datagram_len as usize;
    }

    dlinfos
}

fn parse_one_datagram(byte_array: &[u8]) -> (Option<DLInfo>, u16, LastIndicator) {
    let cmd = byte_array[0];

    if (cmd == 10 || cmd == 11 || cmd == 12) {
        parse_one_logical_datagram(byte_array)
    } else {
        parse_one_raw_datagram(byte_array)
    }
}

fn parse_one_raw_datagram(byte_array: &[u8]) -> (Option<DLInfo>, u16, LastIndicator) {
    let cmd = byte_array[0];
    let index = byte_array[1];
    let slave_addr = u16::from_le_bytes([byte_array[2], byte_array[3]]);
    let offset_addr = u16::from_le_bytes([byte_array[4], byte_array[5]]);
    let len_roundtrip_lastind = u16::from_le_bytes([byte_array[6], byte_array[7]]);
    let len = len_roundtrip_lastind & 0x07FF;
    let roundtrip = if (len_roundtrip_lastind >> 14) as u8 & 0x01 == 0 {
        RoundTrip::NotCirculating
    } else {
        RoundTrip::Circulating
    };

    let last_indicator = if (len_roundtrip_lastind >> 15) as u8 & 0x01 == 0 {
        LastIndicator::LastDatagram
    } else {
        LastIndicator::MoreDatagramFollow
    };
    let irq = u16::from_le_bytes([byte_array[8], byte_array[9]]);
    let data = &byte_array[10..(10 + len as usize)];
    let wkc = u16::from_be_bytes([byte_array[10], byte_array[11]]);

    let mut dlinfo : Option<DLInfo> = None;
    if (offset_addr == 0x110) {
        let cmd = cmd_str(cmd);
        let dlstatus = u16::from_le_bytes([data[0], data[1]]);
        //println!("dlstatus = 0x{:x}", dlstatus);
        let pdiwdt = pdiwdt_str((dlstatus >> 1) & 0x0001);
        let port0status = portstatus_str((dlstatus >> 8) & 0x0003);
        let port1status = portstatus_str((dlstatus >> 10) & 0x0003);
        let port2status = portstatus_str((dlstatus >> 12) & 0x0003);
        let port3status = portstatus_str((dlstatus >> 14) & 0x0003);

        dlinfo = Some( DLInfo {
            addr: slave_addr,
            cmd: cmd,
            pdiwdt: pdiwdt,
            port0_str: port0status,
            port1_str: port1status,
            port2_str: port2status,
            port3_str: port3status,
        });
    }

    (dlinfo, len + 12, last_indicator)
}

fn portstatus_str(portstatus: u16) -> &'static str {
    //println!("portstatus = 0x{:x}", portstatus);
    match portstatus {
        0b00 => "loop open, no link",
        0b01 => "loop closed, no link",
        0b10 => "loop open, with link",
        0b11 => "loop closed, with link",
        _ => "unknown",
    }
}

fn cmd_str(cmd: u8) -> &'static str {
    match cmd {
        0x0 => "NOP",
        0x1 => "APRD",
        0x2 => "APWR",
        0x3 => "APRW",
        0x4 => "FPRD",
        0x5 => "FPWR",
        0x6 => "FPRW",
        0x7 => "BRD",
        0x8 => "BWR",
        0x9 => "BRW",
        0xA => "LRD",
        0xB => "LWR",
        0xC => "LRW",
        0xD => "ARMW",
        0xE => "FRMW",
        _ => "UNKNOWN",
    }
}

fn pdiwdt_str(pdiwdt: u16) -> &'static str {
    match pdiwdt {
        0 => "Run out",
        1 => "Okay",
        _ => "Unknown"
    }
}

fn parse_one_logical_datagram(byte_array: &[u8]) -> (Option<DLInfo>, u16, LastIndicator) {
    (None, 0, LastIndicator::LastDatagram)
}