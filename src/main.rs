use rand::RngCore;
use sha2::{Digest, Sha512};
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const COMMIT_HEX: &str = "90243a7416f52151a8c6cecf633500dceb366895";

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

fn format_uuid(uuid: &[u8; 16], prefix: &str) -> String {
    let hex = hex::encode(uuid);

    format!(
        "{}-{}-{}-{}-{}ca010150",
        prefix,
        &hex[0..4],
        &hex[4..8],
        &hex[8..12],
        &hex[12..16]
    )
}

fn check(hash: &[u8]) -> bool {
    // 前33 bit = 4 bytes + 1 bit
    (u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]]) << 1
        | (hash[4] >> 7) as u32)
        == 0
}

fn worker(commit: Vec<u8>, prefix: String, found: Arc<AtomicBool>) {
    let mut rng = rand::thread_rng();

    while !found.load(Ordering::Relaxed) {
        let mut uuid = [0u8; 16];
        rng.fill_bytes(&mut uuid);

        // UUIDv4 标准位
        uuid[6] = (uuid[6] & 0x0F) | 0x40;
        uuid[8] = (uuid[8] & 0x3F) | 0x80;

        let mut hasher = Sha512::new();
        hasher.update(&commit);
        hasher.update(uuid);
        let result = hasher.finalize();

        if check(&result) {
            let out = format_uuid(&uuid, &prefix);
            println!("/answer {}", out);
            found.store(true, Ordering::Relaxed);
            break;
        }
    }
}

fn main() {
    let commit = hex_to_bytes(COMMIT_HEX);
    let prefix = &COMMIT_HEX[COMMIT_HEX.len() - 8..];

    let cores = num_cpus::get();
    println!("[*] using {} cores", cores);

    let found = Arc::new(AtomicBool::new(false));

    (0..cores).into_par_iter().for_each(|_| {
        worker(commit.clone(), prefix.to_string(), found.clone());
    });
}
