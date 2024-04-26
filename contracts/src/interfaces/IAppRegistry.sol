// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

struct ApplicationRecord {
    string name;
    address governanceToken;
    address usageToken;
}

interface IAppRegistry {
    function registerApplication(string calldata appId) external;
    function getApplication(string calldata appId) external view returns (ApplicationRecord memory);
    function addProcess(string calldata appId, string calldata processId) external;
}
