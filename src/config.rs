use std::fs;
use std::io::Error;
use serde::Deserialize;
use termion::color;
use regex::Regex;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RawConfig {
    status_line_bg_color: String,
    status_line_fg_color: String,
}

impl RawConfig {
   fn to_config(&self) -> Result<Config, Error> {
       let result = Config {
           status_line_fg_color: parse_rgb_string(&self.status_line_fg_color)?,
           status_line_bg_color: parse_rgb_string(&self.status_line_bg_color)?,
       };

       Ok(result)
   }
}

fn parse_rgb_string(string: &str) -> Result<color::Rgb, Error> {
    let re = Regex::new(r"rgb\((\d{1,3}),\s?(\d{1,3}),\s?(\d{1,3})\)").unwrap();
    let captures = re.captures(string).expect("");

    let red: u8 = captures.get(1).unwrap().as_str().parse().unwrap();
    let green: u8 = captures.get(2).unwrap().as_str().parse().unwrap();
    let blue: u8 = captures.get(3).unwrap().as_str().parse().unwrap();

    Ok(color::Rgb(red, green, blue))
}

#[derive(Debug)]
pub struct Config {
    status_line_bg_color: color::Rgb,
    status_line_fg_color: color::Rgb,
}

impl Config {
    pub fn load(filename: &str) -> Result<Config, std::io::Error> {
        let raw_config = Config::load_raw(filename)?;
        let config = raw_config.to_config()?;

        Ok(config)
    }

    fn load_raw(filename: &str) -> Result<RawConfig, std::io::Error> {
        let content = fs::read_to_string(filename)?;
        let raw_config: RawConfig = serde_json::from_str(&content)?;

        Ok(raw_config)
    }
}