#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetFile {
    pub file_name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UploadFile {
    pub file_name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RemoveFile {
    pub file_name: String,
}