pub mod db;
pub mod init;

use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::prelude::*;
