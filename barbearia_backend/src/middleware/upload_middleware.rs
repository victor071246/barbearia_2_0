use axum::extract::multipart::Field;
use tokio::fs;
use uuid::Uuid;

// Recebe um campo (field) do multipart e tenta salvar como imagem
// Retorna Some(url) se conseguir, None se falhar ou não tier imagem
pub async fn extract_image(field: Field<'_>) -> Option<String> {

    // Pega o tipo do conteúdo (content_type) antes de consumir o campo
    let content_type = field.content_type().unwrap_or("").to_string();

    // Define a extensão, é JPG se não reconhecer
    let ext = match content_type.as_str() {
        "image/png" => "png",
        "image/webp" => "webp",
        "image/jpeg" => "jpg",
        _ => "jpg",
    };

    // Lê os bytes do arquivo, se estiver vazio, não tem imagem, então retorna None
    let bytes = field.bytes().await.ok()?;
    if bytes.is_empty() {
        return None;
    }

    // Cria a pasta se não existir
    fs::create_dir_all("uploads/items").await.ok()?;

    // Nome único para evitar colisão de arquivos
    let file_name = format!("{}.{}", Uuid::new_v4(), ext);
    let path = format!("uploads/items/{}", file_name);

    // Salva o arquivo, se falhar, retorna None
    fs::write(&path, &bytes).await.ok()?;

    // Retorna a URL pública do arquivo
    Some(format! ("/uploads/items/{}", file_name))
}