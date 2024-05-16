// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.21 <0.9.0;

import { AppRegistry } from "../src/AppRegistry.sol"; // Adjust this path as necessary
import "../src/interfaces/IAppRegistry.sol";

import { BaseScript } from "./Base.s.sol";

import { console2 } from "forge-std/console2.sol";

contract Deploy is BaseScript {
    AppRegistry public appRegistry;

    function run() public {
        vm.startBroadcast(broadcaster);
        deployAndInitialize();
        registerSampleData();
        vm.stopBroadcast();
    }

    function deployAndInitialize() internal {
        // Deploy the contract
        appRegistry = new AppRegistry();

        // Initialize with the deployer as the owner
        assert(appRegistry.owner() == broadcaster);
        console2.log("AppRegistry deployed and initialized by:", broadcaster);
    }

    function registerSampleData() internal {
        // Register a sample application
        string memory appId = "memedeck:memedeck.os";
        string memory appName = "MemeDeck";
        address governanceToken = address(1); // Example token address
        address usageToken = address(2); // Example token address

        appRegistry.registerApplication(appId, appName, governanceToken, usageToken);
        console2.log("Application registered:", appId, appName);

        // Register a sample process under the registered application
        string memory processId = "diffusion:ai_provider:meme-deck.os";
        appRegistry.registerProcess(appId, processId, "diffusion", "https://github.com/holium/memedeck-node");
        console2.log("Process registered under application:", processId, appId);

        // Register a sample broker
        // address broker = address(3);
        string memory brokerId = "memedeck-broker-1.dev";
        appRegistry.registerBroker(processId, brokerId);
        console2.log("Broker registered under process:", brokerId, processId);

        // Register a sample worker
        // address worker = address(4);
        string memory workerId = "memedeck-worker-1.dev";
        appRegistry.registerWorker(processId, workerId);
        console2.log("Worker registered under process:", workerId, processId);
    }
}
