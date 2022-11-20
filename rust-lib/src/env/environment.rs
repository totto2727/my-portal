/// ビルド環境を定義するEnum
#[derive(Debug)]
pub enum Environment {
    TEST,
    DEVELOPMENT,
    PRODUCTION,
}

impl Environment {
    /// `cfg!` マクロからビルド環境を取得する関数
    pub fn get_environment() -> Environment {
        if cfg!(test) {
            Environment::TEST
        } else if cfg!(debug_assertions) {
            Environment::DEVELOPMENT
        } else {
            Environment::PRODUCTION
        }
    }
}
