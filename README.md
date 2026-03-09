# hydra-sdk-rust

hydra-sdk-rust is a Rust SDK for building applications that interact with Hydra, Cardano's layer 2 protocol for fast, secure off-chain transaction processing. Hydra brings Cardano's safety guarantees to high-speed environments, and Rust is the perfect match, offering performance and safety

⚠️ Warning: this project is under heavy development, things will change frequently

- [ ] Transaction
  - [x] Build simple spending transaction 
  - [ ] Build spending transaction with datum
  - [ ] Build spending transaction with redeemer
- [X] Websocket commands  
  - [X] Init head  
  - [X] Close head  
  - [x] Increment  
  - [x] Recover  
- [x] HTTP interface
  - [X] Get snapshot  
  - [X] Submit transaction
- [x] Websocket interface
  - [X] Support adding callbacks for each type of websocket event
  - [x] Support all event types
    - [X] SnapshotConfirmed
    - [X] TxValid
    - [X] TxInvalid
    - [X] ValidationError
    - [X] Greetings
    - [x] Other messages
- [x] Prometheus metrics
    - [x] Number of nodes
    - [x] Websocket event per second
    - [x] Current utxo count
- [ ] Wallet integration