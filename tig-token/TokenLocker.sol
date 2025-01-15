// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";


interface ITokenLocker {
    event TokensLocked(address indexed user, uint256 amount, uint256 locked);
    event TokensUnlocked(address indexed user, uint256 amount, uint256 locked, uint256 withdrawableTime);
    event TokensWithdrawn(address indexed user, uint256 amount);
    event TokensRewarded(address indexed user, uint256 amount, uint256 locked);

    function getNumPendingWithdrawals(address account) external view returns (uint256);
    function lock(uint256 amount) external;
    function unlock(uint256 amount) external;
    function withdraw(uint256 index) external;
    function rewardTokens(address account, uint256 amount) external;
}

contract TokenLocker is ITokenLocker, ReentrancyGuard, Ownable {
    using SafeMath for uint256;

    struct PendingWithdrawal {
        uint256 amount;
        uint256 timeWithdrawable;
    }

    IERC20 public immutable token;
    uint256 public immutable pendingPeriod;
    
    uint256 public totalLocked;
    uint256 public totalPendingWithdrawal;
    
    mapping(address => uint256) public locked;
    mapping(address => PendingWithdrawal[]) public pendingWithdrawals;
    
    constructor(address tokenAddress, uint256 _pendingPeriod) public Ownable(msg.sender) {
        token = IERC20(tokenAddress);
        pendingPeriod = _pendingPeriod;
    }

    function getNumPendingWithdrawals(address account) external view override returns (uint256) {
        return pendingWithdrawals[account].length;
    }

    function lock(uint256 amount) external override nonReentrant {
        require(amount > 0, "Amount must be greater than 0");
        
        require(token.transferFrom(msg.sender, address(this), amount), "Transfer failed");
        
        locked[msg.sender] = locked[msg.sender].add(amount);
        totalLocked = totalLocked.add(amount);
        
        emit TokensLocked(msg.sender, amount, locked[msg.sender]);
    }

    function unlock(uint256 amount) external override nonReentrant {
        require(amount > 0, "Amount must be greater than 0");
        require(locked[msg.sender] >= amount, "Insufficient locked balance");
        
        locked[msg.sender] = locked[msg.sender].sub(amount);
        totalLocked = totalLocked.sub(amount);
        totalPendingWithdrawal = totalPendingWithdrawal.add(amount);
        
        uint256 timeWithdrawable = block.timestamp.add(pendingPeriod);
        pendingWithdrawals[msg.sender].push(PendingWithdrawal({
            amount: amount,
            timeWithdrawable: timeWithdrawable
        }));
        
        emit TokensUnlocked(msg.sender, amount, locked[msg.sender], timeWithdrawable);
    }

    function withdraw(uint256 index) external override nonReentrant {
        require(index < pendingWithdrawals[msg.sender].length, "Invalid withdrawal index");
        PendingWithdrawal[] storage userWithdrawals = pendingWithdrawals[msg.sender];
        PendingWithdrawal memory withdrawal = userWithdrawals[index];
        
        require(block.timestamp >= withdrawal.timeWithdrawable, "Withdrawal not yet available");
        
        uint256 amount = withdrawal.amount;
        
        // Swap and pop
        userWithdrawals[index] = userWithdrawals[userWithdrawals.length - 1];
        userWithdrawals.pop();
        
        totalPendingWithdrawal = totalPendingWithdrawal.sub(amount);
        
        require(token.transfer(msg.sender, amount), "Transfer failed");
        
        emit TokensWithdrawn(msg.sender, amount);
    }

    function rewardTokens(address account, uint256 amount) external override onlyOwner nonReentrant {
        require(account != address(0), "Invalid address");
        require(amount > 0, "Amount must be greater than 0");
        
        require(token.transferFrom(msg.sender, address(this), amount), "Transfer failed");
        
        locked[account] = locked[account].add(amount);
        totalLocked = totalLocked.add(amount);
        
        emit TokensRewarded(account, amount, locked[account]);
    }
}