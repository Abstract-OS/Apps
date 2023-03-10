use crate::contract::{EtfApp, EtfResult};
use crate::msg::{EtfQueryMsg, StateResponse};
use crate::state::{FEE, STATE};
use cosmwasm_std::{to_binary, Binary, Deps, Env};

pub fn query_handler(deps: Deps, _env: Env, _etf: &EtfApp, msg: EtfQueryMsg) -> EtfResult<Binary> {
    match msg {
        EtfQueryMsg::State {} => {
            let fee = FEE.load(deps.storage)?;
            to_binary(&StateResponse {
                liquidity_token: STATE.load(deps.storage)?.liquidity_token_addr.to_string(),
                fee: fee.share(),
            })
        }
    }
    .map_err(Into::into)
}
