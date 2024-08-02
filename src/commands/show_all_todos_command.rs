use serenity::all::{CommandInteraction, CreateEmbed};
use serenity::async_trait;
use crate::commands::CommandTrait;
use crate::database::team_repo::TeamRepo;
use crate::database::todo_repo::TodoRepo;
use crate::entity::todo::{Todo, VecTodoExtension};
use crate::global::discord::Discord;
use crate::util::colour::GREEN;

pub struct ShowAllTodosCommand;

#[async_trait]
impl CommandTrait for ShowAllTodosCommand {
    async fn run(discord: &Discord, _command: &CommandInteraction) -> serenity::Result<Option<CreateEmbed>> {
        let team_repo = TeamRepo::new(discord);
        let todo_repo = TodoRepo::new(discord);

        let teams = team_repo.get_teams().await?;
        let todos = todo_repo.get_todos().await?;
        let mut message = String::new();
        for team in teams {
            let todos: Vec<Todo> = todos.iter().filter(|todo| todo.team.name == team.name).cloned().collect();
            let mut m = todos.message(&team.name);
            m.push_str("\n\n");
            message.push_str(m.as_str());
        }
        println!("{}", message);
        let create_embed = CreateEmbed::new()
            .description(message)
            .color(GREEN);
        Ok(Some(create_embed))
    }
}