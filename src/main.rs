use std::error::Error;
use std::fs;
use std::io::BufReader;
use std::path::Path;
use regex::Regex;

fn main()-> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all("./new_dir");
    let dir = std::fs::read_dir(".").unwrap();
    let files = dir.filter(|file|
        file.as_ref().unwrap().file_name().to_str().unwrap().ends_with("csv")).collect::<Vec<_>>();
    for file in files {
        let file_name = file.unwrap().file_name().to_string_lossy().into_owned();
        let source = "./".to_string() +   &file_name;
        let dest = "./new_dir/".to_string() +&file_name;
        println!("process file {:?}",&source);
        let result = process_file(&source, &dest);
        if let Err(e) = result {
                println!("file: {:?} process error: {:#?}",&source,e);
        }
    }
    Ok(())
}
fn process_file(sourceFile: &str, destFile: &str) -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let file = fs::File::open(sourceFile)?;
    let  reader = BufReader::new(file);
    let mut csv =  csv::Reader::from_reader(reader);
    let mut writer = csv::Writer::from_path(destFile)?;
    let re = Regex::new("\\[[^]]*\\]").unwrap();
    let mut headers = csv.headers().unwrap().clone();
    headers.push_field("city");
    headers.push_field("country");
    headers.push_field("university");
    writer.write_record(&headers);
    for line in csv.records() {
        let mut record = line?;
        println!("{:?}", record);
        let mut new_record = record.clone();
        let mut city ="".to_string();
        let mut county = "".to_string();
        let mut univ = "".to_string();
        let cell = record.iter().next().unwrap();
        // for cell in record.iter()
        {
            let result = re.replace_all(cell,"");
            println!("{:?}", result);
            let addr_list = result.split(";");
            let mut addr_list = addr_list.collect::<Vec<&str>>();
            let  addr = addr_list.iter().next().ok_or("add error")?;
            {
                println!("========{:?}", &addr);
                if addr.trim().is_empty() {
                    println!("{:?} is empty", &addr);
                    break;
                }
                let addr = addr.split(",");
                let mut addr =  addr.collect::<Vec<&str>>();
                let mut iter = addr.iter().rev();
                 county =(&mut iter).next().ok_or("county not found")?.to_owned().to_string();
                 city = (&mut iter).next().ok_or("city not found")?.to_owned().to_string();
                 univ = (&mut iter).last().ok_or("univ not found")?.to_string();
                println!("city:{:?},county:{:?}", city,county);

            }
        }
        new_record.push_field(city.as_str().trim());
        new_record.push_field(county.as_str().trim());
        new_record.push_field(univ.as_str().trim());
        writer.write_record(&new_record);
    }
    writer.flush();
    Ok(())
}



