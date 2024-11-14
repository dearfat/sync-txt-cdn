mod config;
use anyhow::Result;
use log::info;
use tokio;
use config::{Settings,TemplateConfig,TemplateConfigData};
use env_logger::Env;
use clap::Parser;
use serde::{Serialize,Deserialize};

#[allow(dead_code)]
#[derive(Parser,Debug,Clone,Serialize,Deserialize)]
struct BookCli{
    #[arg(short='b' ,long="book_id")]
    book_id:i32,
    #[arg(short='c' ,long="chapter_id")]
    chapter_id:i32,
}


#[tokio::main]
async fn main()->Result<()>{ 
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Hello, world!");
    let setting = Settings::new()?;
    let client = reqwest::Client::new();
    let cli = BookCli::parse();
    let tpl_config = TemplateConfig::new(setting.template_config.source_dir,setting.template_config.output_dir,setting.template_config.segmentation,setting.template_config.file_name,setting.template_config.url);
    // example: book_id: 27 chapter_id 0
    let template = TemplateConfigData{
        book_id:cli.book_id,
        chapter_id:cli.chapter_id,
        title:None,
        content:None,
        data:None,
        };
    let tpl_data = template.with_req_data(&client,&tpl_config.url).await?;
    tpl_config.new_with_data_file(&tpl_data)?;
    info!("Hello, world End!");
    Ok(())
}
