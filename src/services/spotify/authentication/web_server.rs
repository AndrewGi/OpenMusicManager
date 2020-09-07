use crate::services::spotify::authentication::AuthCode;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

pub fn error_response() -> http::Response<String> {
    http::Response::builder()
        .status(http::StatusCode::INTERNAL_SERVER_ERROR)
        .body(String::new())
        .expect("hard coded response generated")
}
pub fn good_response() -> http::Response<String> {
    http::Response::builder()
        .status(http::StatusCode::OK)
        .body(
            "
		<script>
			window.close();
		</script>
	"
            .to_owned(),
        )
        .expect("hard coded response")
}
pub fn response_to_string(res: http::Response<impl AsRef<str>>) -> String {
    use std::fmt::Write;
    let mut buf = String::new();
    // "HTTP/1.1 200 OK"
    write!(buf, "{:?} {}\r\n", res.version(), res.status()).expect("strings never fail writes");
    // "Header_Name: HeaderValue
    for (name, value) in res.headers().iter() {
        write!(
            buf,
            "{}: {}\r\n",
            name.as_str(),
            value.to_str().expect("invalid response header")
        )
        .expect("strings never fail writes");
    }
    // Before Body
    buf.push_str("\r\n");
    buf.push_str(res.body().as_ref());
    buf
}
pub async fn get_code(port: u16) -> Result<AuthCode, std::io::Error> {
    let mut server = tokio::net::TcpListener::bind(std::net::SocketAddr::new(
        std::net::IpAddr::from([127, 0, 0, 1]),
        port,
    ))
    .await?;
    loop {
        let (connection, _addr) = server.accept().await?;
        let mut connection = BufReader::new(connection);
        let mut line = String::new();
        connection.read_line(&mut line).await?;
        // "GET URL_HERE HTTP/1.1\r\n"
        if line.starts_with("GET ") {
            let line_part = &line["GET ".len()..];
            let space_pos = line_part.find(" "); //
            let url = &line_part[..space_pos.unwrap_or(line_part.len())];
            connection
                .write_all(response_to_string(good_response()).as_bytes())
                .await?;
            #[derive(serde::Deserialize)]
            struct Code {
                code: String,
            }
            let code: Code = serde_urlencoded::from_str(&url["/?".len()..])
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
            return Ok(AuthCode(code.code));
        } else {
            connection
                .write_all(response_to_string(error_response()).as_bytes())
                .await?;
            continue;
        }
    }
}
