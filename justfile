dev:
  just --summary
  concurrently --names "web,server,tailwind" --prefix-colors "blue,green,magenta" \
    "just run-web" \
    "just run-server" \
    "just run-tailwind"

run-web:
  cd web && dx serve

run-server:
  cd server && RUST_LOG=info RUSTFLAGS="-Awarnings" cargo watch -q -c -x run | sed 's/^/[server] /'

run-tailwind:
  cd web && npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch

