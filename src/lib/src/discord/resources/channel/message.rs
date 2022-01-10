use discordrs_codegen::CommandArg;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        abstraction::abstraction_traits::CommandArg,
        interactions::{message::MessageComponent, typing::InteractionType},
    },
    discord::{
        resources::{
            application::Application,
            guild::guild_member::GuildMember,
            sticker::{Sticker, StickerItem},
            user::User,
        },
        snowflake::Snowflake,
    },
};

use super::{
    attachment::Attachment,
    embed::Embed,
    mention::ChannelMention,
    reaction::Reaction,
    typing::{MessageActivity, MessageFlags, MessageReference, MessageType},
    Channel,
};
/**
 * Message Object
 * Represents a message sent in a channel within Discord.
 *
 * Message Structure
 * content, embeds, attachments, and components will require a privileged intent in 2022. Learn more here.
 * @docs https://discord.com/developers/docs/resources/channel#message-object
*/
#[derive(Serialize, Deserialize, Clone, CommandArg)]
pub struct Message {
    /// id of the message
    pub id: Snowflake,
    /// id of the channel the message was sent in
    pub channel_id: Snowflake,
    /// id of the guild the message was sent in
    pub guild_id: Option<Snowflake>,
    /// Author of this message (not guaranteed to be a valid user, see below)
    pub author: Option<User>, // TODO figure out how to make message author a user, or a webhook so that objects are less generic, and users need to account for less
    /// Member properties for this message's author
    pub member: Option<GuildMember>,
    /// Contents of the message
    pub content: String,
    /// When this message was sent
    pub timestamp: String,
    /// When this message was edited (or null if never)
    pub edited_timestamp: Option<String>,
    /// Whether this was a TTS message
    pub tts: bool,
    /// Whether this message mentions everyone
    pub mention_everyone: bool,
    /// Users specifically mentioned in the message
    pub mentions: Vec<User>,
    /// Roles specifically mentioned in this message
    pub mention_roles: Vec<Snowflake>,
    /// Channels specifically mentioned in this message
    pub mention_channels: Option<Vec<ChannelMention>>,
    /// Any attached files
    pub attachments: Vec<Attachment>,
    /// Any embedded content
    pub embeds: Vec<Embed>,
    /// Reactions to the message
    pub reactions: Option<Vec<Reaction>>,
    /// Used for validating a message was sent
    pub nonce: Option<String>,
    /// Whether this message is pinned
    pub pinned: bool,
    /// If the message is generated by a webhook, this is the webhook's id
    pub webhook_id: Option<Snowflake>,
    /// Type of message
    #[serde(rename = "type")]
    pub type_: MessageType,
    /// Sent with Rich Presence-related chat embeds
    pub activity: Option<MessageActivity>,
    /// Sent with Rich Presence-related chat embeds
    pub application: Option<Application>,
    /// If the message is a response to an Interaction, this is the id of the interaction's application
    pub application_id: Option<Snowflake>,
    /// Data showing the source of a crosspost, channel follow add, pin, or reply message
    pub message_reference: Option<MessageReference>,
    /// Message flags combined as a bitfield
    pub flags: MessageFlags,
    /// The message associated with the message_reference
    pub referenced_message: Option<Box<Message>>,
    /// Sent if the message is a response to an Interaction
    pub interaction: Option<Box<MessageInteraction>>,
    /// The thread that was started from this message, includes thread member object
    pub thread: Option<Channel>,
    /// Sent if the message contains components like buttons, action rows, or other interactive components
    pub components: Option<Vec<MessageComponent>>,
    /// Sent if the message contains stickers
    pub sticker_items: Option<Vec<StickerItem>>,
    /// Deprecated the stickers sent with the message
    pub stickers: Option<Vec<Sticker>>,
}

impl Message {
    pub fn is_webhook(&self) -> bool {
        self.webhook_id.is_some()
    }
}

/**
 * Message Interaction Structure
 * @docs https://discord.com/developers/docs/interactions/receiving-and-responding#message-interaction-object-message-interaction-structure
*/
#[derive(Serialize, Deserialize, Clone)]
pub struct MessageInteraction {
    /// id of the interaction
    pub id: Snowflake,
    /// the type of interaction
    #[serde(rename = "type")]
    pub type_: InteractionType,
    /// the name of the application command
    pub name: String,
    /// the user who invoked the interaction
    pub user: User,
}
