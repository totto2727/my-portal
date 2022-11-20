use std::path::PathBuf;

use super::environment::Environment;

/// 環境に応じて.env.*を読み込む。
/// `.env`ファイルは必ず読み込まれる。
///
/// - Production => `.env.production`
/// - Development => `.env.development`
/// - Test => `.env.test`
///
pub fn load_env() -> Result<PathBuf, dotenv::Error> {
   dotenv::dotenv()?;
    match Environment::get_environment() {
        Environment::TEST => dotenv::from_filename(".env.test"),
        Environment::DEVELOPMENT => dotenv::from_filename(".env.development"),
        Environment::PRODUCTION => dotenv::from_filename(".env.production"),
    }
}
