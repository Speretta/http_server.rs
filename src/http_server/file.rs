use std::{ fs::{self}, sync::Arc};


pub struct HttpFileSystem{
    files: Vec<Arc<HttpFile>>
}

impl HttpFileSystem {
    pub(super) fn new() -> Self{
        HttpFileSystem{files: Vec::new()}
    }

    pub fn add_file<S: Into<String>>(&mut self, file_path: S) -> Option<Arc<HttpFile>>{
        if let Some(file) = HttpFile::new(file_path){
            let idx = self.files.iter().position(|temp_file| temp_file.get_path() == file.get_path()).unwrap_or_else(|| {
                self.files.push(Arc::new(file));
                self.files.len() - 1    
            });
            return Some(Arc::clone(&self.files[idx]));
        }
        None
    }
    pub fn add_folder<S: Into<String>>(&mut self, file_path: S){
        let file_path = file_path.into();
        if let Ok(files) = fs::read_dir(file_path){
            files.into_iter().for_each(|file| if let Ok(file) = file{self.add_file(file.path().to_string_lossy());});
        }
    }

    pub fn add_files<S: Into<String>>(&mut self, file_paths: Vec<S>){
        for file_path in file_paths{
            self.add_file(file_path);
        }
    }

    pub fn get_file<S: Into<String>>(&self, file_path: S) -> Option<Arc<HttpFile>>{
        let mut file_path: String = file_path.into();
        file_path = if file_path.starts_with("./"){file_path[2..].to_string()}else{file_path};
        if let Some(file) = self.get_files().into_iter().filter(|file| file.get_path() == &file_path).last(){
            return Some(file.clone());
        }
        None
    }
    pub fn get_files(&self) -> &Vec<Arc<HttpFile>>{
        &self.files
    }
}



pub struct HttpFile{
    path: String,
    content: Vec<u8>,
    str_content: Option<String>
}


impl HttpFile {
    pub(super) fn new<S: Into<String>>(path: S) -> Option<Self> {
        let mut path: String = path.into();
        path = if path.starts_with("./"){path[2..].to_string()}else{path};
        match fs::read(&path){
            Ok(bytes) => {
                Some(HttpFile{path, content: bytes.clone(), str_content: 
                    match String::from_utf8(bytes){
                        Ok(content) => {Some(content)}
                        Err(_) => {None}
                    }
                })         
            },
            Err(_) => {return None;}
        }
    }
    pub(super) fn new_empty() -> Self{
        HttpFile{path: String::new() ,content: Vec::new(), str_content: Some(String::new())}
    }

    pub fn get_string_content(&self) -> Option<&String>{
        if let Some(content) = &self.str_content{
            Some(content)
        }else{
            None
        }
    }

    pub fn get_content(&self) -> &Vec<u8>{
        &self.content
    }

    pub fn get_path(&self) -> &String{
        &self.path
    }
}

