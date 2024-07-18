use crate::cli::Command;
use crate::db::Database;
use crate::models::Item;
use crate::error::TodoResult;
use crate::auth;
use crate::sync;

pub async fn execute_command(command: Command) -> TodoResult<()> {
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
        Command::Push => {
            sync::push(&db).await?;
        }
        Command::Pull => {
            sync::pull(&db).await?;
        }
        Command::Login => {
            auth::login().await?;
        }
        Command::Logout => {
            auth::logout().await?;
        }
    }

    Ok(())
}

async fn show_tasks(db: &Database, all: bool, completed: bool, incomplete: bool, list_name: Option<String>) -> TodoResult<()> {
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

async fn add_task(db: &Database, list_name: &str, item_description: &str) -> TodoResult<()> {
    // Check if the list exists, if not, create it
    if db.get_list(list_name).await.is_err() {
        db.create_list(list_name).await?;
        println!("Created new list '{}'", list_name);
    }

    let item = Item {
        description: item_description.to_string(),
        completed: false,
    };
    db.add_item(list_name, item).await?;
    println!("Task added to list '{}'", list_name);
    Ok(())
}

async fn complete_task(db: &Database, list_name: &str, item_number: usize) -> TodoResult<()> {
    db.update_item_status(list_name, item_number, true).await?;
    println!("Task {} in list '{}' marked as completed", item_number, list_name);
    Ok(())
}

async fn incomplete_task(db: &Database, list_name: &str, item_number: usize) -> TodoResult<()> {
    db.update_item_status(list_name, item_number, false).await?;
    println!("Task {} in list '{}' marked as incomplete", item_number, list_name);
    Ok(())
}

async fn remove_task(db: &Database, list_name: Option<String>, item_number: Option<usize>) -> TodoResult<()> {
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::db::Database;
    

//     #[tokio::test]
//     async fn test_add_task() {
//         let db = Database::new().await.unwrap();
//         let list_name = "Test List";
//         let item_description = "Buy milk";

//         let result = add_task(&db, list_name, item_description).await;

//         assert!(result.is_ok());

//         let list = db.get_list(list_name).await.unwrap();
//         assert_eq!(list.items.len(), 1);
//         assert_eq!(list.items[0].description, item_description);
//         assert_eq!(list.items[0].completed, false);
//     }
// }
