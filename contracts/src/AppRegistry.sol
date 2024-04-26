// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract AppRegistry is Ownable {
    struct Application {
        string name;
        address governanceToken;
        address usageToken;
        mapping(string => bool) processes;
    }

    mapping(string => Application) public applications;

    event ApplicationRegistered(string namespace);
    event ProcessAdded(string namespace, string process);

    function registerApplication(
        string memory namespace,
        string memory name,
        address governanceToken,
        address usageToken
    ) external onlyOwner {
        require(
            applications[namespace].governanceToken == address(0),
            "Application already registered"
        );

        Application storage app = applications[namespace];
        app.name = name;
        app.governanceToken = governanceToken;
        app.usageToken = usageToken;

        emit ApplicationRegistered(namespace);
    }

    function addProcess(
        string memory namespace,
        string memory process
    ) external onlyOwner {
        require(
            applications[namespace].governanceToken != address(0),
            "Application not registered"
        );
        require(
            !applications[namespace].processes[process],
            "Process already added"
        );

        applications[namespace].processes[process] = true;

        emit ProcessAdded(namespace, process);
    }
}
