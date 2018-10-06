use reqwest::Client;

pub enum AuthError {
    InvalidCode,
    InvalidUser,
    UnauthorizedUser,
    Unknown,
}

#[derive(Serialize)]
struct AuthPayload {
    client_id: String,
    client_secret: String,
    code: String,
}

#[derive(Deserialize)]
struct Credentials {
    client_id: String,
    client_secret: String,
}

pub fn verify(auth_code: String) -> Result<(), AuthError> {
    let client = Client::new();

    

    
    let payload = AuthPayload {
        client_id: "".into(),
        client_secret: "".into(),
        code: auth_code.into(),
    };

    let res = client
        .post("https://github.com/login/oauth/access_token")
        .json(&payload)
        .send();

    println!("{:#?}", res);

    Ok(())
}
