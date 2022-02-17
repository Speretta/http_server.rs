use std::sync::Arc;

use super::{ResponseStatus, file::HttpFile};





pub struct HttpResponse{
    headers: Vec<String>,
    pub(super) raw_content: String,
    status: ResponseStatus,
    path_variabe: Vec<HttpVariable>,
}

impl HttpResponse{

    pub(super) fn new() -> Self{
        HttpResponse{status: ResponseStatus::OK, headers: Vec::new(), raw_content: String::new(),path_variabe: Vec::new()}
    }

    pub fn send_file(&mut self, http_file: &Arc<HttpFile>){
        if let Some(content) = http_file.get_string_content(){
            self.set_http_raw(content);
        }
        
    }

    pub fn send<S: Into<String>>(&mut self, temp_content: S){
        self.set_http_raw(&temp_content.into());
    }
    pub fn set_status(&mut self, status: ResponseStatus){
        self.status = status;
    }
    pub fn add_header<S: Into<String>>(&mut self, header: S){
        self.headers.push(header.into());
    }


    pub(super) fn set_http_raw(&mut self, content: &String){
        let mut raw_header = String::from("HTTP/1.1 ");
        raw_header.push_str(format!("{}", self.status).as_str());
        raw_header.push_str("\r\n");

        if self.headers.len() > 0{
            raw_header.push_str(self.headers.join("\r\n").as_str());
            raw_header.push_str("\r\n");
        }
        raw_header.push_str(format!("Content-Length: {}\r\n\r\n{}",content.len(), content).as_str());
        
        self.raw_content = raw_header;
    }

    pub(super) fn get_http_raw(&self) -> &String{
        &self.raw_content
    }

    pub fn get_variable<S: Into<String>>(&self, variable_name: S) -> String{
        let variable_name = variable_name.into();
        for variable in &self.path_variabe{
            if variable.name == variable_name{
                return variable.to_string();
            }
        }
        String::new()
    }

    pub(super) fn add_variabe(&mut self, data: (String, String)){
        if let None = self.path_variabe.iter().find(|variable| variable.name == data.0){
            let mut http_variable = HttpVariable::new(data.0);
            http_variable.set_value(data.1);
            self.path_variabe.push(http_variable)
        }
    }
}



pub struct HttpVariable{
    name: String,
    value: Option<String>,
}

impl HttpVariable{
    fn new(name: String) ->Self{
        HttpVariable{name, value: None}
    }

    fn set_value<S: Into<String>>(&mut self, value: S){
        let value:String = value.into();
        if value.is_empty(){
            self.value = None;
        }else{
            self.value = Some(value.into());
        }
        
    }
}


impl std::fmt::Display for HttpVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match &self.value{
            Some(value) => {value.clone()},
            None => {String::new()},
        })
    }
}