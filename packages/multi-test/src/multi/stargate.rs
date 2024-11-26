//! # Handler for `CosmosMsg::Stargate`, `CosmosMsg::Any`, `QueryRequest::Stargate` and `QueryRequest::Grpc` messages

use crate::{AppResponse, CosmosRouter};
use anybuf::Bufany;
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
        storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        path: String,
        data: Binary,
    ) -> AnyResult<Binary> {
        match path.as_str() {
            "/secret.compute.v1beta1.Query/CodeHashByContractAddress" => {
                let deserialized = Bufany::deserialize(data.as_slice())?;

                if let Some(contract_address) = deserialized.string(1) {
                    let contract = CONTRACTS.load(
                        &prefixed_read(storage, NAMESPACE_WASM),
                        &Addr::unchecked(contract_address),
                    )?;

                    Ok(Binary(contract.code_hash.into()))
                } else {
                    bail!("Failed to decode Protobuf message: String not found for field number 1")
                }
            }
            _ => bail!("Unexpected stargate query: path={}, data={}", path, data),
        }
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

// THIS MAY NOT BE POSSIBLE. I'm assuming the app storage contains the registered contract data.

use super::wasm::ContractData;
use crate::prefixed_storage::prefixed_read;
use secret_storage_plus::Map;

const CONTRACTS: Map<&Addr, ContractData> = Map::new("contracts");
const NAMESPACE_WASM: &[u8] = b"wasm";

pub struct StargateCodeHash;

impl Stargate for StargateCodeHash {
    fn query_stargate(
        &self,
        _api: &dyn Api,
        storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        path: String,
        data: Binary,
    ) -> AnyResult<Binary> {
        match path.as_str() {
            "/secret.compute.v1beta1.Query/CodeHashByContractAddress" => {
                let deserialized = Bufany::deserialize(data.as_slice())?;

                if let Some(contract_address) = deserialized.string(1) {
                    let contract = CONTRACTS.load(
                        &prefixed_read(storage, NAMESPACE_WASM),
                        &Addr::unchecked(contract_address),
                    )?;

                    Ok(Binary(contract.code_hash.into()))
                } else {
                    bail!("Failed to decode Protobuf message: String not found for field number 1")
                }
            }
            _ => bail!("Unexpected stargate query: path={}, data={}", path, data),
        }
    }
}
