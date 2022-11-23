use super::environment::Environment;

/// 環境に応じて.env.*を読み込む。
/// `.env`ファイルは必ず読み込まれる。
///
/// - Production => `.env.production`
/// - Development => `.env.development`
/// - Test => `.env.test`
///
pub fn load_env() -> Result<(), dotenv::Error> {
    match Environment::get_environment() {
        Environment::PRODUCTION => return Ok(()),
        Environment::TEST => dotenv::from_filename(".env.test"),
        Environment::DEVELOPMENT => dotenv::from_filename(".env.development"),
    }?;
    dotenv::dotenv().map(|_| ())
}
