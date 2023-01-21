use {crate::state::*, anchor_lang::prelude::*};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateConfigEntryIx {
    value: String,
    extends: Vec<Pubkey>,
}

#[derive(Accounts)]
#[instruction(ix: UpdateConfigEntryIx)]
pub struct UpdateConfigEntryCtx<'info> {
    config_entry: Account<'info, ConfigEntry>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateConfigEntryCtx>, ix: UpdateConfigEntryIx) -> Result<()> {
    assert_authority(&ctx.accounts.config_entry.key, ctx.accounts.authority.key(), &mut ctx.remaining_accounts.iter())?;
    let config_entry = &mut ctx.accounts.config_entry;

    let new_config_entry = ConfigEntry {
        bump: config_entry.bump,
        key: config_entry.key.to_string(),
        value: ix.value,
        extends: ix.extends,
    };
    let new_space = new_config_entry.try_to_vec()?.len() + 8;
    config_entry.set_inner(new_config_entry);

    resize_account(
        &config_entry.to_account_info(),
        new_space,
        &ctx.accounts.authority.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
    )?;

    Ok(())
}
