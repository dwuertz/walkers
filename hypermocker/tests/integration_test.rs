use hyper::body::Bytes;
use hypermocker::Server;
use std::time::Duration;

#[tokio::test]
async fn anticipate_then_request() {
    let _ = env_logger::try_init();

    let mock = Server::bind().await;
    let url = format!("http://localhost:{}/foo", mock.port());
    let request = mock.anticipate("/foo".to_string()).await;

    // Make sure that mock's internals kick in.
    tokio::time::sleep(Duration::from_secs(1)).await;

    futures::future::join(
        async {
            let response = reqwest::get(url).await.unwrap();
            let bytes = response.bytes().await.unwrap();
            assert_eq!(&bytes[..], b"hello");
        },
        async {
            request.respond(Bytes::from_static(b"hello")).await;
        },
    )
    .await;
}

#[tokio::test]
async fn anticipate_expect_then_request() {
    let _ = env_logger::try_init();

    let mock = Server::bind().await;
    let url = format!("http://localhost:{}/foo", mock.port());
    let mut request = mock.anticipate("/foo".to_string()).await;

    // Make sure that mock's internals kick in.
    tokio::time::sleep(Duration::from_secs(1)).await;

    futures::future::join(
        async {
            let response = reqwest::get(url).await.unwrap();
            let bytes = response.bytes().await.unwrap();
            assert_eq!(&bytes[..], b"hello");
        },
        async {
            request.expect().await;
            request.respond(Bytes::from_static(b"hello")).await;
        },
    )
    .await;
}

#[tokio::test]
#[should_panic(expected = "there are unexpected requests")]
async fn unanticipated_request() {
    let _ = env_logger::try_init();

    let mock = Server::bind().await;
    let url = format!("http://localhost:{}/foo", mock.port());

    let response = reqwest::get(url).await.unwrap();
    let bytes = response.bytes().await.unwrap();
    assert_eq!(&bytes[..], b"unexpected");
}

#[tokio::test]
#[should_panic(expected = "already anticipating")]
async fn can_not_anticipate_twice() {
    let _ = env_logger::try_init();

    let mock = Server::bind().await;

    mock.anticipate("/foo".to_string()).await;
    mock.anticipate("/foo".to_string()).await;
}