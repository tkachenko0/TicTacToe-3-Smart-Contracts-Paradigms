use anchor_lang::prelude::*;

declare_id!("Enq7EUNZshTBsTMDzimmv4dv1wC8jaZmm7wgrSH4UPsj");

#[program]
pub mod tic_tac_toe_anchor {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeCtx>,
        _loby_name: String,
        required_amount: u64,
        delay_slots: u64,
    ) -> Result<()> {
        msg!("Initializing game");
        let game_data = &mut ctx.accounts.game_data;
        game_data.player_a = *ctx.accounts.player_a.to_account_info().key;
        game_data.player_b = *ctx.accounts.player_b.to_account_info().key;
        game_data.turn_a = true;
        game_data.player_a_has_deposited = false;
        game_data.player_b_has_deposited = false;
        game_data.board = [[Symbol::SymbolEmpty; 3]; 3];
        game_data.end_slot = Clock::get()?.slot + delay_slots;
        game_data.required_amount = required_amount;
        Ok(())
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
}

fn check_winner(board: [[Symbol; 3]; 3]) -> bool {
    // Check rows and columns
    for i in 0..3 {
        if board[i][0] != Symbol::SymbolEmpty
            && board[i][0] == board[i][1]
            && board[i][0] == board[i][2]
        {
            return true;
        }
        if board[0][i] != Symbol::SymbolEmpty
            && board[0][i] == board[1][i]
            && board[0][i] == board[2][i]
        {
            return true;
        }
    }
    // Check diagonals
    if board[0][0] != Symbol::SymbolEmpty
        && board[0][0] == board[1][1]
        && board[0][0] == board[2][2]
    {
        return true;
    }
    if board[0][2] != Symbol::SymbolEmpty
        && board[0][2] == board[1][1]
        && board[0][2] == board[2][0]
    {
        return true;
    }
    return false;
}

#[derive(
    InitSpace, borsh::BorshSerialize, borsh::BorshDeserialize, Clone, Debug, PartialEq, Eq, Copy,
)]
pub enum Symbol {
    SymbolX,
    SymbolO,
    SymbolEmpty,
}

#[account]
#[derive(InitSpace)]
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

#[derive(Accounts)]
#[instruction(_loby_name: String)]
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

#[derive(Accounts)]
#[instruction(_loby_name: String)]
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

#[derive(Accounts)]
#[instruction(_loby_name: String)]
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

#[error_code]
pub enum CustomError {
    #[msg("Invalid Player")]
    InvalidPlayer,

    #[msg("Invalid position, must be between 0 and 2")]
    InvalidPosition,

    #[msg("Cell is already occupied")]
    CellOccupied,

    #[msg("Invalid amount, must be greater than 0")]
    InvalidAmount,

    #[msg("The timeout slot was reached")]
    TimeoutReached,

    #[msg("The timeout slot was not reached")]
    TimeoutNotReached,
}
