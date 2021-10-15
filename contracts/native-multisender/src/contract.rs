#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Addr, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, SubMsg,
};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryMsg,
};
use crate::state::{TransferInfo};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:token-multi-sender";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // no setup
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer { transfer_list } => execute_transfer(transfer_list),
    }
}

pub fn execute_transfer(
    transfer_list: Vec<TransferInfo>,
) -> Result<Response, ContractError> {
    let messages: Vec<SubMsg> = send_tokens(transfer_list)?;

    Ok(Response::new()
        .add_attribute("action", "transfer")
        .add_submessages(messages))
}

fn send_tokens(transfer_list: Vec<TransferInfo>) -> StdResult<Vec<SubMsg>> {
    let mut msgs: Vec<SubMsg> = vec![];
    for transfer_info in transfer_list {
        let mut msg = vec![SubMsg::new(BankMsg::Send {
            to_address: transfer_info.recipient.into(),
            amount: transfer_info.native,
        })];
        msgs.append(&mut msg);
    }
    Ok(msgs)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {

    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins};

    use super::*;

    #[test]
    fn test_multi_send() {
        let mut deps = mock_dependencies(&[]);

        // Instantiate an empty contract
        let instantiate_msg = InstantiateMsg {};
        let info = mock_info(&String::from("anyone"), &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Transfer some recipients
        let transfer_list = vec![
            TransferInfo {
                recipient: Addr::unchecked("recp1"),
                native: coins(100, "tokens"),
            },
            TransferInfo {
                recipient: Addr::unchecked("recp2"),
                native: coins(200, "tokens"),
            },
        ];
        let sender = String::from("source");
        let balance = coins(300, "tokens");
        let info = mock_info(&sender, &balance);
        let msg = ExecuteMsg::Transfer { transfer_list };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(2, res.messages.len());
        assert_eq!(("action", "transfer"), res.attributes[0]);
    }
}
