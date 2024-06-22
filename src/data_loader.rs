use std::io::Read;
use log::{debug, info, warn, error};

pub mod model_loader;
pub mod oci_image_loader;

/// BindleResult
pub type DataLoaderResult<T> = Result<T, DataLoaderError>;

#[derive(Debug, thiserror::Error)]
pub enum DataLoaderError {
   #[error("invalid tar archive {0}")]
   ModelLoaderTarError(String),

   #[error("invalid json {0}")]
   ModelLoaderJsonError(String),

//    #[error("Error parsing metadata {0}")]
//    ModelLoaderMetadataError(String),

    #[error("Unable to pull image: {0}")]
    OciImageLoadError(String),

    #[error("Unable to pull image: {0}")]
    OciUncompressError(String),

    #[error("Unable to load image's layer!")]
    OciLayerLoadError,

}