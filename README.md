# Decentralized AI Provider

```ascii
┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│              │  │              │  │              │  │              │
│              │  │              │  │              │  │              │
│              │  │              │  │              │  │              │
│   provider   │  │   provider   │  │   provider   │  │   provider   │
│              │  │              │  │              │  │              │
│              │  │              │  │              │  │              │
│              │  │              │  │              │  │              │
└──────▲───────┘  └──────▲───────┘  └─────▲────────┘  └───────▲──────┘
       │                 │                │                   │       
       │                 │                │                   │       
       │               ┌─┴────────────────┴┐                  │       
       │               │                   │                  │       
       │               │                   │                  │       
       └───────────────┤      routers      ├──────────────────┘       
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

## Install kinode packages

```bash
# 1. start the sequencer and router
kit bs provider-dao-rollup/sequencer -p 8080
kit bs provider_dao_router -p 8080

# 2. start the provider
kit bs comfyui_provider -p 8081
```

## Start the provider & router

```bash
# boot a fakenode for the router, provider and client
kit boot-fake-node -p 8080 --network-router-port 8090 --persist -f memedeck-router.os -h /tmp/memedeck-router
kit boot-fake-node -p 8081 --network-router-port 8091 --persist -f memedeck-provider.os -h /tmp/memedeck-provider
kit boot-fake-node -p 8082 --network-router-port 8092 --persist -f memedeck-client.os -h /tmp/memedeck-client
```

## Start the client
```bash
# start the client
kit bs comfyui_client -p 8082 

admin:comfyui_client:nick1udwig.os {"SetRouterProcess": {"process_id": "provider_dao_router:provider_dao_router:nick1udwig.os"}}
admin:comfyui_client:nick1udwig.os {"SetRollupSequencer": {"address": "memedeck-router.os@sequencer:provider-dao-rollup:nick1udwig.os"}}

m our@client:comfyui_client:nick1udwig.os '{"RunJob": {"workflow": "workflow", "parameters": "{\"quality\": \"fast\", \"aspect_ratio\": \"square\", \"workflow\": \"workflow\", \"user_id\": \"0\", \"negative_prompt\": \"\", \"positive_prompt\": \"going for a walk in the park and looking at beautiful flowers and butterflies\", \"cfg_scale\": {\"min\": 1.0, \"max\": 1.0}, \"character\": {\"id\": \"pepe\"}, \"styler\": {\"id\": \"hand-drawn\"}}"}}'

```
