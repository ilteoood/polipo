use anyhow::{Context, Result};
use log::info;
use regex::Regex;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue, SET_COOKIE};
use scraper::{Html, Selector};

use crate::config::Config;
use crate::octopus::models::{GraphQLResponse, NextData, Product, Viewer};

/// Octopus Energy API client
pub struct OctopusClient {
    client: reqwest::Client,
    config: Config,
}

impl OctopusClient {
    /// Create a new Octopus client with cookie store enabled
    pub fn new(config: Config) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("Polipo")
            .cookie_store(false)
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    /// Fetch tariffs from Octopus website and filter for FIXED_SINGLE_RATE products
    pub async fn fetch_tariffs(&self) -> Result<Vec<Product>> {
        info!("Fetching tariffs from Octopus website");

        let response = self
            .client
            .get("https://octopusenergy.it/le-nostre-tariffe")
            .send()
            .await
            .context("Failed to fetch tariffs page")?;

        let html = response
            .text()
            .await
            .context("Failed to read response body")?;

        // Parse HTML and find the __NEXT_DATA__ script
        let document = Html::parse_document(&html);
        let selector = Selector::parse("script#__NEXT_DATA__").unwrap();

        let script_element = document
            .select(&selector)
            .next()
            .context("Could not find __NEXT_DATA__ script")?;

        let json_text = script_element.inner_html();

        let next_data: NextData =
            serde_json::from_str(&json_text).context("Failed to parse __NEXT_DATA__ JSON")?;

        // Filter for FIXED_SINGLE_RATE products
        let filtered_products: Vec<Product> = next_data
            .props
            .page_props
            .products
            .into_iter()
            .filter(|product| product.params.product_type == "FIXED_SINGLE_RATE")
            .collect();

        info!(
            "Found {} FIXED_SINGLE_RATE products",
            filtered_products.len()
        );
        Ok(filtered_products)
    }

    /// Login to Octopus account and extract access token
    pub async fn login(&self) -> Result<String> {
        info!("Logging into Octopus account");

        let login_data = serde_json::json!({
            "email": self.config.email,
            "password": self.config.password
        });

        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("text/plain;charset=UTF-8"),
        );

        let response = self
            .client
            .post("https://octopusenergy.it/api/auth/login")
            .headers(headers)
            .json(&login_data)
            .send()
            .await
            .context("Failed to login")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Login failed with status: {}",
                response.status()
            ));
        }

        // Extract access token from cookies
        let cookies = response.headers().get_all(SET_COOKIE);
        let access_token_regex = Regex::new(r"accessToken=([^;]+)").unwrap();

        for cookie in cookies {
            let cookie_str = cookie.to_str().unwrap_or("");
            if let Some(captures) = access_token_regex.captures(cookie_str)
                && let Some(token) = captures.get(1)
            {
                let token_value = token.as_str();
                if !token_value.is_empty() {
                    info!("Successfully logged in and extracted access token");
                    return Ok(token_value.to_string());
                }
            }
        }

        Err(anyhow::anyhow!(
            "Could not extract access token from login response"
        ))
    }

    /// Fetch user data using GraphQL
    pub async fn fetch_user_data(&self, access_token: &str) -> Result<Viewer> {
        info!("Fetching user data");

        let query = r#"
        query Viewer {
          viewer {
            email
            fullName
            accounts {
              ... on AccountType {
                number
                properties {
                  electricitySupplyPoints {
                    status
                    product {
                      displayName
                      params {
                        consumptionCharge
                        annualStandingCharge
                        productType
                      }
                    }
                  }
                  gasSupplyPoints {
                    status
                    product {
                      params {
                        consumptionCharge
                        annualStandingCharge
                        productType
                      }
                    }
                  }
                }
              }
            }
          }
        }
        "#;

        let graphql_request = serde_json::json!({
            "query": query
        });

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(access_token).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let response = self
            .client
            .post("https://api.oeit-kraken.energy/v1/graphql/")
            .headers(headers)
            .json(&graphql_request)
            .send()
            .await
            .context("Failed to fetch user data")?;

        let graphql_response: GraphQLResponse = response
            .json()
            .await
            .context("Failed to parse GraphQL response")?;

        Ok(graphql_response.data.viewer)
    }
}
