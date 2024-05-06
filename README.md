# Decentralized AI Provider

```ascii
┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│              │  │              │  │              │  │              │
│              │  │              │  │              │  │              │
│              │  │              │  │              │  │              │
│    worker    │  │    worker    │  │    worker    │  │    worker    │
│              │  │              │  │              │  │              │
│              │  │              │  │              │  │              │
│              │  │              │  │              │  │              │
└──────▲───────┘  └──────▲───────┘  └─────▲────────┘  └───────▲──────┘
       │                 │                │                   │       
       │                 │                │                   │       
       │               ┌─┴────────────────┴┐                  │       
       │               │                   │                  │       
       │               │                   │                  │       
       └───────────────┤      brokers      ├──────────────────┘       
                       │                   │                          
                       │                   │                          
                       └─────────▲─────────┘                          
                                 │                                    
                                 │                                    
┌────────────────────────────────┴───────────────────────────────────┐
│                                                                    │
│                                                                    │
│                            OP Sepolia                              │
│                                                                    │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

## Project setup

```bash
# install foundry
curl -L https://foundry.paradigm.xyz | bash

# install kit
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --git https://github.com/kinode-dao/kit

```

### 1. Run an anvil node
```bash
anvil
# if you want a sepolia fork
# anvil --fork-url https://sepolia.infura.io/v3/9aa3d95b3bc440fa88ea12eaa44561617 --port 8545
```

### 2. Deploy the contracts
The deploy script below will deploy the smart contracts with fake data needed for dev with the fakenode broker and worker.

```bash
cd contracts
forge build
# deploy
forge script script/Deploy.s.sol --broadcast --rpc-url http://localhost:8545
```


### 3. Start the broker & workers

```bash
# a fakenode broker 
kit boot-fake-node -p 8082 --network-router-port 8092 -f memedeck-broker-1.os -h /tmp/memedeck-broker-1 --rpc ws://127.0.0.1:8545

# a fakenode worker
kit boot-fake-node -p 8083 --network-router-port 8092 -f memedeck-worker-1.os -h /tmp/memedeck-worker-1 --rpc ws://127.0.0.1:8545

# a fakenode client (TODO)
```

### 4. Init actions

```bash
# in the kinode terminal for memedeck-broker-1.os

# in the kinode terminal for memedeck-worker-1.os
```
