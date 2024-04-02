// Currently need to import `self` because `contracttype` expects it in the namespace
use loam_sdk::soroban_sdk::{env, Address, Env, Symbol, TryFromVal, Val, Vec};

pub struct ContractCall {
    contract_address: Address,
    func: Symbol,
    args: Vec<Val>,
}

impl From<(Option<Address>, Symbol, Vec<Val>)> for ContractCall {
    fn from((contract_address, func, args): (Option<Address>, Symbol, Vec<Val>)) -> Self {
        let contract_address = contract_address.unwrap_or_else(|| env().current_contract_address());
        Self {
            contract_address,
            func,
            args,
        }
    }
}

impl ContractCall {
    pub fn try_invoke<T: TryFromVal<Env, Val>>(self) -> Result<T, crate::Error> {
        let e = env();
        let Self {
            contract_address,
            func,
            args,
        } = self;

        match e.try_invoke_contract::<T, crate::Error>(&contract_address, &func, args) {
            Ok(Ok(res)) => Ok(res),
            Ok(_) | Err(_) => Err(crate::Error::InvokeFailed),
        }
    }
}
