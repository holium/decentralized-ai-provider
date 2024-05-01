// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

// import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "./interfaces/IAppRegistry.sol";
import "@openzeppelin-upgradeable/contracts/access/OwnableUpgradeable.sol";
import "@openzeppelin-upgradeable/contracts/token/ERC20/ERC20Upgradeable.sol";
import { console2 } from "forge-std/console2.sol";

contract AppRegistry is OwnableUpgradeable {
    mapping(string appId => ApplicationRecord appData) public applications;
    mapping(string processId => ProcessRecord processData) public processes;
    mapping(string processId => string appId) public processToAppId;
    string[] private appIds;
    string[] private processIds;

    // provider and router mappings
    mapping(string workerKnsId => Worker workerData) public workers;
    mapping(string brokerKnsId => Broker brokerData) public brokers;
    mapping(string processId => Worker[] workers) public processIdToWorkers;
    mapping(string processId => Broker[] brokers) public processIdToBrokers;

    uint256 public applicationCount;
    uint256 public processCount;

    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    event ApplicationRegistered(string appId);
    event ProcessRegistered(string appId, string processId);
    event ProcessRemoved(string appId, string processId);
    event WorkerRegistered(string workerId, string processNamespace);
    event WorkerRemoved(string workerId, string processNamespace);
    event BrokerRegistered(string brokerId, string processNamespace);
    event BrokerRemoved(string brokerId, string processNamespace);

    /*//////////////////////////////////////////////////////////////
                                 ERRORS
    //////////////////////////////////////////////////////////////*/
    error Unauthorized();
    error ApplicationAlreadyRegistered();
    error ApplicationNotRegistered();
    error ProcessAlreadyRegistered();
    error ProcessNotRegistered();

    constructor() {
        initialize(msg.sender);
    }

    function initialize(address _owner) public initializer {
        __Ownable_init(_owner);
        transferOwnership(_owner);
    }

    function registerApplication(
        string memory appId,
        string memory name,
        address governanceToken,
        address usageToken
    ) external payable {
        if (applications[appId].governanceToken != address(0)) {
            revert ApplicationAlreadyRegistered();
        }

        ApplicationRecord storage app = applications[appId];
        app.name = name;
        app.governanceToken = governanceToken;
        app.usageToken = usageToken;
        appIds.push(appId);

        emit ApplicationRegistered(appId);
    }

    function registerProcess(
        string memory appId,
        string memory processId,
        string memory name,
        string memory repoUrl
    ) external payable {
        if (applications[appId].governanceToken == address(0)) {
            revert ApplicationNotRegistered();
        }

        if (keccak256(abi.encodePacked(processToAppId[processId])) != keccak256(abi.encodePacked(""))) {
            revert ProcessAlreadyRegistered();
        }

        ProcessRecord storage process = processes[processId];
        process.name = name;
        process.repoUrl = repoUrl;
        processIds.push(processId);
        processCount += 1;

        processToAppId[processId] = appId;

        emit ProcessRegistered(appId, processId);
    }

    function removeProcess(string memory processId) external {
        string memory appId = processToAppId[processId];
        if (bytes(appId).length == 0) {
            revert ProcessNotRegistered();
        }

        delete processToAppId[processId];
        // remove process from processIds

        emit ProcessRemoved(appId, processId);
    }

    // worker functions
    function registerWorker(string calldata processId, string calldata workerKnsId) external {
        Worker storage worker = workers[workerKnsId];
        worker.workerAddress = msg.sender;
        worker.workerKnsId = workerKnsId;
        processIdToWorkers[processId].push(worker);
        // emit WorkerRegistered(workerKnsId, processId);
    }

    function removeWorker(string calldata processId, address workerAddress) external {
        if (msg.sender != owner()) {
            revert Unauthorized();
        }

        Worker[] storage localWorkers = processIdToWorkers[processId];
        uint256 length = localWorkers.length;
        for (uint256 i = 0; i < length; i++) {
            if (localWorkers[i].workerAddress == workerAddress) {
                string memory workerKnsId = localWorkers[i].workerKnsId;
                localWorkers[i] = localWorkers[length - 1];
                localWorkers.pop();
                // emit event
                emit WorkerRemoved(workerKnsId, processId);
                return;
            }
        }
    }

    // broker functions
    function registerBroker(string calldata processId, string calldata brokerKnsId) external {
        // create new broker
        Broker storage broker = brokers[brokerKnsId];
        broker.brokerAddress = msg.sender;
        broker.brokerKnsId = brokerKnsId;
        console2.log("brokerKnsId: %s", broker.brokerKnsId);
        processIdToBrokers[processId].push(broker);
        // emit BrokerRegistered(brokerKnsId, processId);
    }

    function removeBroker(string calldata processId, address brokerAddress) external {
        if (msg.sender != owner()) {
            revert Unauthorized();
        }

        Broker[] storage localBrokers = processIdToBrokers[processId];
        uint256 length = localBrokers.length;
        for (uint256 i = 0; i < length; i++) {
            if (localBrokers[i].brokerAddress == brokerAddress) {
                string memory brokerKnsId = localBrokers[i].brokerKnsId;
                localBrokers[i] = localBrokers[length - 1];
                localBrokers.pop();
                emit BrokerRemoved(brokerKnsId, processId);
                return;
            }
        }
    }

    /*//////////////////////////////////////////////////////////////
                          GOVERNANCE FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /*//////////////////////////////////////////////////////////////
                            VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    function getApplication(string memory appId) external view returns (ApplicationRecord memory) {
        return applications[appId];
    }

    function getApplications() external view returns (ApplicationRecord[] memory) {
        ApplicationRecord[] memory apps = new ApplicationRecord[](appIds.length);
        for (uint256 i = 0; i < appIds.length; i++) {
            apps[i] = applications[appIds[i]];
        }
        return apps;
    }

    function getProcesses() external view returns (ProcessRecord[] memory) {
        ProcessRecord[] memory processList = new ProcessRecord[](processIds.length);
        for (uint256 i = 0; i < processIds.length; i++) {
            processList[i] = processes[processIds[i]];
        }
        return processList;
    }

    function getProcessBrokers(string memory processId) external view returns (Broker[] memory) {
        console2.log("processId: %s", processId);
        return processIdToBrokers[processId];
    }

    function getProcessWorkers(string memory processId) external view returns (Worker[] memory) {
        return processIdToWorkers[processId];
    }

    function isProcessRegistered(string memory processId) external view returns (bool) {
        string memory appId = processToAppId[processId];
        return bytes(appId).length > 0;
    }
}
