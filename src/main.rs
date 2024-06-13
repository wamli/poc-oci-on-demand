use std::fs::read;
use oci_distribution::{
    client::ClientConfig, secrets::RegistryAuth, Client, Reference
};

pub const IMAGE_REFERENCE: &str = "localhost:5000/wamli-ml-01:latest";

// This is not set explicitly in the build script.
// It seems to be derviced by the server.
pub const MEDIA_TYPE: &str = "application/vnd.oci.image.layer.v1.tar+gzip";

pub async fn read_first_image_layer(image_ref: &str, content_type: &str) -> Vec<u8> {
    // Experimenting with an ordinary local docker registry,
    // the protocol is `Http`
    let config = ClientConfig {
        protocol: oci_distribution::client::ClientProtocol::Http,
        ..Default::default()
    };

    let client = Client::new(config);

    let reference: Reference = image_ref.parse().unwrap();

    let image_data = client.pull(
        &reference, 
        &RegistryAuth::Anonymous, 
        vec![content_type])
        .await
        .unwrap();

    // The example image is supposed to have one layer.
    // The first, or only, layer is supposed to contain
    // a text file, e.g. content.txt
    let first_layer = &image_data.layers[0].data;

    return first_layer.to_vec();
}

/// Make use of [oci-distribution](https://crates.io/crates/oci-distribution)
/// in order to pull an image and read out its content.
#[tokio::main]
async fn main() {
    println!("Ramping up ..");

    let first_layer = read_first_image_layer(&IMAGE_REFERENCE, &MEDIA_TYPE).await;

    // Text in text files is supposed to be UTF-8 compliant
    let text_representation = String::from_utf8_lossy(&first_layer);

    println!("Bytes of first layer's file:\n{first_layer:?}\n");
    println!("Text  of first layer's file:\n{text_representation:?}\n");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[tokio::test]
    async fn test_content_equality() {
        let file_bytes = read("./content.txt").unwrap();
        let file_text = String::from_utf8_lossy(&file_bytes);
        
        println!("\nfile: {:?}", &file_bytes);
        println!("file: {:?}\n", &file_text);
        
        let first_image_layer = read_first_image_layer(&IMAGE_REFERENCE, &MEDIA_TYPE).await;

        assert_eq!(file_bytes, first_image_layer, "we are testing equality between the text file's bytes and the layer's bytes!?");
    }
}