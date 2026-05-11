use std::net::TcpStream;

pub struct BeanstalkdServer {
    tubes: Vec<&'static str>,
    clients: Vec<TcpStream>,
}
impl BeanstalkdServer {
    pub fn new() -> Self {
        BeanstalkdServer {
            tubes: ["default"].to_vec(),
            clients: Vec::new(),
        }
    }
    pub fn add_tube(&mut self, tube: &'static str) {
        self.tubes.push(tube);
    }
    pub fn delete_tube(&mut self, tube: &'static str) {
        self.tubes.retain(|&t| t != tube);
    }
    pub fn list_tubes(&self) -> Vec<&'static str> {
        self.tubes.clone()
    }
    pub fn add_client(&mut self, client: TcpStream) {
        self.clients.push(client);
    }
    pub fn remove_client(&mut self, client: TcpStream) {
        self.clients
            .retain(|c| c.peer_addr().unwrap() != client.peer_addr().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_server() {
        let server = BeanstalkdServer::new();
        assert_eq!(server.tubes, ["default"]);
        assert_eq!(server.clients.len(), 0);
    }

    #[test]
    fn test_add_tube() {
        let mut server = BeanstalkdServer::new();
        server.add_tube("test");
        assert!(server.tubes.contains(&"test"));
    }

    #[test]
    fn test_delete_tube() {
        let mut server = BeanstalkdServer::new();
        server.add_tube("test");
        server.delete_tube("test");
        assert!(!server.tubes.contains(&"test"));
    }

    #[test]
    fn test_list_tubes() {
        let mut server = BeanstalkdServer::new();
        server.add_tube("test");
        assert_eq!(server.list_tubes(), ["default", "test"]);
    }

    #[test]
    fn test_add_client() {
        let mut server = BeanstalkdServer::new();
        server.add_client(create_tcp_stream());
        assert_eq!(server.clients.len(), 1);
    }
    #[test]
    fn test_remove_client() {
        let mut server = BeanstalkdServer::new();
        let client = create_tcp_stream();
        server.add_client(client);
        server.remove_client(server.clients[0].try_clone().unwrap());
        assert_eq!(server.clients.len(), 0);
    }
    fn create_tcp_stream() -> TcpStream {
        TcpStream::connect("8.8.8.8:53").unwrap()
    }
}
