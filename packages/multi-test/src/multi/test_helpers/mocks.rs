use cosmwasm_std::{Empty, GovMsg, IbcMsg, IbcQuery};

use crate::{
    BankKeeper, DistributionKeeper, FailingModule, Router, StakingKeeper, StargateFailing,
    WasmKeeper,
};

pub type BasicRouter<ExecC = Empty, QueryC = Empty> = Router<
    BankKeeper,
    FailingModule<ExecC, QueryC, Empty>,
    WasmKeeper<ExecC, QueryC>,
    StakingKeeper,
    DistributionKeeper,
    FailingModule<IbcMsg, IbcQuery, Empty>,
    FailingModule<GovMsg, Empty, Empty>,
    StargateFailing,
>;

pub fn mock_router() -> BasicRouter {
    Router {
        wasm: WasmKeeper::new(),
        bank: BankKeeper::new(),
        custom: FailingModule::new(),
        staking: StakingKeeper::new(),
        distribution: DistributionKeeper::new(),
        ibc: FailingModule::new(),
        gov: FailingModule::new(),
        stargate: StargateFailing,
    }
}
