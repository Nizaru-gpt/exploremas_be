# --- TAHAP 1: BUILD (Memasak) ---
# GANTI DARI 1.80 KE 'slim-bookworm' (ini otomatis pakai versi Rust stable terbaru, misal 1.84+)
FROM rust:slim-bookworm as builder

WORKDIR /app

# Install library sistem untuk kompilasi
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy semua file source code
COPY . .

# Build mode release
# Tips: Jika ada Cargo.lock, ini akan memastikan versi library konsisten
RUN cargo build --release

# --- TAHAP 2: RUN (Menyajikan) ---
FROM debian:bookworm-slim

WORKDIR /app

# Install CA Certs & OpenSSL (PENTING untuk connect ke Neon/Database luar)
# 'libssl-dev' biasanya tidak wajib di runtime, cukup 'openssl' dan 'ca-certificates'
# tapi saya biarkan libssl-dev agar aman sesuai config lamamu.
RUN apt-get update && apt-get install -y ca-certificates openssl libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy hasil build dari tahap 1
COPY --from=builder /app/target/release/capstone-be ./server

# Beri izin eksekusi
RUN chmod +x ./server

# Expose port sesuai ketentuan Hugging Face Spaces (7860)
EXPOSE 7860

# Jalankan server
CMD ["./server"]