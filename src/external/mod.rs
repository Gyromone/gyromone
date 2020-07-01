pub mod line;

lazy_static! {
    pub static ref HTTP_CLIENT: hyper::client::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>, hyper::Body> =
        HTTPClient::new(8);
}

pub struct HTTPClient {}

impl HTTPClient {
    pub fn new(
        thread_size: u8,
    ) -> hyper::client::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>, hyper::Body>
    {
        let thread = usize::from(thread_size);
        let https = hyper_tls::HttpsConnector::new(thread).unwrap();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);
        client
    }
}
