---
sidebar_position: 1
---

# Stateful version

Account-Based smart contracts, when stateful, have the capability to maintain and modify their own internal state. The contract itself maintains the state of the game. It doesn't rely on external systems to keep track of the game board or whose turn it is. Instead, it updates its internal state based on the actions of the players tracking the game board, whose turn it is, and whether someone has won.

:::info
The provided code snippets are pseudocode representations. You can find the actual in [Account Based (Stateful) Implementation in Solidity](https://github.com/tkachenko0/TicTacToe-Blockchain/tree/main/Statefull)
:::

When deployed, the contract requires certain parameters to start a game, including the addresses of two players (`playerA` and `playerB`), the number of blocks until the game times out (`gameTimeoutBlock`), and the required deposit in to participate (`requiredDeposit`). The contract also maintains the current player (`currentPlayer`), the state of the board (`board`), and whether each player has deposited (`playerAhasDeposited` and `playerBhasDeposited`).

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

Then we need a function to allow players to make a move. This function requires the player to be the current player, the cell to be empty, and the game to not have timed out. If these conditions are met, the function updates the board, checks if the player has won, and switches the current player.

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

Finally, we need a function to allow players to withdraw their funds if the game times out. This function requires the game to have timed out and the player to not be the current player. If these conditions are met, the function transfers the funds to the player.

```solidity
    function timeout() external {
        require(block.number >= gameTimeoutBlock, "Timeout has not been reached yet");
        address allowedPlayer = currentPlayer == playerA ? playerB : playerA;
        require(msg.sender != allowedPlayer, "Not valid player");

        // Transfer address(this).balance to allowedPlayer
    }
```
