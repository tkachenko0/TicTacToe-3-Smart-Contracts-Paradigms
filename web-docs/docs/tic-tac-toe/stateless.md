---
sidebar_position: 2
---

# Stateless version

The Account-Based paradigm, in its stateless form, allows executing smart contract functions without maintaining any internal state.

:::tip
You can also play the game on the `Solana testnet` on the address `Enq7EUNZshTBsTMDzimmv4dv1wC8jaZmm7wgrSH4UPsj` using the provided [client code](https://github.com/tkachenko0/TicTacToe-Blockchain/blob/main/Stateless/tic-tac-toe-anchor/tests/test_tic_tac_toe.ts)
:::

:::info
The provided code snippets are pseudocode representations. 
You can find the actual implementation with:
- [Anchor on-chain code](https://github.com/tkachenko0/TicTacToe-Blockchain/blob/main/Stateless/tic-tac-toe-anchor/programs/tic-tac-toe-anchor/src/lib.rs)
- [TypeScript client code](https://github.com/tkachenko0/TicTacToe-Blockchain/blob/main/Stateless/tic-tac-toe-anchor/tests/test_tic_tac_toe.ts)
:::

![Tic Tac Toe Stateless structure](/img/tic_tac_toe_stateless_structure.png)

In the initial steps, we must set up the essential data structures required to establish the initial state of a Tic Tac Toe game on the Solana blockchain. The `GameData` structure is responsible for storing game-specific details, while the `InitializeCtx` structure serves as the context for initiating the game. The `initialize` function acts as the starting point for configuring the game state by utilizing the provided context and parameters.

```rust
// Enum representing the game board cell
pub enum Symbol {
    SymbolX,
    SymbolO,
    SymbolEmpty,
}

// Struct representing the game state
pub struct GameData {
    pub player_a: Pubkey,
    pub player_b: Pubkey,
    pub turn_a: bool,
    pub player_a_has_deposited: bool,
    pub player_b_has_deposited: bool,
    pub board: [[Symbol; 3]; 3], // 3x3 board
    pub end_slot: u64,
    pub required_amount: u64,
}

// Context of accounts passed to the initialize function
pub struct InitializeCtx {
    pub initializer: Signer,
    pub game_data: GameData, // initialized by the 'initializer'
    pub player_a: SystemAccount,
    pub player_b: SystemAccount,
}

// End point to initialize the game
pub fn initialize(ctx: InitializeCtx, required_amount: u64, delay_slots: u64)  {
    let game_data =  ctx.accounts.game_data;
    game_data.player_a = ctx.accounts.player_a
    game_data.player_b = ctx.accounts.player_b
    game_data.turn_a = true;
    game_data.player_a_has_deposited = false;
    game_data.player_b_has_deposited = false;
    game_data.board = [[Symbol::SymbolEmpty; 3]; 3];
    game_data.end_slot = Clock::current_slot + delay_slots;
    game_data.required_amount = required_amount;
}
```

The `make_move` function is responsible for updating the game state by allowing players to make a move. The function requires the `player` to be the current player, the cell to be empty, and the game to not have timed out. If these conditions are met, the function updates the board, checks if the player has won, and switches the current player.

❗ It's also important to note that it is crucial to check if the `game_data` account is owned by the program and also if the provided `player` has signed the transaction. Othervise, anyone could call the `make_move` function and modify the game state acting like the provided `player`. The [Anchor framework](https://www.anchor-lang.com) makes this check automatically by typifying `player` as `Signer`.

```rust
// Context of accounts passed to the make_move function
pub struct MakeMoveCtx {
    pub player: Signer,
    pub game_data: GameData
    pub system_program: Program<'info, System>,
}

// End point to make a move
pub fn make_move(ctx: MakeMoveCtx, row: u8, col: u8)  {
    let game_data = ctx.accounts.game_data;
    let player = ctx.accounts.player;

    let (current_player, deposited) = if game_data.turn_a {
        (game_data.player_a, game_data.player_a_has_deposited)
    } else {
        (game_data.player_b, game_data.player_b_has_deposited)
    };

    if !deposited {
        // make 'player' deposit 'game_data.required_amount' to 'game_data' account

        if game_data.turn_a {
            game_data.player_a_has_deposited = true;
        } else {
            game_data.player_b_has_deposited = true;
        }
    }

    require!(player.key == current_player, Err::InvalidPlayer);
    require!(Clock::current_slot < game_data.end_slot, Err::TimeoutReached);
    require!(row < 3 && col < 3, Err::InvalidPosition);
    require!(game_data.board[row][col] == Symbol::SymbolEmpty,Err::CellOccupied);

    let player_symbol = if game_data.turn_a { Symbol::SymbolX } else { Symbol::SymbolO};
    game_data.board[row][col] = player_symbol;
    game_data.turn_a = !game_data.turn_a;

    if check_winner(game_data.board) {
        let amount = game_data.required_amount * 2;
        // Transfer 'amount' from 'game_data' account to 'player' account
    }
}

fn check_winner(board: [[Symbol; 3]; 3]) -> bool {
    // You can find the implementation of this function in the full code
}
```

Finally, we need a function to allow players to withdraw their funds if the game times out. This function requires the game to have timed out and the player to not be the current player. If these conditions are met, the function transfers the funds to the `player`.

```rust
// Context of accounts passed to the timeout function
pub struct TimeoutCtx {
    pub player: Signer,
    pub game_data: GameData
    pub system_program: Program<'info, System>,
}

// End point to withdraw funds after timeout
pub fn timeout(ctx: TimeoutCtx)  {
    let game_data = ctx.accounts.game_data;
    let player = ctx.accounts.player;

    let allowed_player = if game_data.turn_a { game_data.player_b } else { game_data.player_a };

    require!(Clock::current_slot >= game_data.end_slot, Err::TimeoutNotReached);

    // Make 'player' withdraw 'game_data.required_amount * 2' from 'game_data' account
}
```
