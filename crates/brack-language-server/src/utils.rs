use anyhow::Result;
use lsp_types::Uri;
use url::Url;
use urlencoding::decode;

pub fn to_url(uri: Uri) -> Result<Url> {
    let uri = decode(uri.as_str())?.into_owned();
    Url::parse(uri.as_str()).map_err(Into::into)
}