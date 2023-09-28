use anchor_lang::prelude::*;

declare_id!("Enq7EUNZshTBsTMDzimmv4dv1wC8jaZmm7wgrSH4UPsj");

#[program]
pub mod tic_tac_toe_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
