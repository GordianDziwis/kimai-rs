use clap::crate_name;
use prettytable::{cell, format, row, Table};
use reqwest::header::{self, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;
use std::process::Command;

trait QueryValue {
    fn process(&self) -> String;
}

impl QueryValue for Vec<usize> {
    fn process(&self) -> String {
        self.iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl QueryValue for &str {
    fn process(&self) -> String {
        self.to_string()
    }
}

macro_rules! query{
    ($(($key:expr, $value:expr)),*) => {{
        let mut queries = HashMap::new();
        $(if let Some(v) = $value {
            queries.insert($key, QueryValue::process(&v));
        };)*
        if queries.is_empty() {
            None
        } else {
            Some(queries)
        }
    }}
}

#[derive(Debug)]
pub enum KimaiError {
    XdgBaseDirectories(String),
    IO(String),
    Toml(String),
    Utf8(String),
    Reqwest(String),
    Config(String),
    Other(String),
}

impl std::error::Error for KimaiError {}

impl fmt::Display for KimaiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KimaiError::XdgBaseDirectories(e) => write!(f, "XDG BaseDirectories Error: {}", e),
            KimaiError::IO(e) => write!(f, "IO Error: {}", e),
            KimaiError::Toml(e) => write!(f, "TOML Error: {}", e),
            KimaiError::Utf8(e) => write!(f, "UTF-8 Error: {}", e),
            KimaiError::Reqwest(e) => write!(f, "Reqwest Error: {}", e),
            KimaiError::Config(e) => write!(f, "Config Error: {}", e),
            KimaiError::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

impl From<xdg::BaseDirectoriesError> for KimaiError {
    fn from(error: xdg::BaseDirectoriesError) -> KimaiError {
        KimaiError::XdgBaseDirectories(error.to_string())
    }
}

impl From<std::io::Error> for KimaiError {
    fn from(error: std::io::Error) -> KimaiError {
        KimaiError::IO(error.to_string())
    }
}

impl From<toml::de::Error> for KimaiError {
    fn from(error: toml::de::Error) -> KimaiError {
        KimaiError::Toml(error.to_string())
    }
}

impl From<std::str::Utf8Error> for KimaiError {
    fn from(error: std::str::Utf8Error) -> KimaiError {
        KimaiError::Utf8(error.to_string())
    }
}

impl From<reqwest::Error> for KimaiError {
    fn from(error: reqwest::Error) -> KimaiError {
        KimaiError::Reqwest(error.to_string())
    }
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    host: String,
    user: String,
    password: Option<String>,
    pass_path: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    host: String,
    user: String,
    password: String,
}

impl Config {
    pub fn new(host: String, user: String, password: String) -> Self {
        Config {
            host,
            user,
            password,
        }
    }
    pub fn from_path(path: &Path) -> Result<Self, KimaiError> {
        let config_string = fs::read_to_string(path)?;
        let config_file = toml::from_str::<ConfigFile>(&config_string)?;
        if let Some(p) = config_file.password {
            Ok(Config {
                host: config_file.host,
                user: config_file.user,
                password: p,
            })
        } else if let Some(p) = config_file.pass_path {
            let pass_cmd = Command::new("pass").arg(p).output()?;
            Ok(Config {
                host: config_file.host,
                user: config_file.user,
                password: std::str::from_utf8(&pass_cmd.stdout)?.trim().into(),
            })
        } else {
            Err(KimaiError::Config(
                "No password give in config!".to_string(),
            ))
        }
    }

    pub fn from_xdg() -> Result<Self, KimaiError> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(crate_name!())?;
        let config_path = xdg_dirs
            .find_config_file("config.toml")
            .ok_or_else(|| KimaiError::Config("config file not found!".to_string()))?;
        Self::from_path(Path::new(&config_path))
    }
}

fn get_headers(config: &Config) -> Result<header::HeaderMap, KimaiError> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-auth-user"),
        HeaderValue::from_str(&config.user).unwrap(),
    );
    headers.insert(
        HeaderName::from_static("x-auth-token"),
        HeaderValue::from_str(&config.password).unwrap(),
    );
    Ok(headers)
}

fn make_get_request(
    config: &Config,
    api_endpoint: &str,
    parameters: Option<HashMap<&str, String>>,
) -> Result<reqwest::blocking::Response, KimaiError> {
    let url = format!("{}/{}", config.host, api_endpoint);
    let mut request_builder = reqwest::blocking::Client::builder()
        .default_headers(get_headers(config)?)
        .build()?
        .get(&url);
    if let Some(p) = parameters {
        request_builder = request_builder.query(&p);
    }
    Ok(request_builder.send()?)
}

fn load_config(config_path: Option<String>) -> Result<Config, KimaiError> {
    match config_path {
        Some(p) => Config::from_path(Path::new(&p)),
        None => Config::from_xdg(),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Customer {
    id: usize,
    name: String,
    visible: bool,
    color: String,
}

pub fn get_customers(config: &Config) -> Result<Vec<Customer>, KimaiError> {
    let response = make_get_request(config, "api/customers", None)?;
    Ok(response.json::<Vec<Customer>>()?)
}

pub fn print_customers(config_path: Option<String>) -> Result<(), KimaiError> {
    let config = load_config(config_path)?;
    let customers = get_customers(&config)?;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["ID", "Name"]);
    for customer in customers {
        table.add_row(row![customer.id, customer.name]);
    }

    table.printstd();

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    id: usize,
    name: String,
    customer: usize,
    parent_title: String,
    visible: bool,
    color: Option<String>,
}

pub fn get_projects(
    config: &Config,
    customers: Option<Vec<usize>>,
) -> Result<Vec<Project>, KimaiError> {
    let response = make_get_request(config, "api/projects", query!(("customers", customers)))?;
    Ok(response.json::<Vec<Project>>()?)
}

pub fn print_projects(
    config_path: Option<String>,
    customers: Option<Vec<usize>>,
) -> Result<(), KimaiError> {
    let config = load_config(config_path)?;
    let projects = get_projects(&config, customers)?;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["ID", "Name", "Customer ID", "Customer Name"]);
    for project in projects {
        table.add_row(row![
            r->project.id,
            project.name,
            r->project.customer,
            project.parent_title
        ]);
    }

    table.printstd();

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    id: usize,
    name: String,
    project: Option<usize>,
    parent_title: Option<String>,
    visible: bool,
    color: Option<String>,
}

pub fn get_activities(
    config: &Config,
    projects: Option<Vec<usize>>,
) -> Result<Vec<Activity>, KimaiError> {
    let response = make_get_request(config, "api/activities", query!(("projects", projects)))?;
    Ok(response.json::<Vec<Activity>>()?)
}

pub fn print_activities(
    config_path: Option<String>,
    projects: Option<Vec<usize>>,
) -> Result<(), KimaiError> {
    let config = load_config(config_path)?;
    let activities = get_activities(&config, projects)?;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["ID", "Name", "Project ID", "Project Name"]);
    for activity in activities {
        let project_str = match activity.project {
            Some(p) => p.to_string(),
            None => "".to_string(),
        };
        table.add_row(row![
            r->activity.id,
            activity.name,
            r->project_str,
            activity.parent_title.unwrap_or_default()
        ]);
    }

    table.printstd();

    Ok(())
}
