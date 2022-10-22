use cosmwasm_std::{entry_point, to_binary, Addr};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, JumpRingCheckResponse, SapienceResponse};
use crate::state::{config, config_read, State};
use crate::species::{Traveler, SapienceScale, Sapient};

/////////////////////////////////////////////////////////////////////////////////////////
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::JumpRingPreCheck { traveler } => jumpring_check(traveler),
        QueryMsg::MinimumSapience {} => minimum_sapience(deps),
    }
}

pub fn minimum_sapience(deps: Deps) -> StdResult<Binary> {
    let state = config_read(deps.storage).load()?;
    let out = to_binary(&SapienceResponse {
        level: state.minimum_sapience,
    })?;
    Ok(out)
}

pub fn jumpring_check(traveler: Traveler) -> StdResult<Binary> {
    let out = to_binary(&JumpRingCheckResponse {
        valid: traveler.cyberdized,
    })?;
    Ok(out)
}

/////////////////////////////////////////////////////////////////////////////////////////
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPlanetName { to } => set_planet_name(to, deps, info),
        ExecuteMsg::SetSapientNames { to } => set_sapient_names(to, deps, info),
        ExecuteMsg::SetMinimumSapience { to } => set_minimum_sapience(to, deps, info),
        ExecuteMsg::JumpRingTravel { to } => initiate_jumpring_travel(to, deps, info),
    }
}

pub fn initiate_jumpring_travel(
    _to: Addr,
    _deps: DepsMut,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn set_minimum_sapience(
    to: SapienceScale,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut state : State = config(deps.storage).load()?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    state.minimum_sapience = to;
    config(deps.storage).save(&state)?;
    Ok(Response::default())
}

pub fn set_planet_name(
    to: String,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {

    let mut state : State= config(deps.storage).load()?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {})
    }

    state.planet_name = to;
    // state.planet_name = to.clone();   // In case we want the state to have its own copy, retaining to its ownership
    config(deps.storage).save(&state)?;
    Ok(Response::new().add_attribute("action", "set_planet_name"))


}

pub fn set_sapient_names(
    to: Vec<Sapient>,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut state = config(deps.storage).load()?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }

    state.planet_sapients = to;
    config(deps.storage).save(&state)?;
    Ok(Response::new().add_attribute("action", "set_sapient_names"))
}

/////////////////////////////////////////////////////////////////////////////////////////
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender,
        planet_name: msg.planet_name,
        planet_sapients: msg.planet_sapients,
        minimum_sapience: msg.minimum_sapience,
    };
    config(deps.storage).save(&state)?;
    Ok(Response::new()
        .add_attribute("owner", state.owner)
        .add_attribute("minimum_sapience", state.minimum_sapience.as_str()))
}

