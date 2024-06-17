use zephyr_sdk::{prelude::*, soroban_sdk::{contracttype,  xdr::{ScVal, LedgerEntryData }},  EnvClient, DatabaseDerive};

#[derive(DatabaseDerive, Clone)]
#[with_name("pairs")]
struct PairsTable {
    address: ScVal,
    token_a: ScVal,
    token_b: ScVal,
    reserve_a: ScVal,
    reserve_b: ScVal,
}

pub(crate) const FACTORY_CONTRACT_ADDRESS: [u8; 32] = [218, 84, 31, 49, 226, 201, 127, 151, 106, 85, 127, 229, 77, 127, 214, 20, 83, 199, 183, 54, 49, 135, 253, 221, 62, 169, 188, 21, 187, 147, 233, 135];

//Phoenix Factory
#[derive(Clone, Copy)]
#[repr(u32)]
#[contracttype]
pub enum FactoryDataKey {
    Config = 1,
    LpVec = 2,
    Initialized = 3,
}

#[no_mangle]
pub extern "C" fn on_close() {
    let env = EnvClient::new();

    let entries = env.read_contract_entries(FACTORY_CONTRACT_ADDRESS).unwrap();

    let rows = env.read::<PairsTable>();

    for entry in entries {
        if entry.key == env.to_scval(FactoryDataKey::LpVec) {
            let LedgerEntryData::ContractData(data) = entry.entry.data else {panic!()};

            let ScVal::Vec(Some(data)) = data.val else {panic!()};

            data.iter().for_each(|x| {
                let table = PairsTable {
                    address: x.clone(),
                    reserve_a: env.to_scval(0),
                    reserve_b: env.to_scval(0),
                    token_a: env.to_scval("0"),
                    token_b: env.to_scval("0"),
                };

                if rows.iter().any(|row| row.address == table.address) {
                    env.update().column_equal_to_xdr("address", &table.address).execute(&table).ok();
                }else{
                    table.put(&env);
                }

            });
        }
        
    }

   
      
}            


