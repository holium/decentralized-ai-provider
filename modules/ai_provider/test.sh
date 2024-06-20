#kit update
echo "usage: ./test.sh [optional: no-job] [optional: no-kit-chain]"

use_kit_chain=true
if [ ! -z "$2" ]
  then
    echo "not using kit chain"
    use_kit_chain=""
fi

echo "killing old screens"
screen -S anvil -p 0 -X stuff $(printf \\003)
screen -S broker-1 -p 0 -X stuff $(printf \\004)
screen -S worker-1 -p 0 -X stuff $(printf \\004)

echo "starting new screens with fakenodes running in them"
if [ -z $use_kit_chain ]
  then
    echo "using raw anvil and manually running KNS scripts (requires KNS repo)"
    screen -h 10000 -S anvil -d -m anvil
    sleep 1
    cd ../../../KNS/
    forge script script/LocalDeployment.s.sol --rpc-url http://localhost:8545 --broadcast
    cd ../decentralized-ai-provider/modules/ai_provider
else
  screen -h 10000 -S anvil -d -m kit chain
fi
sleep 1
cd ../../contracts
forge build
# the following deploy script is responsible for registering the broker and the worker on chain, though in practice people will need to be able to do that themselves with something like
    #screen -S broker-1 -p 0 -X stuff "m our@broker:ai_provider:meme-deck.os '{\"RegisterBroker\": {\"process_id\": \"diffusion:ai_provider:meme-deck.os\"}}'$(printf \\r)"
forge script script/Deploy.s.sol --broadcast --rpc-url http://localhost:8545
cd ../modules/ai_provider
sleep 1
screen -d -m -S broker-1 kit boot-fake-node -p 8082 -f memedeck-broker-1 -o /tmp/memedeck-broker-1 --rpc ws://127.0.0.1:8545
sleep 9
screen -d -m -S worker-1 kit boot-fake-node -p 8083 -f memedeck-worker-1 -o /tmp/memedeck-worker-1 --rpc ws://127.0.0.1:8545

sleep 9 # wait for runtime compile

echo "installing the processes on the fakenodes"
kit bs -p 8082
kit s -p 8083

echo "initialize broker"
screen -S broker-1 -p 0 -X stuff "m our@broker:ai_provider:meme-deck.os \"SyncChainState\"$(printf \\r)"
sleep 1
# setup diffusion process to be able to talk to the comfy ui
screen -S worker-1 -p 0 -X stuff "m our@diffusion:ai_provider:meme-deck.os '{\"SetComfyUI\": {\"host\": \"holium:oifjw90f4jfl4ikfj0@node-1.diffusion.api.memedeck.xyz\", \"port\": 443, \"client_id\": 1}}'$(printf \\r)"
# setup the worker process to indicate to the broker that it is ready
screen -S worker-1 -p 0 -X stuff "m our@worker:ai_provider:meme-deck.os '{\"SetContractAddress\": {\"address\": \"0xa51c1fc2f0d1a1b8494ed1fe312d7c3a78ed91c0\"}}'$(printf \\r)"
screen -S worker-1 -p 0 -X stuff "m our@worker:ai_provider:meme-deck.os '{\"SetIsReady\": {\"is_ready\": true}}'$(printf \\r)"

if [ -z "$1" ] # if the 1st argument is null, we assume they DO want to kick off the default diffusion test job. they have to pass something for us to know they want to skip it
  then
    # finally, kick off the test comfy diffusion generation job
    echo "kicking off the diffusion job defined in test.json"
    test_job=`cat test.json | tr -d '\n'`
    command="m our@broker:ai_provider:meme-deck.os '{\"RequestTask\": {\"process_id\": \"diffusion:ai_provider:meme-deck.os\", \"task_parameters\": $test_job}}'"
    echo $command > /tmp/kinode-test-command.txt
    sleep 1
    screen -S broker-1 -p 0 -X readbuf /tmp/kinode-test-command.txt
    sleep 1
    screen -S broker-1 -p 0 -X paste .
    sleep 1
    screen -S broker-1 -p 0 -X stuff "$(printf \\r)"
  else
    echo "not kicking off a diffusion job"
fi
