pub fn compute_checksum(id: u8, length: u8, instruction: u8, parameters: &[u8]) -> u8 {
    // https://emanual.robotis.com/docs/en/dxl/protocol1/#checksum-instruction-packet
    let mut checksum: u16 = 0; // avoid overflows, so set as u16
    checksum += id as u16;
    checksum += length as u16;
    checksum += instruction as u16;
    for param in parameters {
        checksum += *param as u16;
    }
    (!checksum & 0xff) as u8
}
