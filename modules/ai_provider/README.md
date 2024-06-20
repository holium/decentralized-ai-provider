fyi the `crate/shared` dir is nested "pointlessly" like that so that `kit` doesn't think it's a kinode process and get mad on building it. but this way we can still share types via that lib


# in order to run all this crap locally so that you can test kinode broker+worker generating an ai image

`./test.sh` in this directory is the main tool in your arsenal. It does have quite a few dependencies in order to work though.

in essence, this script spins up an anvil eth chain, deploys our contracts to it, and then spins up 2 kinodes and installs the ai_broker processes on them and then does some setup commands on those processes, and then runs an ai image generation if you want.

it puts the anvil and kinodes in separate `screen`s so just use `screen -ls` and `screen -r broker-1` to attach and see what's up. (Ctrl-a ESC will let you scroll up in the screen buffer)

Make sure you're on the current version of kinode and kit (v0.8.1 and kit 0.6.2).

Assuming you already have `anvil` installed and functional, (my version is `anvil 0.2.0 (54b3695 2023-12-07T00:29:49.658613000Z)` for what it's worth), now the `./test.sh` script should be ready to help you out. (also it depends on screen, which you should already have)

```
usage: ./test.sh [optional: no-job] [optional: no-kit-chain]
```

The optional no-job argument is useful to pass when you are trying to use this in conjunction with the backend/api from memedeck.

**YOU SHOULDNT NEED THE FOLLOWING INFO BUT YOU MIGHT:**

the `no-kit-chain` argument is something I needed on my other machine, but don't need on my macbook, so versions and milage my vary, but essentially it's purpose is to NOT use `kit chain` and instead manually deploy their KNS contracts to the local anvil which you may need to do if the contracts/Deploy.s.sol in this repo is failing. the argument requires a local KNS repo to exist in the relative path assumed by the script, and that repo must be on the branch `bp/local-scripts`. so:
```bash
git clone https://github.com/kinode-dao/KNS.git --recurse-submodules
cd KNS
git checkout --track origin/bp/local-scripts
forge build
```

should get you set up on that. contact marks.kino if you need help with this part
