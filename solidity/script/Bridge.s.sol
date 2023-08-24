// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {Bridge} from "src/Bridge.sol";

contract DeployBridge is Script {
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        console2.log("deployng bridge");
        address owner = vm.envAddress("OWNER");
        console2.log("owner: ", owner);

        Bridge bridge = new Bridge(address(owner));

        console2.log("deployed address: ", address(bridge));
        vm.stopBroadcast();

    }
}
