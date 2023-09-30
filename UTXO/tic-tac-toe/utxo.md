---
sidebar_position: 3
---

# UTXO

In the UTXO-Based paradigm, we adapted the Tic Tac Toe game to utilize the unique principles of this model.

You can see the provided pseudocode implementation by opening the collapsed section below.

```yaml
tx1TicTacToe
inputs:
  txA ← sigA(tx1TicTacToe)		(txA holds 1:T)
  txB ← sigB(tx1TicTacToe)		(txB holds 1:T)
outputs:
  2:T → fun sig, row, col [board=[['Empty', 'Empty', 'Empty']], turnA=true]:
          (
            (after N : rtxo.turnA && rtx[0].script: versigB(rtx, sig) && rtx[0].val = 2:T)
            or
            (after N : !rtxo.turnA && rtx[0].script: versigA(rtx, sig) && rtx[0].val = 2:T)
          )
          or
          (
            before N &&
            rtx[0].script == rtxo[0].script &&
            row >= 0 && row < 3 && col >= 0 && col < 3 &&
            ((rtxo.turnA && versigA(rtx, sig)) or (!rtxo.turnA && versigB(rtx, sig))) &&
            rtx[0].turnA == !rtxo.turnA &&
            rtxo.board[row, col] == 'Empty' &&
            ((rtxo.turnA && rtx[0].board[row, col] == 'X') or (!rtxo.turnA && rtx[0].board[pos_x, pos_y] == 'O')) &&
            rtx[0].board[otherx, othery] == rtxo.board[otherx, othery] for all (otherx, othery) != (row, col) &&
            (
              (
                rtxo.turnA &&
                isWinner(rtx[0].board, 'Symbol X') &&
                rtx[0].val = 0:T &&
                rtx[1].script == versigA(rtx, sig) &&
                rtx[1].val = 2:T
              )
              or
              (
                !rtxo.turnA &&
                isWinner(rtx[0].board, 'Symbol O') &&
                rtx[0].val = 0:T &&
                rtx[1].script == versigB(rtx, sig) &&
                rtx[1].val = 2:T
              )
              or
              rtx[0].val = 2:T
            )
          ) &&
          |rtx.inputs|==1
```
