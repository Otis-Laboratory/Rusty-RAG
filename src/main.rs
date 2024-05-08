extern crate anyhow;
extern crate rust_bert;

use rust_bert::{pipelines::sentence_embeddings::SentenceEmbeddingsBuilder, RustBertError};

fn main() -> anyhow::Result<()> {
    let model =
        SentenceEmbeddingsBuilder::local("models/bge-large-en-v1.5")
            .with_device(tch::Device::cuda_if_available())
            .create_model()?;

    // Define input
    let sentences = ["this is an example sentence", "each sentence is converted"];

    // Generate Embeddings
    let embeddings = model.encode(&sentences)?;
    println!("{embeddings:?}");
    Ok(())
}
