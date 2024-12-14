use config::{Config,ConfigError,Environment,File};
use serde::{Deserialize, Serialize};
use tracing_subscriber::fmt::format;
use std::{io::Read, path::{Path,PathBuf}, vec};
use log::{debug,error,info,warn};
use anyhow::{Context, Ok, Result};
use reqwest::{ Client, Response, StatusCode, Url};
use std::fs;
#[derive(Debug,Deserialize,Clone,Serialize)]
pub struct Settings{
	pub template_config:TemplateConfig,
}

#[derive(Debug,Deserialize,Clone,Serialize)]
pub struct TemplateConfig{
	pub source_dir: String,
	pub output_dir: String,
	pub segmentation:String,
	pub file_name:String,
	pub url:String,
}

#[derive(Debug,Deserialize,Clone,Serialize)]
pub struct TemplateConfigData{
	pub book_id:i32,
	pub chapter_id:i32,
	pub title:Option<String>,
	pub content:Option<String>,
	pub data: Option<serde_json::Value>,
}

#[allow(dead_code)]
impl Settings{
	pub fn new() -> Result<Settings,ConfigError>{
		let env = std::env::var("run_mode").unwrap_or_else(|_|"dev".to_string());
		let s = Config::builder()
			.add_source(File::with_name("config/default").required(true))
			.add_source(File::with_name(&format!("config/{}",env)).required(true))
			.add_source(Environment::with_prefix("APP"))
			.build()?;
		s.try_deserialize()
	}
}


// const FILE_NAME_TPL:&str = "{book_id}_{chapter_id}";

#[allow(dead_code)]
impl TemplateConfig {
	pub fn new(source_dir:String,output_dir:String,segmentation:String,file_name:String,url:String)->Self{
		Self{
			source_dir,
			output_dir,
			segmentation,
			file_name,
			url,
		}
	}

	pub fn source_path(&self) -> Result<PathBuf>{
		self.normalize_path(&self.source_dir)
	}
	pub fn output_path(&self) -> Result<PathBuf>{
		self.normalize_path(&self.output_dir)
	}

	pub fn source_file(&self)->Result<String>{
		let file = self.file_name.as_str();
		if self.source_dir.is_empty() {
			return Err(anyhow::anyhow!("Source directory is empty"));
		}
		if file.is_empty(){
			error!("File name is empty {}",&file);
			return Err(anyhow::anyhow!("File name is empty"))
		}
		let source_dir =  self.source_path()?;
		let full_path = Path::new(&source_dir).join(&file);
		let rst = full_path.canonicalize().context("Faild to source resolve absolute path")?.to_string_lossy().to_string();
		if !full_path.exists(){
			error!("File name is empty {}",full_path.display());
			return Err(anyhow::anyhow!("File name is empty"))
		}
			Ok(rst)
	}

	pub fn output_file(&self,tcdata:&TemplateConfigData) -> Result<String>{
		if self.output_dir.is_empty() || self.file_name.is_empty(){
			return Err(anyhow::anyhow!("Output directory is empty"));
		}
		// let source_file = self.source_file()?;
		let file_names  = self.file_name.split(".").collect::<Vec<&str>>();
		let file_name = format!("{}_{}_{}.{}",&file_names[0],tcdata.book_id,tcdata.chapter_id,&file_names[1]);
		let out_path = self.output_path()?;
		let full_path = std::env::current_dir()?.join(out_path).join(file_name);
		info!("Output file path 1: {}",full_path.display());
		let  rst = full_path.to_string_lossy().to_string();
		info!("Output file path 2: {}",rst);
		if full_path.exists(){ // 
			warn!("File already exists {}",full_path.display());
			return  Ok(rst);
		}
		// .context("Faild to resolve absolute path")
		Ok(rst)
	}


	fn normalize_path(&self,path:&str)->Result<PathBuf>{
		let path = Path::new(path);
		let full_path = match path.is_absolute() {
			true => path.to_path_buf(),
			false => std::env::current_dir()?.join(path),
		};
		Ok(full_path)
	}

	pub fn new_with_data_file(&self,data:&TemplateConfigData) -> Result<(),anyhow::Error>{

			let output_file = self.output_file(&data)?;
			let input_file = self.source_file()?;
			let mut source = fs::File::open(input_file)?;
			let mut content = String::new();
			source.read_to_string(&mut content)?;
			
			let data = match  serde_json::to_string(data) {
				Result::Ok(data) => data,
				Err(e) => {
					error!("Failed to serialize data: {}",e);
					return Err(anyhow::anyhow!("Failed to serialize data"));
				}
			};
			let _data = format!("var BOOK_CACHE_DATA={} ;",data);
			let content = content.replace(&self.segmentation, &_data);
			fs::write(output_file, content)?;
			Ok(())
	}



}


#[allow(dead_code)]
impl  TemplateConfigData {
	pub fn new(book_id:i32,chapter_id:i32)->Self{
		Self{
			book_id,
			chapter_id,
			title:None,
			content:None,
			data:None,
		}
	}
	pub fn with_data(mut self,data:serde_json::Value)->Self{
		self.data = Some(data);
		self
	}
	pub fn with_title(mut self,title:&str)->Self{
		self.title = Some(title.to_string());
		self
	}
	pub fn with_content(mut self,content:&str)->Self{
		self.content = Some(content.to_string());
		self
	}
	pub fn with_title_content(mut self,title:&str,content:&str)->Self{
		self.title = Some(title.to_string());
		self.content = Some(content.to_string());
		self
	}
	pub fn with_title_content_data(mut self,title:&str,content:&str,data:serde_json::Value)->Self{
		self.title = Some(title.to_string());
		self.content = Some(content.to_string());
		self.data = Some(data);
		self
	}

	pub async  fn with_req_data(mut self,client:&Client,url:&str)->Result<Self>{
		let local_url = &format!("{}/api/v1/chapter/{}/{}?app_id={}&uuid={}",url ,&self.book_id , &self.chapter_id,"novellettes","8413d93a-1512-4cca-b9e3-5ea520c71240");
		let url = Url::parse(&local_url)?;
		info!("Fetching data from {:?}",url);
		let resp = client.get(url).send().await?;
		if resp.status() == StatusCode::OK{
			let data:serde_json::Value = resp.text().await?.parse()?;
			self.data = Some(data);
			info!("Data fetched successfully:");
		}
		Ok(self)
	}

}