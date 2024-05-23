// SPDX-License-Identifier: MIT
pragma solidity >=0.8.25 <0.9.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

abstract contract HOLToken is ERC20, ReentrancyGuard, Ownable {
    uint256 public constant MAX_SUPPLY = 1e9 * 1e18; // 1 billion tokens
    mapping(address => uint256) public stakingBalances;
    mapping(address => uint256) public lastUpdateTime;
    mapping(address => uint256) public rewardsPerTokenPaid;
    mapping(address => uint256) public rewards;

    uint256 public rewardRate = 100; // Reward rate per token, per second
    uint256 public totalSupplyCapped;
    uint256 public lastRewardUpdateTime = block.timestamp;

    constructor() ERC20("HOL Token", "HOL") {
        _mint(msg.sender, 100000000 * 1e18); // Mint initial supply to the deployer
    }

    function updateReward(address account) internal {
        rewards[account] = earned(account);
        lastUpdateTime[account] = block.timestamp;
        rewardsPerTokenPaid[account] = rewardPerToken();
    }

    function rewardPerToken() public view returns (uint256) {
        if (totalSupplyCapped == 0) {
            return 0;
        }
        return (rewardRate * (block.timestamp - lastRewardUpdateTime) * 1e18) / totalSupplyCapped;
    }

    function earned(address account) public view returns (uint256) {
        return
            ((stakingBalances[account] * (rewardPerToken() - rewardsPerTokenPaid[account])) / 1e18) + rewards[account];
    }

    function stake(uint256 _amount) external nonReentrant {
        require(_amount > 0, "Cannot stake 0");
        require(totalSupplyCapped + _amount <= MAX_SUPPLY, "Exceeds MAX_SUPPLY");
        totalSupplyCapped += _amount;
        stakingBalances[msg.sender] += _amount;
        updateReward(msg.sender);
        _mint(msg.sender, _amount);
    }

    function withdraw(uint256 _amount) public nonReentrant {
        require(_amount > 0, "Cannot withdraw 0");
        require(_amount <= stakingBalances[msg.sender], "Exceeds staked amount");
        totalSupplyCapped -= _amount;
        stakingBalances[msg.sender] -= _amount;
        updateReward(msg.sender);
        _burn(msg.sender, _amount);
    }

    function claimReward() public nonReentrant {
        uint256 reward = earned(msg.sender);
        require(reward > 0, "No reward earned");
        rewards[msg.sender] = 0;
        lastUpdateTime[msg.sender] = block.timestamp;
        _mint(msg.sender, reward);
    }

    function transfer(address recipient, uint256 amount) public override returns (bool) {
        require(amount > 0, "Amount must be greater than zero");
        uint256 burnAmount = amount / 100; // 1% of the transfer amount
        uint256 sendAmount = amount - burnAmount;
        _burn(msg.sender, burnAmount);
        return super.transfer(recipient, sendAmount);
    }

    function increaseRewardRate(uint256 newRate) external onlyOwner {
        rewardRate = newRate;
        lastRewardUpdateTime = block.timestamp;
    }

    function decreaseRewardRate(uint256 newRate) external onlyOwner {
        require(newRate < rewardRate, "New rate must be lower");
        rewardRate = newRate;
        lastRewardUpdateTime = block.timestamp;
    }
}
