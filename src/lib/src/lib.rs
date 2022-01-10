pub mod core;
mod discord;
mod util;

pub use crate::core::abstraction::{
    abstraction_traits::{CommandHandler, EventHandler, Registerable},
    bot::Bot,
    context::Context,
    event_dispatcher::{EventDispatcher, Events},
    interaction_router::InteractionRouter,
};

pub use discordrs_codegen::*;

/**
 * For internal use only. Is public because it is used by the proc macros
 */
pub mod __internal__ {
    pub use crate::core::abstraction::abstraction_traits::InternalEventHandler;
}

/**
 * Common Resources that will often be used
 */
pub mod resources {
    pub use crate::discord::resources::channel::message::Message;
    pub use crate::discord::resources::channel::Channel;
    pub use crate::discord::resources::*;
}

/**
 * Objects associated with different events
 */
pub mod command_args {
    pub use crate::core::interactions::handler::events::dispatch_payloads::*;
    pub use crate::core::interactions::{
        interaction_event::InteractionCreate, typing::Interaction,
    };
    pub use crate::discord::gateway::presence::PresenceUpdate;
    pub use crate::resources::guild::guild_object::{Guild, UnavailableGuild};
    pub use crate::resources::guild::stage_instance::StageInstance;
    pub use crate::resources::guild_scheduled_event::GuildScheduledEvent;
    pub use crate::resources::user::User;
    pub use crate::resources::voice::VoiceState;
    pub use crate::resources::{Channel, Message};
}

pub use discord::interactions::application_command::{ApplicationCommand, ApplicationCommandType};

pub use discord::permissions::Permissions;
pub use discord::snowflake::Snowflake;

pub(crate) static BASE_URL: &str = "https://discord.com/api/v9";
