use krunker_rs::Client;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

pub struct Response {
    status: &'static str,
    body: String,
    headers: Vec<(&'static str, String)>,
    delay: Duration,
    extra_length: usize,
}

impl Response {
    pub fn new(status: &'static str, body: impl Into<String>) -> Self {
        Self {
            status,
            body: body.into(),
            headers: Vec::new(),
            delay: Duration::ZERO,
            extra_length: 0,
        }
    }

    pub fn ok(body: impl Into<String>) -> Self {
        Self::new("200 OK", body)
    }

    pub fn header(mut self, name: &'static str, value: impl Into<String>) -> Self {
        self.headers.push((name, value.into()));
        self
    }

    pub fn delayed(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    pub fn truncated(mut self) -> Self {
        self.extra_length = 64;
        self
    }
}

pub struct Server {
    pub base_url: String,
    requests: Arc<Mutex<Vec<String>>>,
    task: JoinHandle<()>,
}

impl Server {
    pub async fn start(responses: Vec<Response>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}/api", listener.local_addr().unwrap());
        let requests = Arc::new(Mutex::new(Vec::new()));
        let captured = requests.clone();
        let task = tokio::spawn(async move {
            for response in responses {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut request = Vec::new();
                let mut buffer = [0u8; 4096];
                while !request.windows(4).any(|part| part == b"\r\n\r\n") {
                    let length = stream.read(&mut buffer).await.unwrap();
                    assert_ne!(length, 0, "Client disconnected before sending headers");
                    request.extend_from_slice(&buffer[..length]);
                    assert!(request.len() < 65_536, "Unexpectedly large request headers");
                }
                captured
                    .lock()
                    .unwrap()
                    .push(String::from_utf8(request).unwrap());
                tokio::time::sleep(response.delay).await;
                let mut headers = format!(
                    "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: application/json\r\nConnection: close\r\n",
                    response.status,
                    response.body.len() + response.extra_length,
                );
                for (name, value) in response.headers {
                    headers.push_str(&format!("{name}: {value}\r\n"));
                }
                headers.push_str("\r\n");
                // A timeout test may intentionally disconnect before this write.
                if stream.write_all(headers.as_bytes()).await.is_ok() {
                    let _ = stream.write_all(response.body.as_bytes()).await;
                    let _ = stream.shutdown().await;
                }
            }
        });
        Self {
            base_url,
            requests,
            task,
        }
    }

    pub fn client(&self) -> Client {
        Client::builder("test-key")
            .base_url(&self.base_url)
            .http_client(reqwest::Client::builder().no_proxy().build().unwrap())
            .build()
            .unwrap()
    }

    pub fn targets(&self) -> Vec<String> {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .map(|request| request.split_whitespace().nth(1).unwrap().to_owned())
            .collect()
    }

    pub fn request(&self, index: usize) -> String {
        self.requests.lock().unwrap()[index].clone()
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.task.abort();
    }
}
