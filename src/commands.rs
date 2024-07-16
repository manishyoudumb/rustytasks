use crate::cli::Command;
use crate::db::Database;
use crate::auth;
use crate::models::Item;

pub async fn execute_command(command: Command) -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new().await?;

    match command {
        Command::Show { all, completed, incomplete, list_name } => {
            show_tasks(&db, all, completed, incomplete, list_name).await?;
        }
        Command::Add { list_name, item } => {
            add_task(&db, &list_name, &item).await?;
        }
        Command::Complete { list_name, item_number } => {
            complete_task(&db, &list_name, item_number).await?;
        }
        Command::Incomplete { list_name, item_number } => {
            incomplete_task(&db, &list_name, item_number).await?;
        }
        Command::Remove { list_name, item_number } => {
            remove_task(&db, list_name, item_number).await?;
        }
        Command::Login => {
            auth::login().await?;
        }
        Command::Logout => {
            auth::logout().await?;
        }
        Command::Push => {
            push_changes(&db).await?;
        }
        Command::Pull => {
            pull_changes(&db).await?;
        }
    }

    Ok(())
}

async fn push_changes(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    db.push_changes().await?;
    println!("Changes pushed successfully");
    Ok(())
}

async fn pull_changes(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    db.pull_changes().await?;
    println!("Changes pulled successfully");
    Ok(())
}

async fn show_tasks(
    db: &Database,
    all: bool,
    completed: bool,
    incomplete: bool,
    list_name: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let lists = if let Some(name) = list_name {
        vec![db.get_list(&name).await?]
    } else {
        db.get_lists().await?
    };

    for list in lists {
        println!("List: {}", list.name);
        for (i, item) in list.items.iter().enumerate() {
            if (all || (!completed && !incomplete)) ||
               (completed && item.completed) ||
               (incomplete && !item.completed) {
                println!("  {}. [{}] {}", i + 1, if item.completed { "x" } else { " " }, item.description);
            }
        }
        println!();
    }

    Ok(())
}

async fn add_task(db: &Database, list_name: &str, item_description: &str) -> Result<(), Box<dyn std::error::Error>> {
    let item = Item {
        description: item_description.to_string(),
        completed: false,
    };
    db.add_item(list_name, item).await?;
    println!("Task added to list '{}'", list_name);
    Ok(())
}

async fn complete_task(db: &Database, list_name: &str, item_number: usize) -> Result<(), Box<dyn std::error::Error>> {
    db.update_item_status(list_name, item_number, true).await?;
    println!("Task {} in list '{}' marked as completed", item_number, list_name);
    Ok(())
}

async fn incomplete_task(db: &Database, list_name: &str, item_number: usize) -> Result<(), Box<dyn std::error::Error>> {
    db.update_item_status(list_name, item_number, false).await?;
    println!("Task {} in list '{}' marked as incomplete", item_number, list_name);
    Ok(())
}

async fn remove_task(
    db: &Database,
    list_name: Option<String>,
    item_number: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    match (list_name, item_number) {
        (Some(list), Some(item)) => {
            db.remove_item(&list, item).await?;
            println!("Task {} removed from list '{}'", item, list);
        }
        (Some(list), None) => {
            db.remove_list(&list).await?;
            println!("List '{}' removed", list);
        }
        (None, None) => {
            db.remove_all_lists().await?;
            println!("All lists removed");
        }
        _ => println!("Invalid combination of arguments"),
    }
    Ok(())
}