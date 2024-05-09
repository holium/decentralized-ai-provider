echo "killing old screens"
screen -S anvil -p 0 -X stuff $(printf \\003)
screen -S broker-1 -p 0 -X stuff $(printf \\004)
screen -S worker-1 -p 0 -X stuff $(printf \\004)
echo "starting new screens with fakenodes running in them"
screen -h 10000 -S anvil -d -m kit chain
sleep 1
cd ../../contracts
forge build
# the following deploy script is responsible for registering the broker and the worker on chain, though in practice people will need to be able to do that themselves
forge script script/Deploy.s.sol --broadcast --rpc-url http://localhost:8545
cd ../modules/ai_provider
screen -S broker-1 -d -m kit boot-fake-node -p 8082 -f memedeck-broker-1 -h /tmp/memedeck-broker-1 --rpc ws://127.0.0.1:8545
sleep 1
screen -d -m -S worker-1 kit boot-fake-node -p 8083 -f memedeck-worker-1 -h /tmp/memedeck-worker-1 --rpc ws://127.0.0.1:8545
echo "installing the processes on the fakenodes"
kit bs -p 8082
kit s -p 8083
echo "initialize broker"
screen -S broker-1 -p 0 -X stuff "m our@broker:ai_provider:meme-deck.os \"SyncChainState\"$(printf \\r)"
sleep 1
#screen -S broker-1 -p 0 -X stuff "m our@broker:ai_provider:meme-deck.os '{\"RegisterBroker\": {\"process_id\": \"diffusion:memedeck:memedeck.os\"}}'$(printf \\r)"
screen -S worker-1 -p 0 -X stuff "m our@worker:ai_provider:meme-deck.os '{\"SetContractAddress\": {\"addres\": \"0xa51c1fc2f0d1a1b8494ed1fe312d7c3a78ed91c0\"}}'$(printf \\r)"
screen -S worker-1 -p 0 -X stuff "m our@worker:ai_provider:meme-deck.os '{\"SetIsReady\": {\"is_ready\": true}}'$(printf \\r)"
screen -S broker-1 -p 0 -X stuff "m our@broker:ai_provider:meme-deck.os '{\"RequestTask\": {\"process_id\": \"diffusion:memedeck:memedeck.os\", \"task_parameters\": {\"workflow\": \"basic\"}}}'$(printf \\r)"
