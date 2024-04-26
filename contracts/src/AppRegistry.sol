// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

// import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "./interfaces/IAppRegistry.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract AppRegistry is Ownable {

    mapping(string appId => ApplicationRecord appData) public applications;
    mapping(string processId => string appId) public processToAppId;

    uint256 public applicationCount;

    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    event ApplicationRegistered(string appId);
    event ProcessAdded(string appId, string processId);

    /*//////////////////////////////////////////////////////////////
                                 ERRORS
    //////////////////////////////////////////////////////////////*/
    error ApplicationAlreadyRegistered();
    error ApplicationNotRegistered();
    error ProcessAlreadyRegistered();

    constructor(address _owner) Ownable(_owner) {}

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

        emit ApplicationRegistered(appId);
    }

    function addProcess(
        string memory appId,
        string memory processId
    ) external payable {
        if (applications[appId].governanceToken == address(0)) {
            revert ApplicationNotRegistered();
        }

        // check that the processId is not already mapped to another appId
        if (keccak256(abi.encodePacked(processToAppId[processId])) != keccak256(abi.encodePacked(""))) {
            revert ProcessAlreadyRegistered();
        }

        processToAppId[processId] = appId;

        emit ProcessAdded(appId, processId);
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

    function isProcessRegistered(string memory processId) external view returns (bool) {
        string memory appId = processToAppId[processId];
        return bytes(appId).length > 0;
    }
}
