pub mod db;
pub mod init;

use crate::error::AppError;
use redis::{self, AsyncCommands};
