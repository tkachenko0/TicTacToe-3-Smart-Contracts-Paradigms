---
sidebar_position: 1
---

# Stateful 

Account-Based smart contracts, when stateful, have the capability to maintain and modify their own internal state.

You can see the provided pseudocode implementation by opening the collapsed section below.

:::note
The provided code snippets are pseudocode representations. You can find the actual in [Account Based (Stateful) Implementation in Solidity](Statefull)
:::

```solidity
contract TicTacToe {
    address public playerA;
    address public playerB;
    address public currentPlayer;

    uint public gameTimeoutBlock;
    uint public requiredDeposit;

    bool playerAhasDeposited;
    bool playerBhasDeposited;

    enum CellState { Empty, X, O }

    CellState[3][3] public board;

    constructor(address _playerA, address _playerB, uint _gameTimeoutBlocks, uint256 _requiredDeposit)  {
        require(_playerA != address(0), "PlayerA address cannot be zero");
        require(_playerB != address(0), "PlayerB address cannot be zero");

        playerA = _playerA;
        playerB = _playerB;
        playerAhasDeposited = false;
        playerBhasDeposited = false;
        requiredDeposit = _requiredDeposit;
        gameTimeoutBlock = block.number + _gameTimeoutBlocks;
        currentPlayer = playerA;
    }

    ...
```

```solidity
    function makeMove(uint8 row, uint8 col) external payable  {
        require(msg.sender == currentPlayer, "It is not your turn");
        require(row < 3 && col < 3, "Invalid cell coordinates");
        require(board[row][col] == CellState.Empty, "Cell is already occupied");
        require(block.number < gameTimeoutBlock, "Timeout was reached");

        verifyDeposit();

        board[row][col] = (currentPlayer == playerA) ? CellState.X : CellState.O;

        if (checkWinner()) {
            // Player who made the winning move gets the funds
            // Transfer address(this).balance to currentPlayer
        } else {
            // Switch to the other player
            currentPlayer = (currentPlayer == playerA) ? playerB : playerA;
        }
    }

    function verifyDeposit() internal {
        require(msg.value == requiredDeposit, "Player must make the initial deposit");

        if (currentPlayer == playerA && !playerAhasDeposited) {
            playerAhasDeposited = true;
        } else if (currentPlayer == playerB && !playerBhasDeposited) {
            playerBhasDeposited = true;
        }
    }

    function checkWinner() internal view returns (bool) {
        // You can find the implementation of this function in the full code
    }
```

```solidity
    function timeout() external {
        require(block.number >= gameTimeoutBlock, "Timeout has not been reached yet");
        address allowedPlayer = currentPlayer == playerA ? playerB : playerA;
        require(msg.sender != allowedPlayer, "Not valid player");

        // Transfer address(this).balance to allowedPlayer
    }
```
