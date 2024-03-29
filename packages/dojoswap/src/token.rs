use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{StdError, StdResult, Uint128};
use cw20::{Cw20Coin, MinterResponse, Logo};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMarketingInfo {
    pub project: Option<String>,
    pub description: Option<String>,
    pub marketing: Option<String>,
    pub logo: Option<Logo>,
}

/// TokenContract InstantiateMsg
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Vec<Cw20Coin>,
    pub mint: Option<MinterResponse>,
    pub marketing: Option<InstantiateMarketingInfo>,
}

impl InstantiateMsg {
    pub fn get_cap(&self) -> Option<Uint128> {
        self.mint.as_ref().and_then(|v| v.cap)
    }

    pub fn validate(&self) -> StdResult<()> {
        // Check name, symbol, decimals
        if !is_valid_name(&self.name) {
            return Err(StdError::generic_err(
                "Name is not in the expected format (3-50 UTF-8 bytes)",
            ));
        }
        if !is_valid_symbol(&self.symbol) {
            return Err(StdError::generic_err(
                "Ticker symbol is not in expected format [a-zA-Z\\-]{3,12}",
            ));
        }
        if self.decimals > 18 {
            return Err(StdError::generic_err("Decimals must not exceed 18"));
        }
        Ok(())
    }
}

fn is_valid_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.len() < 3 || bytes.len() > 50 {
        return false;
    }
    true
}

fn is_valid_symbol(symbol: &str) -> bool {
    let bytes = symbol.as_bytes();
    if bytes.len() < 3 || bytes.len() > 12 {
        return false;
    }
    for byte in bytes.iter() {
        if (*byte != 45) && (*byte < 65 || *byte > 90) && (*byte < 97 || *byte > 122) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_cap() {
        let msg = InstantiateMsg {
            decimals: 6u8,
            initial_balances: vec![],
            mint: Some(MinterResponse {
                cap: Some(Uint128::from(1u128)),
                minter: "minter0000".to_string(),
            }),
            name: "test_token".to_string(),
            symbol: "TNT".to_string(),
            marketing: None
        };

        assert_eq!(msg.get_cap(), Some(Uint128::from(1u128)))
    }

    #[test]
    fn validate() {
        let valid_msg = InstantiateMsg {
            decimals: 6u8,
            initial_balances: vec![],
            mint: Some(MinterResponse {
                cap: Some(Uint128::from(1u128)),
                minter: "minter0000".to_string(),
            }),
            name: "test_token".to_string(),
            symbol: "TNT".to_string(),
            marketing: None
        };

        assert_eq!(valid_msg.validate(), Ok(()));

        let name_invalid_msg = InstantiateMsg {
            decimals: 6u8,
            initial_balances: vec![],
            mint: Some(MinterResponse {
                cap: Some(Uint128::from(1u128)),
                minter: "minter0000".to_string(),
            }),
            name: "a".to_string(),
            symbol: "TNT".to_string(),
            marketing: None
        };

        assert_eq!(
            name_invalid_msg.validate(),
            Err(StdError::generic_err(
                "Name is not in the expected format (3-50 UTF-8 bytes)",
            ))
        );

        let symbol_invalid_msg = InstantiateMsg {
            decimals: 6u8,
            initial_balances: vec![],
            mint: Some(MinterResponse {
                cap: Some(Uint128::from(1u128)),
                minter: "minter0000".to_string(),
            }),
            name: "test_token".to_string(),
            symbol: "TN".to_string(),
            marketing: None
        };

        assert_eq!(
            symbol_invalid_msg.validate(),
            Err(StdError::generic_err(
                "Ticker symbol is not in expected format [a-zA-Z\\-]{3,12}",
            ))
        );

        let decimal_invalid_msg = InstantiateMsg {
            decimals: 20u8,
            initial_balances: vec![],
            mint: Some(MinterResponse {
                cap: Some(Uint128::from(1u128)),
                minter: "minter0000".to_string(),
            }),
            name: "test_token".to_string(),
            symbol: "TNT".to_string(),
            marketing: None
        };

        assert_eq!(
            decimal_invalid_msg.validate(),
            Err(StdError::generic_err("Decimals must not exceed 18"))
        );
    }
}
