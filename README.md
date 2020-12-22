# DiamondDaggifier

A small website/backend to turn your minecraft skin into a DiamondDagger590 cosplay!

## Requirements

- Rust nightly installed (required by rocket.rs)
- Node.js 8.0 or later
- Yarn (NOT NPM)

## Set-up

### Backend

First make sure you're in the backend directory!

Run ```cargo run``` to run a debug build, make sure you edit Rocket.toml if you want to run the http server on a port other
than 80.

To build the backend for production run ```cargo build --release```

### Frontend
First make sure you're in the frontend directory!

To install all required dependencies run ```yarn install```

Then you can launch a dev server using ```yarn start```

To build the frontend for release use ```yarn build```
