use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

use self::{file::HttpFileSystem, request::HttpRequest, response::HttpResponse};
pub mod response;
pub mod file;
pub mod request;

pub struct HttpServer<'closure>{
    host: String,
    files: file::HttpFileSystem,
    path_and_response: Vec<((String, bool), Box<dyn Fn(&mut HttpResponse, &HttpRequest)+ 'closure>)>,
    not_found_response: Box<dyn Fn(&mut HttpResponse, &HttpRequest)+ 'closure>,
}

impl<'closure> HttpServer<'closure>
{
    pub fn new<S: Into<String>>(ip: S, port: u16) -> Self{
        let ip = ip.into();
        HttpServer{host: format!("{}:{}", &ip, port), files: file::HttpFileSystem::new(), path_and_response: Vec::new(), not_found_response: Box::new(|response, request| {response.send("404")})}
    }

    

    fn handle_client(&mut self,  mut stream: TcpStream){
        let mut buf = [0; 1024];
        stream.read(&mut buf).unwrap();
        let buf: Vec<u8> = buf.to_vec().into_iter().filter(|x| *x!=0).collect();
        let request_string = String::from_utf8(buf);
        if let Ok(request_string) = request_string{
            let request = HttpRequest::new(&request_string);
            let mut founded_path = false;
            'path_looping: for (path,  func) in &mut self.path_and_response{
                let (eq, variables) = request.compare_path(path.1, &path.0);
                if eq{
                    let mut func_response= response::HttpResponse::new();
                    if let Some(variables) = variables{
                        variables.into_iter().for_each(|var| func_response.add_variabe(var));

                    }
                    func(&mut func_response, &request);
                    stream.write(func_response.get_http_raw().as_bytes()).unwrap();
                    
                    founded_path = true;
                    break 'path_looping;
                }
            }
            if !founded_path{
                let mut func_response = response::HttpResponse::new();
                (self.not_found_response)(&mut func_response, &request);
                stream.write(func_response.get_http_raw().as_bytes()).unwrap();
                
            }
        }
    }

    pub fn start_listen(&mut self){
        let listener = TcpListener::bind(&self.host);
        match listener{
        Ok(listener) => {
                for client in listener.incoming(){
                    match client {
                        Ok(stream) => {
                            println!("{:?}", &stream);
                            self.handle_client(stream);
                        },
                        Err(e) => {print!("İstemci bağlanamadı: {}", e)},
                    }
                }
            }
            Err(err) => {panic!("Sunucu oluşturulurken hata oluştu: {}", err);}
        }
    }

    pub fn get<F: Fn(&mut HttpResponse, &HttpRequest) + 'closure ,S: Into<String>>(&mut self, path: S, closure: F){
        self.path_and_response.push(((path.into(), false), Box::new(closure)));
    }

    pub fn get_with_variables<F: Fn(&mut HttpResponse, &HttpRequest) + 'closure, S: Into<String>>(&mut self, path: S, closure: F){
        self.path_and_response.push(((path.into(), true), Box::new(closure)));
    }


    pub fn not_found_page<F: Fn(&mut HttpResponse, &HttpRequest) + 'closure>(&mut self, closure: F){
        self.not_found_response = Box::new(closure);
    }


    pub fn get_file_system(&mut self) -> &mut HttpFileSystem{
        &mut self.files
    }
}

pub enum HttpMethod{
    GET,
    POST,
    Custom(String)
}

impl HttpMethod {
    fn to_string(&self) -> String{
        match self{
            HttpMethod::GET => {String::from("GET")}
            HttpMethod::POST => {String::from("POST")}
            HttpMethod::Custom(value) => {value.to_string()}
        }
    }

    fn from_string<S: Into<String>>(http_type: S) -> Self{
        let http_type:String = http_type.into();
        match http_type.as_str() {
            "GET" => {HttpMethod::GET}
            "POST" => {HttpMethod::POST},
            _ => {HttpMethod::Custom(http_type)}
        }
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


pub enum ResponseStatus{
    OK,
    NotFound,
    Custom(String),
}

impl std::fmt::Display for ResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self{
            ResponseStatus::OK => {"200 OK"}
            ResponseStatus::NotFound => {"404 Not Found"}
            ResponseStatus::Custom(value) => {value}
        })
    }
}