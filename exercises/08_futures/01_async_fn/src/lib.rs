use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

//  `echo` should not return when it finishes processing a connection, but should
//  continue to accept new connections.
//
// Hint: you should rely on `tokio`'s structs and methods to implement the echo server.
// In particular:
// - `tokio::net::TcpListener::accept` to process the next incoming connection
// - `tokio::net::TcpStream::split` to obtain a reader and a writer from the socket
// - `tokio::io::copy` to copy data from the reader to the writer
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut strm, addr) = listener.accept().await?;
        let (mut reader, mut writer) = strm.split();
        let mut buffer = [0; 1024];
        loop {
            let n = reader.read(&mut buffer).await?;
            if n > 0 {
                writer.write_all(&buffer[..n]).await?;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_echo() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(echo(listener));

        let requests = vec!["hello", "world", "foo", "bar"];

        for request in requests {
            let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
            let (mut reader, mut writer) = socket.split();

            // Send the request
            writer.write_all(request.as_bytes()).await.unwrap();
            // Close the write side of the socket
            writer.shutdown().await.unwrap();

            // Read the response
            let mut buf = Vec::with_capacity(request.len());
            reader.read_to_end(&mut buf).await.unwrap();
            assert_eq!(&buf, request.as_bytes());
        }
    }
}
