# --- TAHAP 1: BUILD (Memasak) ---
FROM rust:1.80-slim-bookworm as builder

WORKDIR /app

# Install library sistem
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy semua file
COPY . .

# Build mode release
RUN cargo build --release

# --- TAHAP 2: RUN (Menyajikan) ---
FROM debian:bookworm-slim

WORKDIR /app

# Install SSL (PENTING untuk connect ke Neon)
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*


COPY --from=builder /app/target/release/capstone-be ./server

# Beri izin eksekusi
RUN chmod +x ./server


EXPOSE 7860

# Jalankan server
CMD ["./server"]
