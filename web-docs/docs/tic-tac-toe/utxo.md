---
sidebar_position: 3
---

# UTXO version

In the UTXO-Based paradigm, we adapted the Tic Tac Toe game to utilize the unique principles of this model.

The transaction condition script captures the logic for determining the outcome of a Tic Tac Toe game in the UTXO paradigm, considering factors such as player turns, timeouts, valid moves, and winning conditions. It allows players to withdraw deposits based on the game's outcome or continue the game if it's still ongoing.

The transaction has 2 inputs, one for each player. The inputs are the previous transactions that the players have made to the game. 

The logic is divided into two parts: 
- **Timeout reached**: if the game has timed out, the script allows the player who has not timed out to withdraw their deposit. 
- **Timeout not reached**: if the game is still ongoing, the script allows the player to make a move if it's their turn. If the move is valid, the script updates the board and checks if the player has won. If the player has won, the script allows the player to withdraw their deposit. If the player has not won, the script allows the other player to make a move by constraining the next transaction script's variables: `turnA` and `board`.

```yaml
# TicTacToe
inputs:
  txA ← sigA(tx1TicTacToe) # txA holds 1:T
  txB ← sigB(tx1TicTacToe) # txB holds 1:T
outputs:
  2:T → fun sig, row, col [board=[['Empty'; 3]; 3], turnA=true]:
    (
    # Timeout reached
    after N:
        # Allow player B to withdraw all the deposits
        ctxo.turnA and rtx[0].script == versig(Bob, rtx, sig) and rtx[0].val = 2:T
        or
        # Allow player A to withdraw all the deposits
        !ctxo.turnA and rtx[0].script == versig(Alice, rtx, sig) and rtx[0].val = 2:T
    )
    or
    (
    # Timeout not reached
    before N:
        # If valid coordinates and right board configuration
        ctxo.board[row, col] == 'Empty' and
        row >= 0 and row < 3 and col >= 0 and col < 3 and
        rtx[0].board[otherx, othery] == ctxo.board[otherx, othery] ∀(otherx, othery) != (row, col) and
        ((ctxo.turnA and rtx[0].board[row, col] == 'X') or (!ctxo.turnA and rtx[0].board[row, col] == 'O')) and
        # Checking the turn
        rtx[0].turnA == !ctxo.turnA and
        ((ctxo.turnA and versig(Alice, rtx, sig)) or (!ctxo.turnA and versig(Bob, rtx, sig))) and
        (
            (
                # Allow player A to withdraw
                ctxo.turnA and isWinner(rtx[0].board, 'Symbol X') and
                rtx[0].val = 2:T and
                rtx[0].script == versig(Alice, rtx, sig) and
            )
            or
            (
                # Allow player B to withdraw
                !ctxo.turnA and isWinner(rtx[0].board, 'Symbol O') and
                rtx[0].val = 2:T and
                rtx[0].script == versig(Bob, rtx, sig) and
            )
            or
            (
                # The game is not finished yet
                rtx[0].val = 2:T
                rtx[0].script == ctxo[0].script
            )
        )
    ) and
    |rtx.inputs|==1
```
