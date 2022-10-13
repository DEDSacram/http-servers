use std::io;

use may_minihttp::{HttpServer, HttpService, Request, Response};

#[derive(Clone)]
struct StatusService;

impl HttpService for StatusService {
    fn call(&mut self, req: Request, rsp: &mut Response) -> io::Result<()> {
        let (code, message) = match req.path() {
            "/test" => ("200", "result:Hello World"),
            _ => ("404", "Not Found"),
        };

        rsp.status_code(code, message);
        rsp.body(message);
        Ok(())
    }
}

fn main() {
    env_logger::init();
    let server = HttpServer(StatusService).start("127.0.0.1:8080").unwrap();
    server.join().unwrap();
}
