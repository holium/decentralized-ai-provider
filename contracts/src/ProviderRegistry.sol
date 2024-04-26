// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";

contract ProviderRegistry is Ownable {
    struct Provider {
        address providerAddress;
        string processNamespace;
    }

    mapping(address => Provider) public providers;

    event ProviderRegistered(address provider, string processNamespace);

    function registerProvider(string memory processNamespace) external {
        require(
            providers[msg.sender].providerAddress == address(0),
            "Provider already registered"
        );

        Provider storage provider = providers[msg.sender];
        provider.providerAddress = msg.sender;
        provider.processNamespace = processNamespace;

        emit ProviderRegistered(msg.sender, processNamespace);
    }
}
