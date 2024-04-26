// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import "@openzeppelin/contracts/access/Ownable.sol";
import "./AppRegistry.sol";

contract RouterRegistry is Ownable {
    struct Router {
        address routerAddress;
        string processNamespace;
    }

    mapping(address => Router) public routers;
    AppRegistry public appRegistry;

    event RouterRegistered(address router, string processNamespace);

    constructor(address _appRegistryAddress) Ownable(msg.sender) {
        appRegistry = AppRegistry(_appRegistryAddress);
    }

    function registerRouter(string memory processNamespace) external {
        require(routers[msg.sender].routerAddress == address(0), "Router already registered");

        // Check if the process namespace exists in the AppRegistry
        string memory namespace = extractNamespace(processNamespace);
        string memory process = extractProcess(processNamespace);
        require(appRegistry.isProcessRegistered(namespace, process), "Process not registered in AppRegistry");

        Router storage router = routers[msg.sender];
        router.routerAddress = msg.sender;
        router.processNamespace = processNamespace;

        emit RouterRegistered(msg.sender, processNamespace);
    }

    function extractNamespace(string memory processNamespace) internal pure returns (string memory) {
        // Extract the namespace from the processNamespace string
        // Implement the logic based on your specific format
        // For example, if the format is "process:namespace", you can split the string and return the second part
        // This is a simplified example, you may need to adjust it based on your actual format
        bytes memory namespaceBytes = bytes(processNamespace);
        uint256 colonIndex = 0;
        for (uint256 i = 0; i < namespaceBytes.length; i++) {
            if (namespaceBytes[i] == ":") {
                colonIndex = i;
                break;
            }
        }
        require(colonIndex > 0, "Invalid process namespace format");
        return string(namespaceBytes[colonIndex + 1:]);
    }

    function extractProcess(string memory processNamespace) internal pure returns (string memory) {
        // Extract the process from the processNamespace string
        // Implement the logic based on your specific format
        // For example, if the format is "process:namespace", you can split the string and return the first part
        // This is a simplified example, you may need to adjust it based on your actual format
        bytes memory processBytes = bytes(processNamespace);
        uint256 colonIndex = 0;
        for (uint256 i = 0; i < processBytes.length; i++) {
            if (processBytes[i] == ":") {
                colonIndex = i;
                break;
            }
        }
        require(colonIndex > 0, "Invalid process namespace format");
        return string(processBytes[:colonIndex]);
    }
}
