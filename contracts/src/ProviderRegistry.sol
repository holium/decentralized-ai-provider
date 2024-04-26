// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import "@openzeppelin/contracts/access/Ownable.sol";
import "./AppRegistry.sol";


contract ProviderRegistry is Ownable {
    struct Provider {
        address providerAddress;
        string processNamespace;
    }

    mapping(address => Provider) public providers;
    AppRegistry public appRegistry;

    event ProviderRegistered(address provider, string processNamespace);

    constructor(address _appRegistryAddress) Ownable(msg.sender) {
        appRegistry = AppRegistry(_appRegistryAddress);
    }

    function registerProvider(string memory processNamespace) external {
        require(providers[msg.sender].providerAddress == address(0), "Provider already registered");

        // Check if the process namespace exists in the AppRegistry
        string memory namespace = extractNamespace(processNamespace);
        string memory process = extractProcess(processNamespace);
        require(appRegistry.isProcessRegistered(namespace, process), "Process not registered in AppRegistry");

        Provider storage provider = providers[msg.sender];
        provider.providerAddress = msg.sender;
        provider.processNamespace = processNamespace;

        emit ProviderRegistered(msg.sender, processNamespace);
    }
}
