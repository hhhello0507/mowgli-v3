use std::io::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use serenity::{
    all::Message,
    async_trait,
    gateway::ActivityData,
    model::{
        application::{Command, Interaction},
        gateway::Ready,
    },
    prelude::*,
    utils::MessageBuilder,
};
use serenity::all::{CommandInteraction, ComponentInteraction};
use crate::commands::CommandTrait;
use crate::commands::create_todo_command::CreateTodoCommand;
use crate::commands::get_todos_command::GetTodosCommand;
use crate::commands::not_found_command::NotFoundCommand;
use crate::commands::reset_todos_command::ResetTodosCommand;
use crate::component::ComponentTrait;
use crate::component::create_todo_component::CreateTodoComponent;
use crate::component::not_found::NotFountComponent;
use crate::config::config::Config;
use crate::global::discord::Discord;
use crate::util::create_embed_extension::{ResultCreateEmbed, CreateEmbedExtension};
use crate::util::create_interaction_response_extension::create_response;

mod commands;
mod database;
mod util;
mod entity;
mod config;
mod component;
mod global;

struct Handler;

// 욕설 리스트
static ARR: &[&str] = &[
    "ㅅㅂ",
    "시발",
    "병신",
    "ㅂㅅ",
    "장애",
    "새끼"
];

// impl for interaction_create fn
impl Handler {
    async fn handle_command_interaction(&self, ctx: &Context, command: &CommandInteraction) {
        // create discord
        let guild_id = match command.guild_id.ok_or_else(|| Error::other("guild id를 찾을 수 없습니다")) {
            Ok(v) => v,
            _ => return
        };
        let discord = Discord::new(ctx, &guild_id);

        // handle command
        let command_name = command.data.name.as_str();
        let result = match command_name {
            "할일" => GetTodosCommand::run(&discord, command).await,
            "할일초기화" => ResetTodosCommand::run(&discord, command).await,
            "할일추가" => CreateTodoCommand::run(&discord, command).await,
            _ => NotFoundCommand::run(&discord, command).await
        };

        // handle result
        if let Some(create_embed) = result.create_embed() {
            let builder = create_response(create_embed);
            if let Err(why) = command.create_response(&ctx.http, builder).await {
                println!("API resposne 에러 발생 - {}", why);
            };
        }
    }

    async fn handle_component_interaction(&self, ctx: &Context, component: &ComponentInteraction) {

        // validation
        let message_interaction = match &component.message.interaction {
            Some(v) => v,
            _ => {
                print!("message interaction 이 외에 핸들링 필요");
                return;
            }
        };
        // create discord
        let guild_id = match component.guild_id.ok_or_else(|| Error::other("guild id를 찾을 수 없습니다")) {
            Ok(v) => v,
            _ => return
        };
        let discord = Discord::new(ctx, &guild_id);

        // handle message interaction
        let interaction_name = message_interaction.name.as_str();
        let result = match interaction_name {
            "할일추가" => CreateTodoComponent::run(&discord, component).await,
            _ => NotFountComponent::run(&discord, component).await
        };

        // handle result
        if let Some(create_embed) = result.create_embed() {
            let builder = create_response(create_embed);
            if let Err(why) = component.create_response(&ctx.http, builder).await {
                println!("API resposne 에러 발생 - {}", why);
            };
        }
    }
}

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

        // set activity
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        ctx.set_activity(Some(ActivityData::playing(format!("{}", now))));

        // register command
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
            _ => return,
        };
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