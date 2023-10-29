use async_std::task::block_on;
use serde_json::json;
use zen_engine::DecisionEngine;
use zen_engine::model::DecisionContent;

async fn evaluate() {
    let decision_content: DecisionContent = serde_json::from_str(include_str!("THE_COOLEST_KB.json")).unwrap();
    let engine = DecisionEngine::default();
    let decision = engine.create_decision(decision_content.into());

    let request = json!({
        "company_type": "LTD",
        "country": "UA",

        "turnover": 4000000,
        "debt": 20000,

        "credit": 1000000,
        "percent": 3.4,
        "bilresp": 12
    });

    let response = decision.evaluate(&request).await.unwrap();
    println!("request: {}", request);
    println!("response: {}", response.result);
}

fn main() {
    block_on(evaluate());
}
