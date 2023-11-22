use headers::HeaderMapExt;
use headers::{ContentLength, ContentType};
use std::{convert::Infallible, pin::Pin, task::Poll};
use tokio::io::{AsyncRead, ReadBuf};

use bytes::BytesMut;
use futures::Stream;
use warp::Filter;

struct TestStream {
    file: tokio::fs::File,
    total_file_bytes: u64,
    returned_bytes: usize,
    returned_error: bool,
}

impl TestStream {
    fn new() -> Self {
        let std_file = std::fs::File::open("./testfile.zip").unwrap();
        let total_file_bytes = std_file.metadata().unwrap().len();
        TestStream {
            file: std_file.into(),
            total_file_bytes,
            returned_bytes: 0,
            returned_error: false,
        }
    }

    fn state(&self) -> State {
        if (self.returned_bytes as u64) < (self.total_file_bytes / 5) {
            return State::ReadAndReturnBytes;
        }

        if self.returned_error {
            return State::Done;
        }

        State::ReturnError
    }
}

enum State {
    ReadAndReturnBytes,
    ReturnError,
    Done,
}

#[derive(Debug, thiserror::Error)]
enum DownloadError {
    #[error("Some error")]
    SomeError,
}

impl Stream for TestStream {
    type Item = Result<BytesMut, DownloadError>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let self_mut = self.get_mut();

        match self_mut.state() {
            State::ReadAndReturnBytes => {
                let mut buffer = vec![0u8; 1024];
                let mut read_buffer = ReadBuf::new(&mut buffer);
                let file = Pin::new(&mut self_mut.file);
                match file.poll_read(cx, &mut read_buffer) {
                    Poll::Ready(Ok(())) => {
                        let filled = read_buffer.filled();
                        self_mut.returned_bytes += filled.len();
                        let bytes = BytesMut::from(filled);
                        Poll::Ready(Some(Ok(bytes)))
                    }
                    Poll::Ready(e) => {
                        e.unwrap();
                        unreachable!();
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
            State::ReturnError => Poll::Ready(Some(Err(DownloadError::SomeError))),
            State::Done => Poll::Ready(None),
        }
    }
}

async fn handle_request() -> Result<warp::reply::Response, Infallible> {
    let mut response_builder = warp::http::Response::builder().status(warp::http::StatusCode::OK);

    let test_stream = TestStream::new();

    response_builder
        .headers_mut()
        .unwrap()
        .typed_insert(ContentLength(test_stream.total_file_bytes));
    response_builder
        .headers_mut()
        .unwrap()
        .typed_insert("application/zip".parse::<ContentType>().unwrap());

    let response = response_builder.body(warp::hyper::Body::wrap_stream(TestStream::new()));

    Ok(response.unwrap())
}

#[tokio::main]
async fn main() {
    let hello = warp::any().and_then(handle_request);

    warp::serve(hello)
        .tls()
        .key_path("MyKey.key")
        .cert_path("MyCertificate.crt")
        .run(([0, 0, 0, 0], 3030))
        .await;
}
