use std::env;
use std::io::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use dotenv::dotenv;
use serenity::{
    all::Message,
    async_trait,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    gateway::ActivityData,
    model::{
        application::{Command, Interaction},
        gateway::Ready,
    },
    prelude::*,
    utils::MessageBuilder,
};
use serenity::all::{CommandInteraction, ComponentInteraction, CreateEmbed, Integration};
use serenity::all::Opcode::Heartbeat;
use crate::commands::CommandTrait;
use crate::commands::create_todo::CreateTodoCommand;
use crate::commands::get_todos::GetTodosCommand;
use crate::commands::not_found::NotFoundCommand;
use crate::commands::reset_todos::ResetTodosCommand;
use crate::component::ComponentTrait;
use crate::component::create_todo::CreateTodoComponent;
use crate::component::not_found::NotFountComponent;
use crate::config::config::Config;
use crate::util::error::{ResultCreateEmbed, UnknownCreateEmbed};

mod commands;
mod service;
mod database;
mod util;
mod entity;
mod config;
mod component;

struct Handler;

static ARR: &[&str] = &[
    "ㅅㅂ",
    "시발",
    "병신",
    "ㅂㅅ",
    "장애",
    "새끼"
];

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let content = msg.content;
        if ARR.iter().any(|&i| content.contains(i)) {
            let response = MessageBuilder::new()
                .push(format!("{}님 ", msg.author.mention()))
                .push_bold_safe("올바른 언어 습관")
                .push("을 들입시다")
                .build();
            _ = msg.channel_id.say(&ctx.http, &response).await;
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} 봇 실행 완료!", ready.user.name);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        ctx.set_activity(Some(ActivityData::playing(format!("{}", now))));
        Command::set_global_commands(&ctx.http, vec![
            GetTodosCommand::register().await,
            ResetTodosCommand::register().await,
            CreateTodoCommand::register().await,
        ])
            .await
            .expect("명령 생성에 실패했습니다.");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => self.handle_command_interaction(&ctx, &command).await,
            Interaction::Component(component) => self.handle_component_interaction(&ctx, &component).await,
                _ => {}
        };
    }
}

impl Handler {
    async fn handle_command_interaction(&self, ctx: &Context, command: &CommandInteraction) {
        let command_name = command.data.name.as_str();
        let result = match command_name {
            "할일" => GetTodosCommand::run(ctx, command).await,
            "할일초기화" => ResetTodosCommand::run(ctx, command).await,
            "할일추가" => CreateTodoCommand::run(ctx, command).await,
            _ => NotFoundCommand::run(ctx, command).await
        };
        if let Err(why) = result {
            println!("API 에러 발생 - {}", why);
        }
    }

    async fn handle_component_interaction(&self, ctx: &Context, component: &ComponentInteraction) {
        // println!("{:#?}", component);
        let message_interaction = match &component.message.interaction {
            Some(v) => v,
            _ => return
        };
        let interaction_name = message_interaction.name.as_str();
        let result = match interaction_name {
            "할일추가" => CreateTodoComponent::run(ctx, component).await,
            _ => NotFountComponent::run(ctx, component).await
        };
        if let Err(why) = result {
            println!("API 에러 발생 - {}", why);
        }
    }
}


#[tokio::main]
async fn main() {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES;

    let config = Config::new();
    let mut client = Client::builder(config.discord_bot_token, intents)
        .event_handler(Handler)
        .await
        .expect("클라이언트 생성에 실패했습니다.");

    if let Err(why) = client.start().await {
        println!("클라이언트 오류가 발생했습니다: {why}");
    }
}


/*
ComponentInteraction {
    id: InteractionId(
        1268605177120686081,
    ),
    application_id: ApplicationId(
        1268146621896720464,
    ),
    data: ComponentInteractionData {
        custom_id: "Web",
        kind: Button,
    },
    guild_id: Some(
        GuildId(
            1243554058221129838,
        ),
    ),
    channel: Some(
        PartialChannel {
            id: ChannelId(
                1243554058808197193,
            ),
            name: Some(
                "일반",
            ),
            kind: Text,
            permissions: Some(
                Permissions(
                    985162418487295,
                ),
            ),
            thread_metadata: None,
            parent_id: Some(
                ChannelId(
                    1243554058808197191,
                ),
            ),
        },
    ),
    channel_id: ChannelId(
        1243554058808197193,
    ),
    member: Some(
        Member {
            user: User {
                id: UserId(
                    805819594253533245,
                ),
                name: "hhhello0507",
                discriminator: None,
                global_name: Some(
                    "이강현",
                ),
                avatar: Some(
                    "f80ee2bea03fd808e067f6820148c994",
                ),
                bot: false,
                system: false,
                mfa_enabled: false,
                banner: None,
                accent_colour: None,
                locale: None,
                verified: None,
                email: None,
                flags: UserPublicFlags(
                    0,
                ),
                premium_type: None,
                public_flags: Some(
                    UserPublicFlags(
                        0,
                    ),
                ),
                member: None,
            },
            nick: None,
            avatar: None,
            roles: [],
            joined_at: Some(
                Timestamp(
                    2024-05-24T13:19:51.645Z,
                ),
            ),
            premium_since: None,
            deaf: false,
            mute: false,
            flags: GuildMemberFlags(
                0,
            ),
            pending: false,
            permissions: Some(
                Permissions(
                    985162418487295,
                ),
            ),
            communication_disabled_until: None,
            guild_id: GuildId(
                1243554058221129838,
            ),
            unusual_dm_activity_until: None,
        },
    ),
    user: User {
        id: UserId(
            805819594253533245,
        ),
        name: "hhhello0507",
        discriminator: None,
        global_name: Some(
            "이강현",
        ),
        avatar: Some(
            "f80ee2bea03fd808e067f6820148c994",
        ),
        bot: false,
        system: false,
        mfa_enabled: false,
        banner: None,
        accent_colour: None,
        locale: None,
        verified: None,
        email: None,
        flags: UserPublicFlags(
            0,
        ),
        premium_type: None,
        public_flags: Some(
            UserPublicFlags(
                0,
            ),
        ),
        member: None,
    },
    token: "aW50ZXJhY3Rpb246MTI2ODYwNTE3NzEyMDY4NjA4MTpHOU9NZEI4UTBRdDBGZFIwWUR2S1IzempSWklmNHZVTDdyMnhobTRlZmprUXVRcHRvYjZzazdiYXNUR3RWWlhvUVVvclJYWGk0RWh3M0ZScGIyejQ4VXppVEVJNThZMVQyb01MNE13blZnTWRJbmJRckxvZXl5SG1UUGNwS3BiRw",
    version: 1,
    message: Message {
        id: MessageId(
            1268603263864012831,
        ),
        channel_id: ChannelId(
            1243554058808197193,
        ),
        author: User {
            id: UserId(
                1268146621896720464,
            ),
            name: "모글리v3",
            discriminator: Some(
                5773,
            ),
            global_name: None,
            avatar: Some(
                "39aea05846c8fb7b6f019c2c2b2d5e60",
            ),
            bot: true,
            system: false,
            mfa_enabled: false,
            banner: None,
            accent_colour: None,
            locale: None,
            verified: None,
            email: None,
            flags: UserPublicFlags(
                0,
            ),
            premium_type: None,
            public_flags: Some(
                UserPublicFlags(
                    0,
                ),
            ),
            member: None,
        },
        content: "팀을 알려주세요!",
        timestamp: Timestamp(
            2024-08-01T16:16:27.614Z,
        ),
        edited_timestamp: None,
        tts: false,
        mention_everyone: false,
        mentions: [],
        mention_roles: [],
        mention_channels: [],
        attachments: [],
        embeds: [],
        reactions: [],
        nonce: None,
        pinned: false,
        webhook_id: Some(
            WebhookId(
                1268146621896720464,
            ),
        ),
        kind: ChatInputCommand,
        activity: None,
        application: None,
        application_id: Some(
            ApplicationId(
                1268146621896720464,
            ),
        ),
        message_reference: None,
        flags: Some(
            MessageFlags(
                0,
            ),
        ),
        referenced_message: None,
        interaction: Some(
            MessageInteraction {
                id: InteractionId(
                    1268603256922443888,
                ),
                kind: Command,
                name: "할일추가",
                user: User {
                    id: UserId(
                        805819594253533245,
                    ),
                    name: "hhhello0507",
                    discriminator: None,
                    global_name: Some(
                        "이강현",
                    ),
                    avatar: Some(
                        "f80ee2bea03fd808e067f6820148c994",
                    ),
                    bot: false,
                    system: false,
                    mfa_enabled: false,
                    banner: None,
                    accent_colour: None,
                    locale: None,
                    verified: None,
                    email: None,
                    flags: UserPublicFlags(
                        0,
                    ),
                    premium_type: None,
                    public_flags: Some(
                        UserPublicFlags(
                            0,
                        ),
                    ),
                    member: None,
                },
                member: None,
            },
        ),
        thread: None,
        components: [
            ActionRow {
                kind: ActionRow,
                components: [
                    Button(
                        Button {
                            kind: Button,
                            data: NonLink {
                                custom_id: "iOS",
                                style: Secondary,
                            },
                            label: Some(
                                "iOS",
                            ),
                            emoji: None,
                            disabled: false,
                        },
                    ),
                    Button(
                        Button {
                            kind: Button,
                            data: NonLink {
                                custom_id: "Android",
                                style: Secondary,
                            },
                            label: Some(
                                "Android",
                            ),
                            emoji: None,
                            disabled: false,
                        },
                    ),
                    Button(
                        Button {
                            kind: Button,
                            data: NonLink {
                                custom_id: "Web",
                                style: Secondary,
                            },
                            label: Some(
                                "Web",
                            ),
                            emoji: None,
                            disabled: false,
                        },
                    ),
                    Button(
                        Button {
                            kind: Button,
                            data: NonLink {
                                custom_id: "Server",
                                style: Secondary,
                            },
                            label: Some(
                                "Server",
                            ),
                            emoji: None,
                            disabled: false,
                        },
                    ),
                ],
            },
        ],
        sticker_items: [],
        position: Some(
            0,
        ),
        role_subscription_data: None,
        guild_id: None,
        member: None,
        poll: None,
    },
    app_permissions: Some(
        Permissions(
            985162418487295,
        ),
    ),
    locale: "ko",
    guild_locale: Some(
        "en-US",
    ),
    entitlements: [],
}


 */