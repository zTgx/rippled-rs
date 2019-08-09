
use crate::core::config::Config;

#[derive(Debug, Default)]
pub struct DataBaseSetup {}
impl DataBaseSetup {
    pub fn new() -> Self {
        DataBaseSetup {

        }
    }

    // Config::StartUpType startUp = Config::NORMAL;
    // bool standAlone = false;
    // boost::filesystem::path dataDir;
}

pub struct DatabaseCon {
    //file path...
    // pub setup: DataBaseSetup,
}
impl DatabaseCon {
    pub fn setup_databaseCon (_c: &Config) {

    }

    // soci::session& getSession()
    // {
    //     return session_;
    // }

    // LockedSociSession checkoutDb ()
    // {
    //     return LockedSociSession (&session_, lock_);
    // }

    pub fn checkout_db() -> bool {
        true
    }
}
