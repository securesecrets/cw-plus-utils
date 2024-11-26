//! # Handler for `CosmosMsg::Stargate`, `CosmosMsg::Any`, `QueryRequest::Stargate` and `QueryRequest::Grpc` messages

use crate::{AppResponse, CosmosRouter};
use anyhow::bail;
use anyhow::Result as AnyResult;
use cosmwasm_std::to_binary;
use cosmwasm_std::{Addr, Api, Binary, BlockInfo, CustomMsg, CustomQuery, Empty, Querier, Storage};
use serde::de::DeserializeOwned;

/// Interface of handlers for processing `Stargate` message variants
pub trait Stargate {
    /// Processes `CosmosMsg::Stargate` message variant.
    fn execute_stargate<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        sender: Addr,
        type_url: String,
        value: Binary,
    ) -> AnyResult<AppResponse>
    where
        ExecC: CustomMsg + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        bail!(
            "Unexpected stargate execute: type_url={}, value={} from {}",
            type_url,
            value,
            sender,
        )
    }

    /// Processes `QueryRequest::Stargate` query.
    fn query_stargate(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        path: String,
        data: Binary,
    ) -> AnyResult<Binary> {
        bail!("Unexpected stargate query: path={}, data={}", path, data)
    }
}

/// Always failing handler for `Stargate` message variants and `Stargate` queries.
pub struct StargateFailing;

impl Stargate for StargateFailing {}

/// Always accepting handler for `Stargate` message variants and `Stargate` queries.
pub struct StargateAccepting;

impl Stargate for StargateAccepting {
    fn execute_stargate<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _sender: Addr,
        _type_url: String,
        _value: Binary,
    ) -> AnyResult<AppResponse>
    where
        ExecC: CustomMsg + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        Ok(AppResponse::default())
    }

    fn query_stargate(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        _path: String,
        _data: Binary,
    ) -> AnyResult<Binary> {
        to_binary(&Empty {}).map_err(Into::into)
    }
}
