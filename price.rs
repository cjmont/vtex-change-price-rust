use reqwest::{Client, Error, Response, header::HeaderValue};
use chrono::{Utc, TimeZone, Datelike, Timelike};
use serde_json::Value;

// Definición de las estructuras

struct Price {
    sku: Sku,
    type_: String,
    store: Option<Store>,
    prices: Vec<PriceDetail>,
}

struct PriceDetail {
    value: f64,
}

struct Sku {
    identity: Vec<Identity>,
    product: Product,
}

struct Product {
    merchant: Merchant,
}

struct Merchant {
    account_set: Vec<Account>,
    identity: Vec<Identity>,
}

struct Account {
    app: String,
    key: String,
    token: String,
}

struct Store {
    identity: Vec<Identity>,
}

struct Identity {
    app: String,
    external_id: String,
}

struct Log {}

impl Log {
    fn create(instance: &Price, app: String, method: String, url: String, body: String, code: u16) {
        println!("Log created: {}, {}, {}, {}, {}, {}", app, method, url, body, code, instance.type_);
    }
}

fn on_change_price(sender: &str, instance: &Price, created: bool, update_fields: Option<Vec<&str>>) -> Result<(), Error> {
    if created || update_fields.map_or(false, |fields| {
        fields.iter().any(|&key| ["minimum", "value", "start", "end", "status"].contains(key))
    }) {
        if instance.sku.identity.iter().any(|id| id.app == module_path!().to_lowercase()) {
            let merchant = &instance.sku.product.merchant;

            if merchant.account_set.iter().any(|acc| acc.app == module_path!().to_lowercase()) &&
                merchant.identity.iter().any(|id| id.app == module_path!().to_lowercase()) {
                let account = merchant.account_set.iter().find(|&acc| acc.app == module_path!().to_lowercase()).unwrap();

                let client = Client::new();
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert("X-VTEX-API-AppKey", HeaderValue::from_str(&account.key).unwrap());
                headers.insert("X-VTEX-API-AppToken", HeaderValue::from_str(&account.token).unwrap());

                let mut response = None;
                let mut json = Vec::new(); 

                if let Some(store) = &instance.store {
                    if store.identity.iter().any(|id| id.app == module_path!().to_lowercase()) {
                        let mut table = store.identity.iter().find(|&id| id.app == module_path!().to_lowercase()).unwrap().external_id.clone();

                        if instance.type_ == "discount" {
                            table.push_str("-");
                            table.push_str(&instance.type_);
                        }

                        for price in &instance.sku.prices {
                            if price.value > 1.0 {
                                let mut price_data = serde_json::Map::new();
                                price_data.insert("value".to_string(), Value::from(price.value));
                                json.push(Value::Object(price_data));
                            }
                        }

                        // Realizar la solicitud POST:
                        response = Some(client.post(&format!("https://api.vtex.com/{}/pricing/prices/{}/fixed/{}", account.merchant.identity.iter().find(|&id| id.app == module_path!().to_lowercase()).unwrap().external_id, instance.sku.identity.iter().find(|&id| id.app == module_path!().to_lowercase()).unwrap().external_id, table))
                            .headers(headers.clone())
                            .json(&json)
                            .send()?);
                    }
                } else {
                    response = Some(client.put(&format!("https://api.vtex.com/{}/pricing/prices/{}", merchant.identity.iter().find(|&id| id.app == module_path!().to_lowercase()).unwrap().external_id, instance.sku.identity.iter().find(|&id| id.app == module_path!().to_lowercase()).unwrap().external_id))
                        .headers(headers)
                        .json(&json)
                        .send()?);
                }

                if let Some(resp) = response {
                    Log::create(instance, module_path!().to_lowercase(), resp.method().as_str().to_lowercase(), resp.url().to_string(), resp.text()?, resp.status().as_u16());
                }
            }
        }
    }
    Ok(())
}
