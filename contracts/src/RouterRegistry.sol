// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";

contract RouterRegistry is Ownable {
    struct Router {
        address routerAddress;
        string processNamespace;
    }

    mapping(address => Router) public routers;

    event RouterRegistered(address router, string processNamespace);

    function registerRouter(string memory processNamespace) external {
        require(
            routers[msg.sender].routerAddress == address(0),
            "Router already registered"
        );

        Router storage router = routers[msg.sender];
        router.routerAddress = msg.sender;
        router.processNamespace = processNamespace;

        emit RouterRegistered(msg.sender, processNamespace);
    }
}
