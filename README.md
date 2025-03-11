# sage-playground

Your playground to get started with the Substrate Asset Gaming Framework (SAGE)

## Backend Quickstart

To quickly launch a testing node:

```sh
cd backend
cargo build-sage-node
./target/release/sage-playground-node --dev
```

Check [here](/backend/README.md) for more details on the backend, also check the SAGE [book](https://wiki.ajuna.io/build-with-us/sage-substrate-asset-game-engine). 

## Frontend Quickstart

To run the frontend:

```sh
cd frontend
pnpm i
pnpm dev
```

Make sure the node is running so the connection works.

Check [here](/frontend/README.md) for more details on the frontend.
