use anyhow::{Ok, Result};
use tiktoken_rs::get_bpe_from_model;

pub struct TokenStats {
    pub original_tokens: usize,
    pub formatted_tokens: usize,
    pub token_saved: usize,
    pub savings_percent: f64,
    // cost
    pub cost_original: f64,
    pub cost_formatted: f64,
    pub cost_saved: f64,
}

pub fn count_token(text: &str, model: &str) -> Result<usize> {
    let bpe = get_bpe_from_model(model)?;
    let tokens = bpe.encode_with_special_tokens(text);
    Ok(tokens.len())
}

pub fn calculate_cost(token: usize, price: f64) -> f64 {
    (token as f64 / 1_000_000.0) * price 
}

pub fn compare_tokens(original: &str, formatted: &str, model: &str,) -> Result<TokenStats> {
    let original_tokens = count_token(original, model)?;
    let formatted_tokens = count_token(formatted, model)?;
    let token_saved = original_tokens - formatted_tokens;
    let savings_percent = (token_saved as f64 / original_tokens as f64) * 100.0;

    let price = 30.0;
    let cost_original = calculate_cost(original_tokens, price);
    let cost_formatted = calculate_cost(formatted_tokens, price);
    let cost_saved = cost_original - cost_formatted;


    Ok(TokenStats { original_tokens, formatted_tokens, token_saved, savings_percent, cost_original, cost_formatted, cost_saved })
}