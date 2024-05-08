// Embed a query
// Take in text -> embed into x dimensions, return the array of vectors.

use std::time::{Duration, Instant};
use anyhow;
use rust_bert::{RustBertError, pipelines::sentence_embeddings::SentenceEmbeddingsModel};

pub fn embed_text(text: &str, model: &SentenceEmbeddingsModel) -> Result<Vec<Vec<f32>>, RustBertError> {
    // Embed the query into an array of vectors.
    let embedding = model.encode(&[text]);

    // Return embedding
    return embedding;
}