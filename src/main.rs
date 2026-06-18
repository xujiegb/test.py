use rand::RngCore;
use rayon::prelude::*;
use sha2::{Digest, Sha512};

const COMMIT: &[u8] = &hex_literal::hex!(
    "90243a7416f52151a8c6cecf633500dceb366895"
);

const PREFIX: &str = "eb366895";

fn gen_uuid(mut b: [u8; 16]) -> [u8; 16] {
    // UUIDv4 标准位
    b[6] = (b[6] & 0x0F) | 0x40;
    b[8] = (b[8] & 0x3F) | 0x80;
    b
}

fn check(h: &[u8]) -> bool {
    // 前33 bit = 4 bytes + 1 bit
    (u32::from_be_bytes([h[0], h[1], h[2], h[3]]) << 1)
        | ((h[4] >> 7) as u32)
        == 0
}

fn format(uuid: &[u8; 16]) -> String {
    let hex = hex::encode(uuid);

    format!(
        "{}-{}-{}-{}-{}ca010150",
        PREFIX,
        &hex[0..4],
        &hex[4..8],
        &hex[8..12],
        &hex[12..16]
    )
}

fn worker() -> Option<String> {
    let mut rng = rand::thread_rng();

    loop {
        let mut b = [0u8; 16];
        rng.fill_bytes(&mut b);

        let uuid = gen_uuid(b);

        let mut hasher = Sha512::new();
        hasher.update(COMMIT);
        hasher.update(uuid);
        let hash = hasher.finalize();

        if check(&hash) {
            return Some(format(&uuid));
        }
    }
}

fn main() {
    let cores = num_cpus::get();

    println!("[*] using {} cores", cores);

    let result = (0..cores)
        .into_par_iter()
        .find_map_any(|_| worker());

    if let Some(ans) = result {
        println!("/answer {}", ans);
    }
}
