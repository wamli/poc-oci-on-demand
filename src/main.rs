mod data_loader;

use data_loader as dl;
use dl::DataLoaderResult;

pub const IMAGE_REFERENCE: &str = "localhost:5000/wamli-mobilenet:latest";

// This is not set explicitly in the build script.
// It seems to be derviced by the server.
pub const MEDIA_TYPE: &str = "application/vnd.oci.image.layer.v1.tar+gzip";

// pub async fn untar_archive_and_extract(data: Vec<u8>, file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
// // pub async fn untar_archive_and_extract(data: Vec<u8>, file_path: &str) {
//     let mut archive = Archive::new(Cursor::new(data));
    
//     // for file in archive.entries().unwrap() {
//     //     // Make sure there wasn't an I/O error
//     //     let mut file = file.unwrap();

//     //     // Inspect metadata about the file
//     //     println!("Found file {:?} of size {}", file.header().path().unwrap(), file.header().size().unwrap());

//     //     // files implement the Read trait
//     //     let mut s = String::new();
//     //     file.read_to_string(&mut s).unwrap();
//     //     println!("{}", s);
//     // }


//     // let entries = archive.entries().unwrap(); 
//     // for entry in entries {
//     //     let mut file = entry.unwrap(); 
//     //     let mut contents = String::new();
//     //     file.read_to_string(&mut contents).unwrap();
//     //     println!("My file content: {}", contents);
//     //     break;
//     // }

//     let file_entry = archive
//         .entries()?
//         .find(|entry|  entry.as_ref().unwrap().path().unwrap().to_str() == Some(file_path))
//         .ok_or_else(|| format!("File {} not found in the tar archive", file_path))?;

//     let mut entry = file_entry?;
//     let mut content = Vec::new();
//     entry.read_to_end(&mut content)?;
    
//     Ok(content)
// }

/// Make use of [oci-distribution](https://crates.io/crates/oci-distribution)
/// in order to pull an image and read out its content.
#[tokio::main]
async fn main() -> DataLoaderResult<()> {
    println!("Ramping up ..");

    // let oci_image = dl::oci_image_loader::pull_image(&IMAGE_REFERENCE, &MEDIA_TYPE).await?;
    // let first_layer = dl::oci_image_loader::read_first_layer(oci_image).await?;
    // let uncompressed_layer = dl::oci_image_loader::uncompress_layer(first_layer).await?;
    // println!("Uncompressed layer size: {} [bytes]\n", uncompressed_layer.len());

    // // let mut tar_archive = Archive::new(Cursor::new(uncompressed_layer));
    // // let metadata = dl::model_loader::untar_metadata(&mut tar_archive).await?;
    // // let _model_data = dl::model_loader::untar_model(&mut tar_archive).await?;

    // let model_data = dl::model_loader::untar_model_and_metadata(uncompressed_layer).await.expect("SOMETHING WRONG - REPLACE ME!");
    
    let model_data = dl::pull_model_and_metadata(&IMAGE_REFERENCE, &MEDIA_TYPE).await?;
    
    println!("model configuration: {:?}", model_data.metadata);

    Ok(())
}





// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     #[tokio::test]
//     async fn test_content_equality() {
//         let file_bytes = read("./content.txt").unwrap();
//         let file_text = String::from_utf8_lossy(&file_bytes);
        
//         println!("\nfile: {:?}", &file_bytes);
//         println!("file: {:?}\n", &file_text);
        
//         let first_image_layer = read_first_image_layer(&IMAGE_REFERENCE, &MEDIA_TYPE).await;

//         assert_eq!(file_bytes, first_image_layer, "we are testing equality between the text file's bytes and the layer's bytes!?");
//     }
// }