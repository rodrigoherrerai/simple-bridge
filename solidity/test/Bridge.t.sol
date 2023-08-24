// SPDX-License-Identifier: MIT
pragma solidity >= 0.8.0;

import {Test, console} from "forge-std/Test.sol";
import {Bridge} from "../src/Bridge.sol";

contract CounterTest is Test {
    event BridgeEth(address sender, uint256 amount, uint256 targetChain);

    Bridge public bridge;

    receive() external payable {}

    function setUp() public {
        bridge = new Bridge(address(this));
    }

    function testSetup() public {
        assertEq(bridge.owner(), address(this));
    }

    function testChain() public {
        assertEq(bridge.thisChain(), 31337);
    }

    function testShouldntReceive() public {
        assertEq(address(bridge).balance, 0);

        vm.expectRevert();
        payable(address(bridge)).transfer(1 ether);
    }

    function testBridgeEth() public {
        assertEq(address(bridge).balance, 0);

        uint256 amount = 1 ether;
        uint256 targetChain = 420;

        vm.expectEmit();
        emit BridgeEth(address(this), amount, targetChain);
        bridge.bridgeEth{value: amount}(targetChain);

        assertEq(address(bridge).balance, amount);
    }

    function testWithdraw() public {
        assertEq(address(bridge).balance, 0);

        uint256 initialBalance = address(this).balance;
        uint256 amount = 1 ether;
        uint256 targetChain = 420;
        bridge.bridgeEth{value: amount}(targetChain);
        assertEq(address(bridge).balance, amount);
        assertEq(address(this).balance, initialBalance - amount);

        bridge.withdraw(amount);
        assertEq(address(bridge).balance, 0);
        assertEq(address(this).balance, initialBalance);
    }
}
