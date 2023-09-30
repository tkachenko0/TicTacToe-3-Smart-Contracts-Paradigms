---
sidebar_position: 2
---

# Stateless

The Account-Based paradigm, in its stateless form, allows for executing smart contract functions without maintaining any internal state.

You can see the provided pseudocode implementation by opening the collapsed section below.

:::info
The provided code snippets are pseudocode representations. 
You can find the actual in [Account Based (Stateless) Implementation in Anchor for Solana](Stateless)
with:
- [Anchor on-chain code](Stateless/tic-tac-toe-anchor/programs/tic-tac-toe-anchor/src/lib.rs)
- [TypeScript client code](Stateless/tic-tac-toe-anchor/tests/test_tic_tac_toe.ts)
:::

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
    pub board: [[Symbol; 3]; 3],
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
    require!(game_data.board[row as usize][col as usize] == Symbol::SymbolEmpty,Err::CellOccupied);

    let player_symbol = if game_data.turn_a { Symbol::SymbolX } else { Symbol::SymbolO};
    game_data.board[row as usize][col as usize] = player_symbol;
    game_data.turn_a = !game_data.turn_a;

    if check_winner(game_data.board) {
        msg!("Winner");
        let amount = game_data.required_amount * 2;
        **player.to_account_info().try_borrow_mut_lamports()? += amount;
        **game_data.to_account_info().try_borrow_mut_lamports()? -= amount;
    }
}

fn check_winner(board: [[Symbol; 3]; 3]) -> bool {
    // You can find the implementation of this function in the full code
}
```

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

    require!(player.key == &allowed_player, Err::InvalidPlayer);
    require!(Clock::current_slot >= game_data.end_slot, Err::TimeoutNotReached);

    // Make 'player' withdraw 'game_data.required_amount * 2' from 'game_data' account
}
```
