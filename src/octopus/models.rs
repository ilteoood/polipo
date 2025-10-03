use serde::Deserialize;

use crate::cache::UtilityType;

/// Parameters for product pricing
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductParams {
    pub consumption_charge: String,
    pub annual_standing_charge: String,
    pub product_type: String,
}

impl ProductParams {
    pub fn parse_charges(&self) -> Result<(f64, f64), std::num::ParseFloatError> {
        let consumption = self.consumption_charge.replace(',', ".").parse::<f64>()?;
        let standing = self
            .annual_standing_charge
            .replace(',', ".")
            .parse::<f64>()?;
        Ok((consumption, standing))
    }
}

/// Product from Octopus Energy API (electricity or gas)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub code: String,
    pub display_name: String,
    pub full_name: String,
    pub description: String,
    pub params: ProductParams,
}

impl Product {
    pub fn is_same_type(&self, utility_type: UtilityType) -> bool {
        matches!(
            (self.typename.as_str(), utility_type),
            ("ElectricityProductType", UtilityType::Luce) | ("GasProductType", UtilityType::Gas)
        )
    }
}

/// Page props containing products from Next.js data
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageProps {
    pub products: Vec<Product>,
}

/// Props wrapper for Next.js data
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Props {
    pub page_props: PageProps,
}

/// Next.js data structure from __NEXT_DATA__ script
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextData {
    pub props: Props,
}

/// Product information for supply points
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplyPointProduct {
    pub display_name: String,
    pub params: ProductParams,
}

/// Supply point from GraphQL API (electricity or gas)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplyPoint {
    pub status: String,
    pub product: SupplyPointProduct,
}

/// Property containing supply points
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub electricity_supply_points: Vec<SupplyPoint>,
    pub gas_supply_points: Vec<SupplyPoint>,
}

/// Account information from GraphQL API
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub number: String,
    pub properties: Vec<Property>,
}

/// User viewer data from GraphQL API
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewer {
    pub email: String,
    pub full_name: String,
    pub accounts: Vec<Account>,
}

/// Wrapper for viewer data
#[derive(Debug, Deserialize)]
pub struct ViewerData {
    pub viewer: Viewer,
}

/// GraphQL response structure
#[derive(Debug, Deserialize)]
pub struct GraphQLResponse {
    pub data: ViewerData,
}
