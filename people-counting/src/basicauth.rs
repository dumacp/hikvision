use digest_auth::{self, AuthContext, WwwAuthenticateHeader, HttpMethod};
use reqwest::blocking::{Client};

pub const INITIAL_URI: &str = "http://192.168.122.1:8080/ISAPI/";
// const WWW_AUTH_DEFAULT: &str = "Digest qop=\"auth\", realm=\"IP Camera(D8291)\", nonce=\"4d6a59334d446b7a4e6d59364d574579595751335a6a4d3d\", stale=\"FALSE\"";

pub struct BasicAuth {
    username: String,
    passwird: String,
    // www_authenticate: String,
    prompt: WwwAuthenticateHeader,
}

impl BasicAuth {
    pub fn new(username: String, passwird: String) -> crate::Result<BasicAuth> {

        let auth = get_initial_www_auth(INITIAL_URI, &username, &passwird)?;
        let auth_str = auth.to_str()?;

        // println!("debug, www-auth: {}", auth_str);

        let prompt = digest_auth::parse(auth_str)?;
        let basic = BasicAuth {
            username,
            passwird,
            // www_authenticate: auth_str.to_string(),
            prompt: prompt,
        };
        Ok(basic)
    }

    pub fn username(&self) -> String {
        self.username.to_string()
    }

    pub fn passwird(&self) -> String {
        self.passwird.to_string()
    }

    pub fn get_auth_header(&mut self, dir: &str) -> crate::Result<String> {
        let mut context = AuthContext::new(&self.username, &self.passwird, dir);
        context.set_custom_cnonce("f2/wE4q74E6zIJEtWaHKaf5wv/H5QzzpXusqGemxURZJ");

        let answer = self.prompt.respond(&context)?;
        Ok(answer.to_string())
    }

    pub fn put_auth_header(&mut self, dir: &str, body: Vec<u8>) -> crate::Result<String> {
        let mut context = AuthContext::new_with_method(
            &self.username, 
            &self.passwird,
            dir,
            Some(body),
            HttpMethod::OTHER("PUT"),
            
        );
        context.set_custom_cnonce("f2/wE4q74E6zIJEtWaHKaf5wv/H5QzzpXusqGemxURZJ");

        let answer = self.prompt.respond(&context)?;
        Ok(answer.to_string())
    }
    pub fn post_auth_header(&mut self, dir: &str, body: Vec<u8>) -> crate::Result<String> {
        let mut context = AuthContext::new_post(
            &self.username, 
            &self.passwird,
            dir,
            Some(body),
        );
        context.set_custom_cnonce("f2/wE4q74E6zIJEtWaHKaf5wv/H5QzzpXusqGemxURZJ");

        let answer = self.prompt.respond(&context)?;
        Ok(answer.to_string())
    }
}

pub fn get_initial_www_auth(
    uri: &str,
    username: &str,
    passwird: &str,
) -> crate::Result<reqwest::header::HeaderValue> {
    let client = Client::new();
    let req = client
        .get(uri)
        .basic_auth(username, Some(passwird))
        .send()?;

    let auth = req
        .headers()
        .iter()
        .find(|(k, _)| &reqwest::header::WWW_AUTHENTICATE == k);
    let res = match auth {
        Some((_, v)) => Ok(v.clone()),
        None => Err(crate::Error::new("www-authenticate not found").into()),
    };
    res
}
