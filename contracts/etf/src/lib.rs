pub mod contract;
pub mod error;
mod handlers;
pub mod msg;
pub mod response;
pub mod state;

pub const ETF: &str = "abstract:etf";

#[cfg(feature = "boot")]
pub mod boot {
    use crate::msg::*;
    use abstract_boot::AppDeployer;
    use abstract_os::app::MigrateMsg;
    use boot_core::{boot_contract, BootEnvironment, Contract};
    use boot_core::ContractWrapper;

    #[boot_contract(EtfInstantiateMsg, EtfExecuteMsg, EtfQueryMsg, MigrateMsg)]
    pub struct ETF<Chain>;

    impl<Chain: BootEnvironment> AppDeployer<Chain> for ETF<Chain> {}

    impl<Chain: BootEnvironment> ETF<Chain> {
        pub fn new(name: &str, chain: Chain) -> Self {
            let mut contract = Contract::new(name, chain);
            contract = contract.with_wasm_path("etf").with_mock(Box::new(
                ContractWrapper::new_with_empty(
                    crate::contract::execute,
                    crate::contract::instantiate,
                    crate::contract::query,
                ),
            ));
            Self(contract)
        }
    }
}
