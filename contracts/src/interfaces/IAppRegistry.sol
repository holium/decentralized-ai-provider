// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import "@openzeppelin-upgradeable/contracts/access/OwnableUpgradeable.sol";

struct ApplicationRecord {
    string name;
    address governanceToken;
    address usageToken;
}

struct ProcessRecord {
    string name;
    string repoUrl;
}

struct Worker {
    string workerKnsId;
    address workerAddress;
}

struct Broker {
    string brokerKnsId;
    address brokerAddress;
}

interface IAppRegistry {
    function registerApplication(
        string memory appId,
        string memory name,
        address governanceToken,
        address usageToken
    ) external payable;

    function registerProcess(string memory appId, string memory processId, string memory repoUrl) external payable;

    function removeProcess(string memory appId, string memory processId) external;

    function registerWorker(string memory appId, string memory workerKnsId, address workerAddress) external;

    function removeWorker(string memory appId, string memory workerKnsId) external;

    function registerBroker(string memory appId, string memory brokerKnsId, address brokerAddress) external;

    function removeBroker(string memory appId, string memory brokerKnsId) external;

    // Views
    function getApplication(string memory appId) external view returns (ApplicationRecord memory);

    function getApplications() external view returns (ApplicationRecord[] memory);

    function getProcesses() external view returns (ProcessRecord[] memory);

    function getProcessBrokers(string memory processId) external view returns (Broker[] memory);

    function getProcessWorkers(string memory processId) external view returns (Worker[] memory);

    function isProcessRegistered(string memory processId) external view returns (bool);
}
