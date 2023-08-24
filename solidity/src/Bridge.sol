// SPDX-License-Identifier: MIT
pragma solidity >= 0.8.0;

/// @notice Super simple bridge that bridges Eth from Mainnet and Optimism.
contract Bridge {
    address public owner;

    event BridgeEth(address sender, uint256 amount, uint256 targetChain);

    constructor(address _owner) {
        owner = _owner;
    }

    function verifyChain(uint256 targetChain) public pure returns (bool) {
        // optimism = 420 mainnet is 10 but whatev.
        // mainnet = 1
        if (targetChain == 420 || targetChain == 1) return true;

        return false;
    }

    function withdraw(uint256 amount) external {
        require(msg.sender == owner);

        (bool success,) = owner.call{value: amount}("");
        require(success, "transfer failed");
    }

    function thisChain() public view returns (uint256) {
        return block.chainid;
    }

    function bridgeEth(uint256 targetChain) external payable {
        require(verifyChain(targetChain), "invalid chain");
        emit BridgeEth(msg.sender, msg.value, targetChain);
    }
}
