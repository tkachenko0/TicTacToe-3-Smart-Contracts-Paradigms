// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract TicTacToe {
    address public playerA;
    address public playerB;
    address public currentPlayer;

    uint public gameTimeoutBlock;
    uint public initialDeposit;

    bool playerBhasDeposited;

    enum CellState { Empty, X, O }

    CellState[3][3] public board;

    constructor(address _playerB, uint _gameTimeoutBlocks) payable {
        require(msg.value > 0, "Initial deposit must be greater than 0");
        require(_playerB != address(0), "PlayerB address cannot be zero");

        playerA = msg.sender;
        playerB = _playerB;
        playerBhasDeposited = false;
        initialDeposit = msg.value;
        gameTimeoutBlock = block.number + _gameTimeoutBlocks;
        currentPlayer = playerA;
    }

    function makeMove(uint8 row, uint8 col) external payable  {
        require(msg.sender == currentPlayer, "It's not your turn");
        require(row < 3 && col < 3, "Invalid cell coordinates");
        require(board[row][col] == CellState.Empty, "Cell is already occupied");

        // Verify if playerB has deposited
        if (currentPlayer == playerB && !playerBhasDeposited) {
            require(msg.value == initialDeposit, "Player B must makes the initial deposit");
            playerBhasDeposited = true;
        }

        board[row][col] = (currentPlayer == playerA) ? CellState.X : CellState.O;

        if (checkWinner()) {
            // Player who made the winning move gets the funds
            (bool success, ) = currentPlayer.call{value: address(this).balance}("");
            require(success, "Transfer failed");
        } else {
            // Switch to the other player
            currentPlayer = (currentPlayer == playerA) ? playerB : playerA;
        }
    }


    function timeout() external {
        require(block.number >= gameTimeoutBlock, "Timeout has not been reached yet");
        address allowedPlayer = currentPlayer == playerA ? playerB : playerA;
        require(msg.sender != allowedPlayer, "Not valid player");

        (bool success, ) = allowedPlayer.call{value: address(this).balance}("");
        require(success, "Transfer failed");
    }

   function checkWinner() internal view returns (bool) {
        // Check rows
        for (uint8 i = 0; i < 3; i++) {
            if (board[i][0] == board[i][1] && board[i][1] == board[i][2] && board[i][0] != CellState.Empty) {
                return true;
            }
        }

        // Check columns
        for (uint8 i = 0; i < 3; i++) {
            if (board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] != CellState.Empty) {
                return true;
            }
        }

        // Check diagonals
        if (board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != CellState.Empty) {
            return true;
        }
        if (board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] != CellState.Empty) {
            return true;
        }

        return false;
    }

}
