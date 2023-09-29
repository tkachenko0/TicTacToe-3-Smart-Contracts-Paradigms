# TicTacToe in 3 Smart Contracts Paradigms 🧩

## Introduction 👋

Hello, in this repository, we explore the three fundamental smart contract paradigms: UTXO-based, Account-Based (Stateless and Stateful).

The aim of this project is to provide a comprehensive understanding of these smart contract paradigms and their real-world implications. By scaffolding a familiar game (Tic Tac Toe) in each paradigm, we offer practical insights into the differences, challenges, and advantages of each approach.

## Smart Contract Paradigms

Smart contracts are self-executing, immutable pieces of code that automate and enforce the execution of predefined agreements on a blockchain. These agreements can range from simple transactions to complex decentralized applications (DApps). To better understand the implementations in this project, let's explore three primary smart contract paradigms:

### Account-Based Paradigm

The Account-Based paradigm, exemplified by [Ethereum](https://ethereum.org/en/) and many other blockchain platforms, operates differently from the UTXO model.

In this paradigm:

- **State Management**: accounts maintain balances and can execute code. These accounts can be externally owned accounts, controlled by private keys, or contract accounts, controlled by the code of a smart contract.
- **State Transitions**: contracts are executed by sending transactions to their respective addresses. Contracts can store and modify their own state, which is maintained on the blockchain.
- **Flexibility**: account-based systems are highly flexible and can accommodate complex smart contract logic.

Within the Account-Based paradigm, we can further categorize smart contracts into two main types: Stateless and Stateful contracts.

#### Stateful Contracts

Stateful contracts are contracts that maintain and manage their own state. They can store data, update it based on transactions, and execute complex logic.
Stateful contracts are suitable for applications that require persistent storage and complex business logic, such as games, decentralized finance (DeFi) platforms, and more.

#### Stateless Contracts

Stateless contracts are contracts that do not maintain their own state. They execute deterministic functions based solely on their input parameters and the blockchain's current state that is stored in other accounts and passed to contracts as input.

### UTXO Based Paradigm

The UTXO (Unspent Transaction Output) based paradigm is famously associated with Bitcoin and some other cryptocurrencies.

UTXOs are unspent transaction outputs that are created when a transaction is executed and they are consumed when a new transaction is executed.

In this paradigm each transaction consumes one or more UTXOs and creates one or more new UTXOs and each UTXO can only be consumed once.

UTXO-based systems are very simple. However, they can be less flexible for complex operations.

Understanding these smart contract paradigms is crucial when developing blockchain applications, as they influence how contracts handle state, transitions, and security. In the following sections, we'll explore how each of these paradigms has been applied to our Tic Tac Toe game, providing real-world examples of their capabilities and limitations.

## Tic Tac Toe 🧩

In this section, we provide an overview of the Tic Tac Toe game implemented in three different smart contract paradigms: UTXO-Based, Account-Based (Stateless), and Account-Based (Stateful). Each implementation serves as a practical demonstration of how these paradigms handle state, transitions, and logic execution.

> Don't worry, we won't bore you with a tutorial on how to play Tic Tac Toe – we trust that you've all mastered it during those 'productive' school days when lessons just couldn't compete with this classic game. 😉

### Account Based (Stateful) Implementation

Account-Based smart contracts, when stateful, have the capability to maintain and modify their own internal state.

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

```

```solidity

    function makeMove(uint8 row, uint8 col) external payable  {
        require(msg.sender == currentPlayer, "It's not your turn");
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

### Account Based (Stateless) Implementation

The Account-Based paradigm, in its stateless form, allows for executing smart contract functions without maintaining any internal state.

```rust
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

pub struct InitializeCtx<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        init,
        payer = initializer,
        seeds = [_loby_name.as_ref()],
        bump,
        space = 8 + GameData::INIT_SPACE
    )]
    pub game_data: Account<'info, GameData>,
    pub player_a: SystemAccount<'info>,
    pub player_b: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(
    ctx: Context<InitializeCtx>,
    _loby_name: String,
    required_amount: u64,
    delay_slots: u64,
) -> Result<()> {
    msg!("Initializing game");
    let game_data = &mut ctx.accounts.game_data;
    game_data.player_a = *ctx.accounts.player_a
    game_data.player_b = *ctx.accounts.player_b
    game_data.turn_a = true;
    game_data.player_a_has_deposited = false;
    game_data.player_b_has_deposited = false;
    game_data.board = [[Symbol::SymbolEmpty; 3]; 3];
    game_data.end_slot = Clock::get()?.slot + delay_slots;
    game_data.required_amount = required_amount;
    Ok(())
}
```

```rust
pub struct MakeMoveCtx<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        mut,
        seeds = [_loby_name.as_ref()],
        bump,
    )]
    pub game_data: Account<'info, GameData>,
    pub system_program: Program<'info, System>,
}

pub fn make_move(
    ctx: Context<MakeMoveCtx>,
    _loby_name: String,
    row: u8,
    col: u8,
) -> Result<()> {
    let game_data = &mut ctx.accounts.game_data;
    let player = &ctx.accounts.player;

    let (current_player, deposited) = if game_data.turn_a {
        (game_data.player_a, game_data.player_a_has_deposited)
    } else {
        (game_data.player_b, game_data.player_b_has_deposited)
    };

    if !deposited {
        msg!("Player deposits");
        let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
            &player.key(),
            &game_data.key(),
            game_data.required_amount,
        );

        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[player.to_account_info(), game_data.to_account_info()],
        )
        .unwrap();

        if game_data.turn_a {
            game_data.player_a_has_deposited = true;
        } else {
            game_data.player_b_has_deposited = true;
        }
    }

    require!(player.key == &current_player, CustomError::InvalidPlayer);
    require!(
        Clock::get()?.slot < game_data.end_slot,
        CustomError::TimeoutReached
    );
    require!(row < 3 && col < 3, CustomError::InvalidPosition);
    require!(
        game_data.board[row as usize][col as usize] == Symbol::SymbolEmpty,
        CustomError::CellOccupied
    );

    let player_symbol = if game_data.turn_a {
        Symbol::SymbolX
    } else {
        Symbol::SymbolO
    };
    game_data.board[row as usize][col as usize] = player_symbol;
    game_data.turn_a = !game_data.turn_a;

    if check_winner(game_data.board) {
        msg!("Winner");
        let amount = game_data.required_amount * 2;
        **player.to_account_info().try_borrow_mut_lamports()? += amount;
        **game_data.to_account_info().try_borrow_mut_lamports()? -= amount;
    }

    Ok(())
}
```

```rust
pub struct TimeoutCtx<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        mut,
        seeds = [_loby_name.as_ref()],
        bump,
    )]
    pub game_data: Account<'info, GameData>,
    pub system_program: Program<'info, System>,
}

pub fn timeout(ctx: Context<TimeoutCtx>, _loby_name: String) -> Result<()> {
    let game_data = &mut ctx.accounts.game_data;
    let player = &ctx.accounts.player;

    let allowed_player = if game_data.turn_a {
        game_data.player_b
    } else {
        game_data.player_a
    };

    require!(player.key == &allowed_player, CustomError::InvalidPlayer);
    require!(
        Clock::get()?.slot >= game_data.end_slot,
        CustomError::TimeoutNotReached
    );

    let amount = game_data.required_amount * 2;
    **player.to_account_info().try_borrow_mut_lamports()? += amount;
    **game_data.to_account_info().try_borrow_mut_lamports()? = amount;

    Ok(())
}
```

### UTXO Based Implementation

In the UTXO-Based paradigm, we adapted the Tic Tac Toe game to utilize the unique principles of this model.

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

## Contributing 🙌

We welcome contributions to this project! If you're interested, here is an idea: note that while we have implemented Tic Tac Toe game for [Ethereum](https://ethereum.org/en/) using [Solidity](https://soliditylang.org) (Statefull) and for [Solana](https://solana.com) using [Rust](https://www.rust-lang.org) with the [Anchor framework](https://www.anchor-lang.com) (Stateless), we currently only have a scaffolded pseudocode for the UTXO-based paradigm.

## Acknowledgments 🙏

This project drew inspiration from the speakers and content of the [4th Scientific School on Blockchain & Distributed Ledger Technologies](https://dlt-school.github.io) held at the University of Cagliari.

Here are some of the resources that we found particularly useful:

- [UTXO vs. Account-Based Chains](https://academy.glassnode.com/concepts/utxo#)
- [A Deep Dive into Solana Account Model](https://medium.com/@lianxiongdi/a-deep-dive-into-solana-account-model-1-introduction-7b0408656593)
- [An Introduction to the Solana Account Model](https://www.quicknode.com/guides/solana-development/getting-started/an-introduction-to-the-solana-account-model)

## License 📜

This project is licensed under the [MIT License](LICENSE).
