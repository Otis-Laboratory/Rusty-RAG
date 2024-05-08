mod utils {
    pub mod embed_text;
}

use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsBuilder;
use utils::embed_text::embed_text;

fn main() {
    let embedding_model = SentenceEmbeddingsBuilder::local("models/bge-large-en-v1.5")
        .with_device(tch::Device::cuda_if_available())
        .create_model();

    match embedding_model {
        Ok(model) => {
            let query: &str = "Sally sold sea shells at the sea shore.";
            let embedding = embed_text(query, &model);
            // println!("{:?}", embedding.unwrap());
        }
        Err(err) => {
            eprintln!("Error creating embedding model: {}", err);
        }
    }
}
