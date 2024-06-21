use tar::Archive;
use std::io::Read;
// use std::fs::File;
use std::io::Cursor;
// use std::ffi::OsStr;
// use std::path::PathBuf;
// use thiserror::Error as ThisError;
// use std::io::BufWriter;
use serde::{Deserialize, Serialize};
// use crate::{ExecutionTarget, GraphEncoding};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelMetadata {
   /// Model name (optional)
   #[serde(default)]
   pub model_name: Option<String>,

   // /// graph encoding
   // #[serde(default)]
   // pub graph_encoding: GraphEncoding,

   // /// execution target
   // #[serde(default)]
   // pub execution_target: ExecutionTarget,

   /// tensor type
   #[serde(default)]
   pub tensor_type: String,

   /// tensor dimensions in (optional)
   #[serde(default)]
   pub tensor_dimensions_in: Option<Vec<u32>>,

   /// tensor dimensions out (optional)
   #[serde(default)]
   pub tensor_dimensions_out: Option<Vec<u32>>,
}

impl ModelMetadata {
   /// load metadata from json
   pub fn from_json(data: &[u8]) -> Result<Self, ModelLoaderError> {
      serde_json::from_slice(data)
         .map_err(|e| ModelLoaderError::ModelLoaderJsonError(format!("invalid json: {}", e)))
   }
}
pub struct ModelData {
   pub metadata: Vec<u8>,
   pub model: Vec<u8>,
}

pub struct ModelLoader;

impl ModelLoader {
   pub async fn deserialize_metadata(data: &[u8]) -> ModelLoaderResult<ModelMetadata> {
      let metadata: ModelMetadata =
            ModelMetadata::from_json(&data).map_err(|error| {
               //  log::error!("BindleParsingMetadataError: '{}'", error);
                ModelLoaderError::ModelLoaderJsonError(format!("{}", error))
            })?;
      
      Ok(metadata)
   }

   /// get model and metadata
   pub async fn get_model_and_metadata(
      data: Vec<u8>
   ) -> ModelLoaderResult<ModelData> {
      let mut tar_archive = Archive::new(Cursor::new(data));
  
      let mut tar_entries = tar_archive.entries()
      .map_err(|error| {
         // log::error!("The tar archive does not contain any entries!");
         ModelLoaderError::ModelLoaderTarError(format!("{}", error))
      })?;

      let mut config_file = tar_entries
      .find(|entry| 
         entry
         .as_ref()
         .is_ok_and(|e| e
            .path()
            .is_ok_and(|path| path
               .extension()
               .is_some_and(|ext| ext.to_str().is_some_and(|e| e == "json"))
            )
         )
      )
      .ok_or_else(|| format!("No JSON file found in the tar archive"))
      .map_err(|e| ModelLoaderError::ModelLoaderTarError(format!("{}", e)))?
      .map_err(|e| ModelLoaderError::ModelLoaderTarError(format!("{}", e)))?;

      // let mut content = config_file.map_err(|e| ModelLoaderError::ModelLoaderTarError(format!("{}", e)))?;
      let mut metadata:Vec<u8> = Vec::new();
      config_file.read_to_end(&mut metadata).map_err(|e| ModelLoaderError::ModelLoaderTarError(format!("{}", e)))?;

      let mut model_file = tar_entries
      .find(|entry| 
         entry
         .as_ref()
         .is_ok_and(|e| e
            .path()
            .is_ok_and(|path| path
               .extension()
               .is_some_and(|ext| ext.to_str().is_some_and(|e| e != "json"))
            )
         )
      )
      .ok_or_else(|| format!("No JSON file found in the tar archive"))
      .map_err(|e| ModelLoaderError::ModelLoaderTarError(format!("{}", e)))?
      .map_err(|e| ModelLoaderError::ModelLoaderTarError(format!("{}", e)))?;

      // let mut content = config_file.map_err(|e| ModelLoaderError::ModelLoaderTarError(format!("{}", e)))?;
      let mut model:Vec<u8> = Vec::new();
      model_file.read_to_end(&mut model).map_err(|e| ModelLoaderError::ModelLoaderTarError(format!("{}", e)))?;


      // model = tar_archive
      // .entries()?
      // .find(|entry| {
      //     let path = entry.path()?;
      //     path.extension()
      //         .and_then(|ext| ext.to_str())
      //         .map(|ext_str| ext_str == "json")
      //         .unwrap_or(false)
      // })
      // .ok_or_else(|| "No .json file found in the tar archive".to_string())?;

      // for entry in tar_entries 
      // {
      //    let entry = entry
      //     .map_err(|error| {
      //       // log::error!("The tar entry is invalid!");
      //       ModelLoaderError::ModelLoaderTarError(format!("{}", error))
      //    })?;
          
      //    let entry_path = entry.path()
      //       .map_err(|error| {
      //       // log::error!("The tar entry does not have any path!?");
      //       ModelLoaderError::ModelLoaderTarError(format!("{}", error))
      //    })?
      //    .to_path_buf();

      //    if entry_path.extension().and_then(|ext| ext.to_str()) == Some("json") {
      //       metadata = Some(entry_path);
      //    } else {
      //       model = Some(entry_path)
      //    }
      // };

      // let metadata = metadata.ok_or("No .json file found in the archive")
      // .map_err(|error| {
      //    // log::error!("The tar entry does not have any path!?");
      //    ModelLoaderError::ModelLoaderTarError(format!("{}", error))
      // })?;
      
      // let model = model.ok_or("No model file found in the archive")
      // .map_err(|error| {
      //    // log::error!("The tar entry does not have any path!?");
      //    ModelLoaderError::ModelLoaderTarError(format!("{}", error))
      // })?;
  
      Ok(ModelData { metadata, model })
   }
}

/// BindleResult
pub type ModelLoaderResult<T> = Result<T, ModelLoaderError>;

#[derive(Debug, thiserror::Error)]
pub enum ModelLoaderError {
   #[error("invalid tar archive {0}")]
   ModelLoaderTarError(String),

   #[error("invalid json {0}")]
   ModelLoaderJsonError(String),

   #[error("Error parsing metadata {0}")]
   ModelLoaderMetadataError(String),
}