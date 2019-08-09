use crate::core::database_con::{DatabaseCon, DataBaseSetup};
pub struct Application {}
impl Application {
    // pub fn get_txnDB() -> DataBaseCon {
    //     assert (mTxnDB.get() != nullptr);
    //     return *mTxnDB;
    // }
    // DatabaseCon& getLedgerDB () override
    // {
    //     assert (mLedgerDB.get() != nullptr);
    //     return *mLedgerDB;
    // }
    // DatabaseCon& getWalletDB () override
    // {
    //     assert (mWalletDB.get() != nullptr);
    //     return *mWalletDB;
    // }

    // bool initSqliteDbs ()
    // {
    //     assert (mTxnDB.get () == nullptr);
    //     assert (mLedgerDB.get () == nullptr);
    //     assert (mWalletDB.get () == nullptr);
    //
    //     DatabaseCon::Setup setup = setup_DatabaseCon (*config_);
    //     mTxnDB = std::make_unique <DatabaseCon> (setup, TxnDBName,
    //             TxnDBInit, TxnDBCount);
    //     mLedgerDB = std::make_unique <DatabaseCon> (setup, "ledger.db",
    //             LedgerDBInit, LedgerDBCount);
    //     mWalletDB = std::make_unique <DatabaseCon> (setup, "wallet.db",
    //             WalletDBInit, WalletDBCount);
    //
    //     return
    //         mTxnDB.get () != nullptr &&
    //         mLedgerDB.get () != nullptr &&
    //         mWalletDB.get () != nullptr;
    // }

}
