# run both web (dx serve) and server (cargo run) concurrently

dev:
  just --summary
  concurrently --names "web,server" --prefix-colors "blue,green" \
    "just run-web" \
    "just run-server"

run-web:
  cd web && dx serve

run-server:
  cd server && cargo watch -q -c -x run | sed 's/^/[server] /'
