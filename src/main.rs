use model_loader::ModelLoader;
use tar::Archive;
// use std::fs::File;
// use std::fs::read;
use std::io::Cursor;
use std::io::prelude::*;
use flate2::read::GzDecoder;
// use std::io::{BufWriter};
use oci_distribution::{
    client::ClientConfig, secrets::RegistryAuth, Client, Reference
};

mod model_loader;

pub const IMAGE_REFERENCE: &str = "localhost:5000/wamli-mobilenet:latest";

// This is not set explicitly in the build script.
// It seems to be derviced by the server.
pub const MEDIA_TYPE: &str = "application/vnd.oci.image.layer.v1.tar+gzip";

pub async fn uncompress_layer(data: Vec<u8>) -> Vec<u8> {
    let mut decompressed_data = Vec::new();
    let mut gz_decoder = GzDecoder::new(&data[..]);
    gz_decoder.read_to_end(&mut decompressed_data).unwrap();
    return decompressed_data;
}

pub async fn untar_archive_and_extract(data: Vec<u8>, file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
// pub async fn untar_archive_and_extract(data: Vec<u8>, file_path: &str) {
    let mut archive = Archive::new(Cursor::new(data));
    
    // for file in archive.entries().unwrap() {
    //     // Make sure there wasn't an I/O error
    //     let mut file = file.unwrap();

    //     // Inspect metadata about the file
    //     println!("Found file {:?} of size {}", file.header().path().unwrap(), file.header().size().unwrap());

    //     // files implement the Read trait
    //     let mut s = String::new();
    //     file.read_to_string(&mut s).unwrap();
    //     println!("{}", s);
    // }


    // let entries = archive.entries().unwrap(); 
    // for entry in entries {
    //     let mut file = entry.unwrap(); 
    //     let mut contents = String::new();
    //     file.read_to_string(&mut contents).unwrap();
    //     println!("My file content: {}", contents);
    //     break;
    // }

    let file_entry = archive
        .entries()?
        .find(|entry|  entry.as_ref().unwrap().path().unwrap().to_str() == Some(file_path))
        .ok_or_else(|| format!("File {} not found in the tar archive", file_path))?;

    let mut entry = file_entry?;
    let mut content = Vec::new();
    entry.read_to_end(&mut content)?;
    
    Ok(content)
}

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

    println!("This image has {} layer(s)", &image_data.layers.len());

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

    let uncompressed_layer = uncompress_layer(first_layer).await;

    println!("Uncompressed layer size: {} [bytes]\n", uncompressed_layer.len());

    let model_data = ModelLoader::get_model_and_metadata(uncompressed_layer).await.expect("SOMETHING WRONG - REPLACE ME!");

    let model_configuration = ModelLoader::deserialize_metadata(&model_data.metadata).await.expect("SOMETHING WRONG - REPLACE ME!");

    println!("model configuration: {:?}", model_configuration);

    // let maybe_file1 = untar_archive_and_extract(uncompressed.clone(), "mobilenetv2-7.json").await;
    // let file2 = untar_archive_and_extract(uncompressed, "mobilenetv2-7.onnx").await;

    // let file1 = maybe_file1.expect("file2 could NOT be extracted");
    // let text = String::from_utf8_lossy(&file1);

    // println!("Content of metadata:\n{}", text);
    // println!("Size of Metadata: {:?}", file1.len());

    // // println!("Content of metadata:\n{:?}", file2);
    // println!("\nSize of AI model: {:?} Byte - the content is a binary and not shown here for clarity", file2.expect("file2 could NOT be extracted").len());
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