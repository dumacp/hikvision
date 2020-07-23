use reqwest::blocking::{Client, Response};
use crate::basicauth::BasicAuth;
use crate::request::{NotificationElement, HttpHostNotification};

pub const BASE_URI: &str = "http://192.168.122.1:8080";

pub struct PeopleCounting<'a> {
    basic_auth: &'a mut BasicAuth,
    channel_id: u32,
}

impl<'a> PeopleCounting<'a> {
    pub fn new(basic_auth: &'a mut BasicAuth, channel_id: u32) -> PeopleCounting {
        PeopleCounting {
            basic_auth,
            channel_id,
        }
    }

    pub fn get_capabilities(&mut self) -> crate::Result<Response> {
        let dir = "/ISAPI/System/Video/capabilities";
        self.get(dir)
    }

    pub fn get_people_capabilities(&mut self, id: u32) -> crate::Result<Response> {
        let dir = format!("/ISAPI/System/Video/inputs/channels/{}/counting/capabilities", id);
        self.get(dir.as_str())
    }

    pub fn get_configuration_counting(&mut self, id: u32) -> crate::Result<Response> {
        let dir = format!("/ISAPI/System/Video/inputs/channels/{}/counting", id);
        self.get(dir.as_str())
    }

    pub fn get_http_host(&mut self) ->  crate::Result<Response> {
        let dir = "/ISAPI/Event/notification/httpHosts";
        self.get(dir)
    }

    pub fn put_http_host_list(&mut self, els: Vec<NotificationElement>) ->  crate::Result<Response> {
        let dir = "/ISAPI/Event/notification/httpHosts";
        let body = HttpHostNotification::new(els).parse_list()?;
        println!("debug: {}", String::from_utf8(body.to_owned())?);
        self.put(dir, body)
    }

    pub fn post_http_host(&mut self, els: Vec<NotificationElement>) ->  crate::Result<Response> {
        let dir = "/ISAPI/Event/notification/httpHosts";
        let http_ntt = HttpHostNotification::new(els);
        self.post(dir, http_ntt.parse()?)
    }

    fn get_build_header(&mut self, dir: &str) ->  crate::Result<reqwest::header::HeaderMap> {
        let header_auth = self.basic_auth.get_auth_header(dir)?;

        // println!("debug, header_auth: {}", header_auth);

        /***/
        let h_value = reqwest::header::HeaderValue::from_str(&header_auth.to_string())?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, h_value);
        Ok(headers)
    }

    fn get(&mut self, dir: &str) ->  crate::Result<Response> {
        
        // let dir = "/ISAPI/System/Video/capabilities";
        let header_auth = self.get_build_header(dir)?;
        let mut uri = String::from(BASE_URI);
        uri.push_str(dir);
        let client = Client::new();
        let res = client
            .get(&uri)
            .basic_auth(&self.basic_auth.username(), Some(&self.basic_auth.passwird()))
            .headers(header_auth)
            .send()?;
        // res.headers().iter().by_ref().for_each(|(k, v)| {
        //     println!("k: {:?}; v: {:?}", k, v);
        // });
        // match res.text() {
        //     Ok(x) => Ok(x),
        //     Err(e) => Err(Error::new(format!("{}", e).as_str()).into()),
        // }
        Ok(res)
        /**/
    }

    fn put_build_header(&mut self, dir: &str, body: Vec<u8>) ->  crate::Result<reqwest::header::HeaderMap> {
        let header_auth = self.basic_auth.put_auth_header(dir, body)?;

        // println!("debug, header_auth: {}", header_auth);

        /***/
        let h_value = reqwest::header::HeaderValue::from_str(&header_auth.to_string())?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, h_value);
        Ok(headers)
    }

    fn put(&mut self, dir: &str, body: Vec<u8>) ->  crate::Result<Response> {
        
        // let dir = "/ISAPI/System/Video/capabilities";
        let header_auth = self.put_build_header(dir, body.to_owned())?;
        let mut uri = String::from(BASE_URI);
        uri.push_str(dir);

        
        let client = Client::new();
        let res = client
            .put(&uri)
            .basic_auth(&self.basic_auth.username(), Some(&self.basic_auth.passwird()))
            .headers(header_auth)
            .body(body)
            .send()?;
        Ok(res)
    }

    fn post_build_header(&mut self, dir: &str, body: Vec<u8>) ->  crate::Result<reqwest::header::HeaderMap> {
        let header_auth = self.basic_auth.post_auth_header(dir, body)?;

        // println!("debug, header_auth: {}", header_auth);

        /***/
        let h_value = reqwest::header::HeaderValue::from_str(&header_auth.to_string())?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, h_value);
        Ok(headers)
    }

    fn post(&mut self, dir: &str, body: Vec<u8>) ->  crate::Result<Response> {
        
        // let dir = "/ISAPI/System/Video/capabilities";
        let header_auth = self.post_build_header(dir, body.to_owned())?;
        let mut uri = String::from(BASE_URI);
        uri.push_str(dir);

        
        let client = Client::new();
        let res = client
            .post(&uri)
            .basic_auth(&self.basic_auth.username(), Some(&self.basic_auth.passwird()))
            .headers(header_auth)
            .body(body)
            .send()?;
        Ok(res)
    }
}
