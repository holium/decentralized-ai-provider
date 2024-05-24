// SPDX-License-Identifier: UNLICENSED
// https://book.getfoundry.sh/forge/writing-tests
pragma solidity >=0.8.21 <0.9.0;

import { PRBTest } from "prb-test/PRBTest.sol";
import { StdCheats } from "forge-std/StdCheats.sol";

import { AppRegistry } from "../src/AppRegistry.sol";

// Run it with `forge test -vvv` to see the console log.
contract AppRegistryTest is PRBTest, StdCheats {
    AppRegistry internal appRegistry;

    address internal owner;

    // A function invoked before each test case is run.
    function setUp() public virtual {
        owner = address(0x123);
        vm.deal(owner, 1 ether);
        // setup and gas fee
        vm.prank(owner);
        appRegistry = new AppRegistry();
        assertEq(appRegistry.owner(), owner);
        vm.stopPrank();
    }

    function test_registerApp() external {
        vm.startPrank(owner);
        string memory appId = "memedeck.os";
        string memory appName = "MemeDeck";
        address[] memory whitelist = new address[](1);
        whitelist[0] = address(0x70997970C51812dc3A010C7d01b50e0d17dc79C8);// pre-funded anvil address
        address governanceToken = address(1);
        address usageToken = address(2);

        appRegistry.registerApplication{ value: 0.01 ether }(appId, appName, whitelist, governanceToken, usageToken);
        vm.stopPrank();
    }
}
