use super::{HttpMethod};



pub struct HttpRequest{
	headers: Vec<String>,
	method: HttpMethod,
	path: String,
	host: String,
	http_version: String,
}


impl HttpRequest {
	pub(super) fn new(request_string: &String) -> Self{
		let mut values: (HttpMethod, String, String, String) = (HttpMethod::Custom(String::new()), String::new(), String::new(), String::new());
		let mut headers: Vec<String> = Vec::new();
		for (i, raw_header) in request_string.lines().filter(|s| !s.is_empty()).enumerate(){
			if i == 0{
				let mut a: Vec<String> = raw_header.splitn(2, " ").map(|s| s.to_string()).collect();
				if a.len() == 2 {
					values.0 = HttpMethod::from_string(&a[0]);
					let mut b =  a[1].clone();
					'http_version: for ch in a[1].chars().rev(){
		                if ch.is_numeric() || ch == '/' || ch == 'H' || ch == 'T' || ch == 'P' || ch == '.'{
			                values.3.insert(0, b.pop().unwrap_or(ch));
			            }else {
			                break 'http_version;
			            }
			        }

			        // %F0%9F%98%ADa%F0%9F%98%A4
			        let mut unicode_list: Vec<(usize, Vec<u8>)> = Vec::new();
			        let mut unicode = String::new();
			        let mut space = true;
			        for (i,ch) in b.trim().chars().enumerate(){
			        	match ch{
			        		'%' => {if space{unicode_list.push((i, Vec::new()));} unicode = String::new();},
			        		_ => {
			        			
			        			if let Some(last) = unicode_list.last_mut(){

			        				if unicode.len() < 2 {
			        					unicode.push(ch);
			        					space = false;
			        					if unicode.len() == 2 {
			        						if let Ok(hex) = u8::from_str_radix(unicode.as_str(), 16){
			        							last.1.push(hex);
			        						}
			        					}
			        					
			        				}else{
			        					space = true;
			        				}
			        			}
			        		},
			        	}
			        }
			        values.1 = b.trim().to_string();
			        let mut delta = 0usize;
			        println!("{:?}", unicode_list);
			        for (from, hex) in unicode_list{
			        	let a = hex.len()*3;
			        	if let Ok(text) = String::from_utf8(hex){
			        		let from = from - delta;
			        		delta = 0;
			        		for mut i in from..from+a{
			        			i -= delta; 
			        			print!("{},", values.1.remove(i));
			        			delta+=1;
			        		}
			        		
			        		println!("TEXT: {}|{}|{}|{}|{:?}", from, a, delta, text, &values.1[from-2..from+2]);
			        		delta -= text.len();
			        		println!("TEXT: {}|{}|{}|{}|{:?}", from, a, delta, text, values.1);
			        		values.1.insert_str(from,&text);

			        	}
			        	
			        }
			        println!("{}", values.1);
			        
				}
				
			}else if raw_header.starts_with("Host:"){
				values.2 = raw_header.splitn(2, "Host: ").last().unwrap_or("").to_string();
			}else{
				headers.push(raw_header.to_string());
			}


		}
		HttpRequest{ headers, method: values.0,	path: values.1, host: values.2, http_version: values.3}
	}
	pub fn get_http_version(&self) -> &String{
		&self.http_version
	}

	pub fn get_headers(&self) -> &Vec<String>{
		&self.headers
	}

	pub fn get_path(&self) -> &String{
		&self.path
	}

	pub fn get_host(&self) -> &String{
		&self.host	
	}

	pub fn get_method(&self) -> &HttpMethod{
		&self.method
	}

	pub fn get_header<S: Into<String>>(&self, header_name: S) -> Option<String>{
		let mut header_name:String = header_name.into();
		if header_name.ends_with(":"){
			header_name.pop();
		}
		for header in &self.headers{
			let header_and_value = header.splitn(2, ": ").map(|value| value.to_string()).collect::<Vec<String>>();
			if header_and_value[0] == header_name{
				return Some(header_and_value[1].to_string());
			}
		}
		None
	}

	pub(super) fn compare_path<S: Into<String>>(&self, is_variable_path: bool, other_path: S) -> (bool, Option<Vec<(String, String)>>){
		let mut other_path:String = other_path.into();
		if !is_variable_path{
			(self.path == other_path, None)
		}else{
			let mut variables:Vec<(String, String)> = vec![(String::new(),String::new())];
			let mut self_path = self.path.clone();
            let mut path_slice : Vec<String> = vec![String::new()];
            for selfch in other_path.chars(){
                    match selfch{
                    '{' => {variables.last_mut().unwrap().0.push(selfch); path_slice.push(String::new())},
                    '}' => {variables.last_mut().unwrap().0.push(selfch); variables.push((String::new(),String::new()));},
                    _ => {
                        if let Some(last_variable) = variables.last_mut(){
                            if last_variable.0.is_empty(){
                                path_slice.last_mut().unwrap().push(selfch);
                            }else{
                                last_variable.0.push(selfch);
                            }
                        }
                    },
                }
               
            }
            variables.remove(variables.len()-1);
            for path_part in path_slice{
                if !self_path.contains(&path_part){
                    return (false, None);
                }
                self_path = self_path.splitn(2, &path_part).into_iter().filter(|txt| !txt.is_empty()).collect::<Vec<&str>>().join(" ");
            }
            for (i, variable_value) in self_path.split(" ").map(|x| x.to_string()).enumerate(){
                if let Some(variable) = variables.get_mut(i){
                    variable.1 = variable_value;
                }
            }
            return (true, Some(variables));
		}
	}
}