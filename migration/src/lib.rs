pub use sea_orm_migration::prelude::*;

mod m20240830_132555_init_basic;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240830_132555_init_basic::Migration)]
    }
}
