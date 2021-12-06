use regex::Regex;
use std::error::Error;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let dir = args.nth(1).unwrap_or("./".to_string());
    println!("work dir: {:?}", dir);
    let work_dir = Path::new(&dir);
    let output_dir = &work_dir.join("new_dir");
    fs::create_dir_all(output_dir)?;
    let dir = fs::read_dir(work_dir).unwrap();
    let files = dir
        .filter(|file| {
            file.as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .ends_with("csv")
        })
        .collect::<Vec<_>>();
    for file in files {
        let source = file.unwrap().path();
        let dest = &output_dir.join(&source.file_name().unwrap());
        println!("process file {:?}", &source);
        let result = process_file(&source, &dest);
        if let Err(e) = result {
            println!("file: {:?} process error: {:#?}", &source, e);
        }
    }
    Ok(())
}
fn process_file(source_file: &PathBuf, dest_file: &PathBuf) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(source_file)?;
    let reader = BufReader::new(file);
    let mut csv = csv::Reader::from_reader(reader);
    let mut writer = csv::Writer::from_path(dest_file)?;
    let re = Regex::new("\\[[^]]*]").unwrap();
    let mut headers = csv.headers().unwrap().clone();
    headers.push_field("city");
    headers.push_field("country");
    headers.push_field("university");
    writer.write_record(&headers)?;
    for line in csv.records() {
        let record = line?;
        println!("{:?}", record);
        let mut new_record = record.clone();
        let cell = record.iter().next().unwrap();
        // for cell in record.iter()
        {
            let result = re.replace_all(cell, "");
            println!("{:?}", result);
            let addr_list = result.split(";");
            let addr_list = addr_list.collect::<Vec<&str>>();
            let addr = addr_list.iter().next().ok_or("add error")?;
            {
                println!("========{:?}=======", &addr);
                if addr.trim().is_empty() {
                    println!("addr is empty");
                    break;
                }
                let addr = addr.split(",");
                let addr = addr.collect::<Vec<&str>>();
                let mut iter = addr.iter().rev();
                let county = (&mut iter).next().ok_or("county not found")?;
                let city = (&mut iter).next().ok_or("city not found")?;
                let univ = (&mut iter).last().ok_or("univ not found")?;
                println!("City:{:?},County:{:?},Univ:{:?}", city, county, univ);
                new_record.push_field(city.trim());
                new_record.push_field(county.trim());
                new_record.push_field(univ.trim());
                writer.write_record(&new_record)?;
            }
        }
    }
    Ok(())
}
