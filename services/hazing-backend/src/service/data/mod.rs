use crate::config::Config;
use crate::MigrateMode;
use sqlx::PgPool;

pub trait Migration {
    fn migrate_table(&self) -> String;
    fn drop_table(&self) -> String;
}

pub async fn migrate(config: &Config, mode: MigrateMode) {
    let table_list: [Box<dyn Migration>; 0] = [];

    let mut sql: Vec<String> = Vec::new();
    match mode {
        MigrateMode::Up => {
            for table in table_list {
                sql.push(table.migrate_table())
            }
        }
        MigrateMode::Down => {
            for table in table_list {
                sql.push(table.drop_table())
            }
        }
    }

    sqlx::query(sql.join("; ").as_str())
        .execute(
            &PgPool::connect(config.database.address.as_str())
                .await
                .expect("failed connect to postgres database"),
        )
        .await
        .expect("failed to migrate database");
}
