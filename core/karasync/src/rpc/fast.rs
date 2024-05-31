pub mod ssh {
    use ssh2::Session;
    use std::net::TcpStream;

    #[allow(dead_code)]
    pub fn new(
        ip: String,
        port: u16,
        user: String,
        password: String,
    ) -> Result<ssh2::Session, ssh2::Error> {
        let mut session = Session::new().unwrap();
        let addr = format!("{}:{}", ip, port);
        let tcpsession = TcpStream::connect(addr).unwrap();
        session.set_tcp_stream(tcpsession);
        session.handshake()?;
        session.userauth_password(user.as_str(), password.as_str())?;
        Ok(session)
    }
}
