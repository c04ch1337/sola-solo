use local_ip_address::local_ip;

/// Phase 15: Terminal pairing helper.
///
/// Prints the LAN URL and renders an ASCII QR code to allow instant onboarding
/// of a phone to the Mobile Bridge.
pub fn print_mobile_pairing_info(port: u16) {
    let my_local_ip = local_ip().unwrap_or_else(|_| "127.0.0.1".parse().unwrap());
    let pairing_url = format!("http://{}:{}", my_local_ip, port);

    println!("\n{}", "=".repeat(40));
    println!("L9 MOBILE BRIDGE PAIRING");
    println!("{}", "=".repeat(40));
    println!("LAN URL: {}", pairing_url);
    println!("Scan this QR code to launch the Mobile Bridge:");

    // Renders the QR code directly in the terminal.
    // Keep this best-effort: terminal rendering may vary by emulator.
    if let Err(e) = qr2term::print_qr(&pairing_url) {
        eprintln!("Failed to render QR code: {e}");
    }

    println!("{}\n", "=".repeat(40));
}

