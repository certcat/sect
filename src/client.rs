use thiserror::Error;

pub struct CT {
    base_url: reqwest::Url,
    client: reqwest::Client,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    HTTP(#[from] reqwest::Error),

    #[error("CT didn't return HTTP success: {0}")]
    HTTPStatus(reqwest::StatusCode),
}

/// parse_with_default_https parses a URL, including an https:// scheme unless
/// the URL is explicitly http://
fn parse_with_default_https(server: &str) -> Result<url::Url, url::ParseError> {
    if !server.starts_with("http://") {
        if !server.starts_with("https://") {
            let mut with_default_scheme = String::from("https://");
            with_default_scheme.push_str(server);
            return url::Url::parse(&with_default_scheme);
        }
    }
    return url::Url::parse(server);
}

impl CT {
    /// New CT client for the given CT server.
    /// Server should be an absolute URL.
    /// If it should start with http://, https://, that scheme will be used.
    /// Otherwise, it will assumed to be https://.
    pub fn new(server: &str) -> Result<CT, url::ParseError> {
        let base = parse_with_default_https(server)?;
        if base.cannot_be_a_base() {
            return Err(url::ParseError::RelativeUrlWithoutBase);
        }
        return Ok(CT {
            base_url: base,
            client: reqwest::Client::new(),
        });
    }

    /// ct_url returns a URL for a CT endpoint
    fn ct_url(&self, endpoint: &str) -> url::Url {
        let mut url = self.base_url.clone();
        url.path_segments_mut()
            .ok()
            .expect("cannot_be_a_base was checked in new()")
            .push("ct")
            .push("v1")
            .push(endpoint);
        return url;
    }

    fn get(&self, endpoint: &str) -> reqwest::RequestBuilder {
        return self.client.get(self.ct_url(endpoint));
    }

    pub async fn add_chain(&self, _chain: &[&[u8]]) -> Result<String, Error> {
        todo!()
        //let server = &self.server;
        //println!("POST https://{server}/ct/v1/add-chain")
    }

    pub async fn add_pre_chain(&self, _chain: &[&[u8]]) -> Result<String, Error> {
        todo!()
        //let server = &self.server;
        //println!("POST https://{server}/ct/v1/add-pre-chain")
    }

    pub async fn get_sth(&self) -> Result<String, Error> {
        let req = self.get_sth_request();
        let resp = self.client.execute(req).await?;
        if !resp.status().is_success() {
            return Err(Error::HTTPStatus(resp.status()));
        }
        Ok(resp.text().await?)
    }

    fn get_sth_request(&self) -> reqwest::Request {
        return self.get("get-sth").build().ok().unwrap();
    }

    pub async fn get_sth_consistency(&self, first: u64, second: u64) -> Result<String, Error> {
        let req = self.get_sth_consistency_request(first, second);
        let resp = self.client.execute(req).await?;
        if !resp.status().is_success() {
            return Err(Error::HTTPStatus(resp.status()));
        }
        Ok(resp.text().await?)
    }

    fn get_sth_consistency_request(&self, first: u64, second: u64) -> reqwest::Request {
        return self
            .get("get-sth-consistency")
            .query(&[("first", first), ("second", second)])
            .build()
            .ok()
            .unwrap();
    }

    pub async fn get_proof_by_hash(&self, hash: &str, tree_size: u64) -> Result<String, Error> {
        let req = self.get_proof_by_hash_request(hash, tree_size);
        let resp = self.client.execute(req).await?;
        if !resp.status().is_success() {
            return Err(Error::HTTPStatus(resp.status()));
        }
        Ok(resp.text().await?)
    }

    fn get_proof_by_hash_request(&self, hash: &str, tree_size: u64) -> reqwest::Request {
        return self
            .get("get-proof-by-hash")
            .query(&[("hash", hash)])
            .query(&[("tree_size", tree_size)])
            .build()
            .ok()
            .unwrap();
    }

    pub async fn get_entries(&self, start: u64, end: u64) -> Result<String, Error> {
        let req = self.get_entries_request(start, end);
        let resp = self.client.execute(req).await?;
        if !resp.status().is_success() {
            return Err(Error::HTTPStatus(resp.status()));
        }
        Ok(resp.text().await?)
    }

    fn get_entries_request(&self, start: u64, end: u64) -> reqwest::Request {
        return self
            .get("get-entries")
            .query(&[("start", start), ("end", end)])
            .build()
            .ok()
            .unwrap();
    }

    pub async fn get_roots(&self) -> Result<String, Error> {
        let req = self.get_roots_request();
        let resp = self.client.execute(req).await?;
        if !resp.status().is_success() {
            return Err(Error::HTTPStatus(resp.status()));
        }
        Ok(resp.text().await?)
    }

    fn get_roots_request(&self) -> reqwest::Request {
        return self.get("get-roots").build().ok().unwrap();
    }

    pub async fn get_entry_and_proof(
        &self,
        leaf_index: u64,
        tree_size: u64,
    ) -> Result<String, Error> {
        let req = self.get_entry_and_proof_request(leaf_index, tree_size);
        let resp = self.client.execute(req).await?;
        if !resp.status().is_success() {
            return Err(Error::HTTPStatus(resp.status()));
        }
        Ok(resp.text().await?)
    }

    fn get_entry_and_proof_request(&self, leaf_index: u64, tree_size: u64) -> reqwest::Request {
        return self
            .get("get-entry-and-proof")
            .query(&[("leaf_index", leaf_index), ("tree_size", tree_size)])
            .build()
            .ok()
            .unwrap();
    }
}

#[test]
fn test_request_urls() {
    let ct = CT::new("server/prefix").ok().unwrap();
    assert_eq!(
        ct.get_sth_request().url().as_str(),
        "https://server/prefix/ct/v1/get-sth"
    );
    assert_eq!(
        ct.get_sth_consistency_request(1234, 99999).url().as_str(),
        "https://server/prefix/ct/v1/get-sth-consistency?first=1234&second=99999"
    );
    assert_eq!(
        ct.get_proof_by_hash_request("some-hash", 1).url().as_str(),
        "https://server/prefix/ct/v1/get-proof-by-hash?hash=some-hash&tree_size=1"
    );
    assert_eq!(
        ct.get_entries_request(9000, 9255).url().as_str(),
        "https://server/prefix/ct/v1/get-entries?start=9000&end=9255"
    );
    assert_eq!(
        ct.get_roots_request().url().as_str(),
        "https://server/prefix/ct/v1/get-roots"
    );
    assert_eq!(
        ct.get_entry_and_proof_request(777, 7777).url().as_str(),
        "https://server/prefix/ct/v1/get-entry-and-proof?leaf_index=777&tree_size=7777"
    );
}

#[test]
fn test_server_args() {
    for (server, url) in [
        ("http://plaintext", "http://plaintext/ct/v1/get-sth"),
        ("http://plaintext/", "http://plaintext/ct/v1/get-sth"),
        ("http://plain/2025", "http://plain/2025/ct/v1/get-sth"),
        ("http://plain/2025/", "http://plain/2025//ct/v1/get-sth"),
        ("https://https", "https://https/ct/v1/get-sth"),
        ("https://https/", "https://https/ct/v1/get-sth"),
        ("https://https/2025", "https://https/2025/ct/v1/get-sth"),
        ("https://https/2025/", "https://https/2025//ct/v1/get-sth"),
        ("server", "https://server/ct/v1/get-sth"),
    ]
    .iter()
    {
        assert_eq!(
            CT::new(server)
                .expect("parses")
                .get_sth_request()
                .url()
                .as_str(),
            *url
        );
    }
}
