use std::process::Command;
use std::{os::unix::process::CommandExt, str::FromStr};

use crate::config::read_config;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::Value;

pub struct ServerConnectionDetails {
    pub ip: String,
    pub port: String,
    pub username: String,
    pub password: String,
}

pub async fn handle_ssh(server_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let details = get_server_connection_details(&server_id).await?;

    let ssh_cmd = format!(
        "sshpass -p '{}' ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null {}@{} -p {}",
        details.password, details.username, details.ip, details.port
    );

    Err(Command::new("zsh").arg("-c").arg(&ssh_cmd).exec().into())
}

pub async fn get_server_connection_details(
    server_id: &str,
) -> Result<ServerConnectionDetails, Box<dyn std::error::Error>> {
    let config = read_config()?;

    if let Some(ssh) = &config.ssh {
        if let Some(server) = ssh.servers.get(server_id) {
            return Ok(ServerConnectionDetails {
                ip: server.ip.clone().unwrap_or_default(),
                port: server.port.clone().unwrap_or(String::from("22")),
                username: server.username.clone().unwrap_or_default(),
                password: server.password.clone().unwrap_or_default(),
            });
        }

        if let Some(derive) = &ssh.derive {
            if let Some(appwrite) = &derive.appwrite {
                if let Some(appwrite_server) = appwrite.servers.get(server_id) {
                    let ip = if let Some(get_url) = &appwrite_server.get {
                        let mut headers = HeaderMap::new();

                        if let Some(project) = &appwrite_server.x_appwrite_project {
                            headers.insert(
                                HeaderName::from_str("X-Appwrite-Project").unwrap(),
                                HeaderValue::from_str(project).unwrap(),
                            );
                        }

                        if let Some(key) = &appwrite_server.x_appwrite_key {
                            headers.insert(
                                HeaderName::from_str("X-Appwrite-Key").unwrap(),
                                HeaderValue::from_str(key).unwrap(),
                            );
                        }

                        let client = reqwest::Client::new();
                        let response: Value = client
                            .get(get_url)
                            .headers(headers)
                            .send()
                            .await?
                            .json()
                            .await?;

                        let ip = response["ip_address"]
                            .as_str()
                            .ok_or_else(|| "Missing 'ip_address' field in response".to_string())?
                            .to_string();

                        ip
                    } else {
                        return Err(format!(
                            "No 'get' URL specified for Appwrite server {}",
                            server_id
                        )
                        .into());
                    };

                    return Ok(ServerConnectionDetails {
                        ip,
                        port: appwrite_server
                            .port
                            .clone()
                            .unwrap_or_else(|| String::from("22")),
                        username: appwrite_server.username.clone().unwrap_or_default(),
                        password: appwrite_server.password.clone().unwrap_or_default(),
                    });
                }
            }
        }
    }

    Err(format!("Server '{}' not found in configuration", server_id).into())
}
