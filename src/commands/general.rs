use std::fmt::Write;
/// External crates brought into scope.
use tinyvec::{TinyVec, tiny_vec};

/// Local crates brought into scopes.
use crate::assets::{
    common::{gen_num, speak},
    equipment::{Armour, Elixir, Tincture, Weapon},
    loot_tables::{ARMOUR_LOOT, ELIXIR_LOOT, TINCTURE_LOOT, WEAPON_LOOT},
};
use crate::fetch_subcommand;
use crate::{Context, Error};

/// Parent command of various subcommands with similar function.
#[poise::command(
    slash_command,
    member_cooldown = 2,
    subcommands(
        "armour",
        "weapon",
        "elixir",
        "generic",
        "coin",
        "condition",
        "tincture"
    )
)]
#[allow(clippy::unused_async)] // False positive.
pub(crate) async fn fetch(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

fetch_subcommand!(
    /// Grab a randomized amount of coins.
    coin
    #[description = "Maximum amount of coins."]
    #[max = 1000_usize]
    limit: Option<usize>
    let => result: usize,
    gen_num(0, limit.unwrap_or(20)).await;
    {}
);

// Test 
fetch_subcommand!(
    /// Grab Armour(s) from a table.
    armour
    #[description = "Maximum amount of rolls."]
    #[max = 10_usize]
    count: Option<usize>
    let mut => armours: TinyVec<[Armour; 20]>,
    tiny_vec!();
    for _ in 0..count.unwrap_or(1) {
        armours.push(ARMOUR_LOOT[gen_num(0, ARMOUR_LOOT.len()).await]);
    }
);

fetch_subcommand!(
    /// Grab Weapon(s) from a table.
    weapon
    #[description = "Maximum amount of rolls."]
    #[max = 10_usize]
    count: Option<usize>
    let mut => weapons: TinyVec<[Weapon; 20]>,
    tiny_vec!();
    for _ in 0..count.unwrap_or(1) {
        weapons.push(WEAPON_LOOT[gen_num(0, WEAPON_LOOT.len()).await]);
    }
);

fetch_subcommand!(
    /// Grab Elixir(s) from a table.
    elixir
    #[description = "Maximum amount of rolls."]
    #[max = 10_usize]
    count: Option<usize>
    let mut => elixirs: TinyVec<[Elixir; 20]>,
    tiny_vec!();
    for _ in 0..count.unwrap_or(1) {
        elixirs.push(ELIXIR_LOOT[gen_num(0, ELIXIR_LOOT.len()).await]);
    }
);

fetch_subcommand!(
    /// Grab Tincture(s) from a table.
    tincture
    #[description = "Maximum amount of rolls."]
    #[max = 10_usize]
    count: Option<usize>
    let mut => tinctures: TinyVec<[Tincture; 20]>,
    tiny_vec!();
    for _ in 0..count.unwrap_or(1) {
        tinctures.push(TINCTURE_LOOT[gen_num(0, TINCTURE_LOOT.len()).await]);
    }
);

/// Grab randomized loot from a table.
#[poise::command(slash_command, member_cooldown = 2)]
pub(crate) async fn generic(
    context: Context<'_>,
    #[description = "Get generic loot from tables with set roll count."]
    #[max = 20_usize]
    count: Option<usize>,
) -> Result<(), Error> {
    let mut loot = String::new();

    for _ in 0..count.unwrap_or(1) {
        let table = gen_num(0, 4).await;
        match table {
            0 => write!(
                loot,
                "{}, ",
                ARMOUR_LOOT[gen_num(0, ARMOUR_LOOT.len()).await]
            )?,
            1 => write!(
                loot,
                "{}, ",
                WEAPON_LOOT[gen_num(0, WEAPON_LOOT.len()).await]
            )?,
            2 => write!(
                loot,
                "{}, ",
                ELIXIR_LOOT[gen_num(0, ELIXIR_LOOT.len()).await]
            )?,
            3 => write!(
                loot,
                "{}, ",
                TINCTURE_LOOT[gen_num(0, TINCTURE_LOOT.len()).await]
            )?,
            _ => write!(loot, "Error")?,
        }
    }

    speak(context, &format!("{loot:#?}")).await;

    Ok(())
}

/// Grab randomized conditions for successful dice roll.
#[poise::command(slash_command, owners_only, member_cooldown = 2)]
pub(crate) async fn condition(
    context: Context<'_>,
    #[description = "If a Threshold should be required. Default is True."] threshold: Option<bool>,
    #[description = "If a Parity should be required. Default is False."] parity: Option<bool>,
    #[description = "Limits lower Threshold amount for result. Default is 0."]
    #[max = 100_usize]
    floor: Option<usize>,
    #[description = "Limits upper Threshold amount for result. Default is 20."]
    #[max = 100_usize]
    ceiling: Option<usize>,
) -> Result<(), Error> {
    let threshold_amount = if floor.unwrap_or(0) > ceiling.unwrap_or(20) {
        speak(
            context,
            "The floor can't be greater than the ceiling, silly!",
        )
        .await;
        gen_num(0, 20).await
    } else {
        gen_num(floor.unwrap_or(0), ceiling.unwrap_or(20)).await
    };

    let parity_type = match gen_num(0, 2).await {
        0 => "even",
        1 => "odd",
        _ => "Error",
    };

    if threshold.unwrap_or(true) && !parity.unwrap_or(false) {
        speak(context, &format!("You must reach/surpass a threshold of **{threshold_amount}**!")).await;
    } else if parity.unwrap_or(false) && !threshold.unwrap_or(true) {
        speak(context, &format!("You must have an **{parity_type}** parity!")).await;
    } else if parity.unwrap_or(false) && threshold.unwrap_or(true) {
        speak(context, &format!(
            "Surpass a threshold of **{threshold_amount}** and have an **{parity_type}** parity!")
        ).await;
    } else {
        speak(context, "You're so silly! I can't do nothing!").await;
    }

    Ok(())
}

/// Lists off available commands for Ayuyan.
#[poise::command(track_edits, slash_command, member_cooldown = 2)]
pub(crate) async fn help(
    context: Context<'_>,
    #[description = "List commands for Ayuyan."]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        context,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "Consider the following...",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Simple command to check to see if Ayuyan is online.
#[poise::command(slash_command, member_cooldown = 2)]
pub(crate) async fn ping(context: Context<'_>) -> Result<(), Error> {
    speak(context, "Pong!").await;
    Ok(())
}
