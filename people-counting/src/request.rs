#![allow(non_snake_case)]
use crate::{Result};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use xml::reader::XmlEvent;
use xml::{EventReader, ParserConfig, EmitterConfig};
use crate::gen_setters;
// use std::str;




#[test]
    fn test_completeXml() {
        let mut reader = ParserConfig::new()
            .ignore_comments(true)
            .create_reader(XMLHTTP_HOST_NOTIFICATION_LIST);
        let mut hm = HashMap::new();
        hm.insert(String::from("id"), NotificationElement::Id("test1".to_string()));
        hm.insert(String::from("ipAddress"), NotificationElement::IpAddress(IpAddr::V4(Ipv4Addr::new(192,168,1,61))));
        hm.insert(String::from("type"), NotificationElement::EventType(vec![EventType::PeopleCounting, EventType::scenechangedetection]));
        hm.insert(String::from("eventMode"), NotificationElement::EventMode(EventMode::list));
        hm.insert(String::from("uploadImagesDataType"), NotificationElement::UploadImagesDataType(UploadImagesDataType::URL));
        let dels = vec![
            //String::from("EventList"),
            //"eventMode".to_string(),
            "password".to_owned(),
            ];
        let res = completeXml(&mut reader, &hm, dels).unwrap();
        println!("{}", std::str::from_utf8(&res).unwrap());
    }

pub const XMLHTTP_HOST_NOTIFICATION_LIST: &[u8] = br#"<HttpHostNotificationList version="2.0" xmlns="http://www.isapi.org/ver20/XMLSchema">
<HttpHostNotification>
  <id><!--req, xs:string, ID--></id>
  <url><!--req, xs:string--></url>
  <protocolType><!--req, xs:string, "HTTP,HTTPS"--></protocolType>
  <parameterFormatType><!--req, xs:string, alarm/event information format, "XML,JSON"--></parameterFormatType>
  <addressingFormatType><!--req, xs:string, "ipaddress,hostname"--></addressingFormatType>
  <hostName><!--dep, xs:string--></hostName>
  <ipAddress><!--dep, xs:string--></ipAddress>
  <ipv6Address><!--dep, xs:string--></ipv6Address>
  <portNo><!--opt, xs:integer--></portNo>
  <userName><!--dep, xs:string--></userName>
  <password><!--dep, xs:string--></password>
  <httpAuthenticationMethod><!--req, xs:string, "MD5digest,none"--></httpAuthenticationMethod>
  <uploadImagesDataType><!--opt, xs:string, "URL", "binary" (default), for cloud storage, only "URL" is supported--></uploadImagesDataType>
  <eventMode><!--opt, xs:string, "all,list"--></eventMode>
  <EventList><!--dep, it is valid only when eventMode is "list"-->
    <Event><!--req-->
      <type><!--req, xs:string--></type>
    </Event>
  </EventList>
  <channels><!--opt, xs:string, "1,2,3,4"--></channels>
</HttpHostNotification>
</HttpHostNotificationList>"#;

pub const XMLHTTP_HOST_NOTIFICATION: &[u8] = br#"<HttpHostNotification version="2.0" xmlns="http://www.isapi.org/ver20/XMLSchema">
  <id><!--required, xs:string, ID--></id>
  <url><!--required, xs:string, the absolute path, e.g., http://<ipAddress>:<portNo>/<uri>--></url>
  <protocolType><!--required, xs:string, "HTTP,HTTPS,EHome"--></protocolType>
  <parameterFormatType><!--required, xs:string, alarm/event information format, "XML,JSON"--></parameterFormatType>
  <addressingFormatType><!--required, xs:string, "ipaddress,hostname"--></addressingFormatType>
  <hostName><!--dependent, xs:string--></hostName>
  <ipAddress><!--dependent, xs:string--></ipAddress>
  <ipv6Address><!--dependent, xs:string--></ipv6Address>
  <portNo><!--optional, xs:integer--></portNo>
  <userName><!--dependent, xs:string--></userName>
  <password><!--dependent, xs:string--></password>
  <httpAuthenticationMethod><!--required, xs:string, "MD5digest,none"--></httpAuthenticationMethod>
  <ANPR><!--optional-->
    <detectionUpLoadPicturesType>
      <!--optional, xs:string, types of alarm picture to be uploaded: "all, licensePlatePicture, detectionPicture"-->
    </detectionUpLoadPicturesType>
  </ANPR>
  <eventType optional="AID,TFS,TPS"><!--required, xs:string--></eventType>
  <uploadImagesDataType>
    <!--optional, xs:string, "URL", "binary" (default), for cloud storage, only "URL" is supported-->
  </uploadImagesDataType>
  <eventMode><!--optional, xs:string, "all,list"--></eventMode>
  <EventList><!--dependent, it is valid only when eventMode is "list"-->
    <Event><!--required-->
      <type><!--required, xs:string--></type>
    </Event>
  </EventList>
  <channels><!--optional, xs:string, "1,2,3,4"--></channels>
  <SubscribeEvent/><!--optional, event subscription parameters, see details in the message of XML_SubscribeEvent-->
</HttpHostNotification>"#;

trait HashElemts<T> {
    fn insert(&mut self, el: T) -> Option<T>;
}


#[derive(Debug)]
pub enum ProtocolType {
    HTTP,
    HTTPS,
    EHome,
}

#[derive(Debug)]
pub enum ParameterFormatType {
    XML,
    JSON,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum AddressingFormatType {
    ipaddress,
    hostname,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum HttpAuthenticationMethod {
    MD5digest,
    none,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum EventType {
    PeopleCounting,
    scenechangedetection,
}

impl std::string::ToString for EventType {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum EventMode {
    list,
    all,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum UploadImagesDataType {
    URL,
    binary,
}

//#[derive(Clone)]
pub enum NotificationElement {
    Id(String),
    IpAddress(IpAddr),
    Port(u64),
    Hostname(String),
    Url(String),
    Protocol(ProtocolType),
    ParameterFormat(ParameterFormatType),
    AddressingFormat(AddressingFormatType),
    HttpAuthentication(HttpAuthenticationMethod),
    EventType(Vec<EventType>),
    //EventType(EventType),
    EventMode(EventMode),
    Channels(String),
    UploadImagesDataType(UploadImagesDataType),
}
/* */
pub struct HttpHostConfig {
    id: String,
    ipAddress: IpAddr,
    portNo: u16,
    hostName: String,
    url: String,
    protocolType: ProtocolType,
    parameterFormatType: ParameterFormatType,
    addressingFormatType: AddressingFormatType,
    httpAuthenticationMethod: HttpAuthenticationMethod,
    channels: String,
    uploadImagesDataType: UploadImagesDataType,
    eventMode: EventMode,
    eventType: Vec<EventType>,
}

impl HttpHostConfig {
    pub fn new() -> HttpHostConfig {
        HttpHostConfig {
            id: "1".to_owned(),
            ipAddress: IpAddr::V4(Ipv4Addr::new(127,0,0,1)),
            portNo: 8080,
            hostName: "localhost".to_owned(),
            url: "http://127.0.0.1:8080/".to_owned(),
            protocolType: ProtocolType::HTTP,
            parameterFormatType: ParameterFormatType::XML,
            addressingFormatType: AddressingFormatType::ipaddress,
            httpAuthenticationMethod: HttpAuthenticationMethod::none,
            channels: "1".to_owned(),
            uploadImagesDataType: UploadImagesDataType::URL,
            eventMode: EventMode::list,
            eventType: vec![EventType::PeopleCounting],
        }
    }
}

/*
gen_setters! { HttpHostConfig,
    id: val String,
    ipAddress: val IpAddr,
    portNo: val u16,
    hostName: val String,
    url: val String,
    protocolType: val ProtocolType,
    parameterFormatType: val ParameterFormatType,
    addressingFormatType: val AddressingFormatType,
    httpAuthenticationMethod: val HttpAuthenticationMethod,
    channels: val String,
    uploadImagesDataType: val UploadImagesDataType,
    eventMode: val EventMode,
    eventType: val EventType
}
*/

impl NotificationElement {
    fn getName(&self) -> String {
        match self {
            NotificationElement::Id(_) => String::from("id"),
            NotificationElement::IpAddress(_) => String::from("ipAddress"),
            NotificationElement::Port(_) => String::from("portNo"),
            NotificationElement::Hostname(_) => String::from("hostName"),
            NotificationElement::Protocol(_) => String::from("protocolType"),
            NotificationElement::Url(_) => String::from("url"),
            NotificationElement::ParameterFormat(_) => String::from("parameterFormatType"),
            NotificationElement::HttpAuthentication(_) => String::from("httpAuthenticationMethod"),
            NotificationElement::AddressingFormat(_) => String::from("addressingFormatType"),
            NotificationElement::EventType(_) => String::from("type"),
            NotificationElement::Channels(_) => String::from("channels"),
            NotificationElement::UploadImagesDataType(_) => String::from("uploadImagesDataType"),
            NotificationElement::EventMode(_) => String::from("eventMode"),

        }
    }
    fn getValue(&self) -> String {
        match self {
            NotificationElement::Id(v) => format!("{}",v),
            NotificationElement::IpAddress(v) => format!("{}",v),
            NotificationElement::Port(v) => format!("{}",v),
            NotificationElement::Hostname(v) => format!("{}",v),
            NotificationElement::Protocol(v) => format!("{:?}",v),
            NotificationElement::Url(v) => format!("{}",v),
            NotificationElement::ParameterFormat(v) => format!("{:?}",v),
            NotificationElement::HttpAuthentication(v) => format!("{:?}",v),
            NotificationElement::AddressingFormat(v) => format!("{:?}",v),
            NotificationElement::EventType(v) => {
                let mut s = format!("{}",v.iter().fold(String::new(), |acc, x| acc + &x.to_string() + "," ));
                s.pop();
                s
            },
            NotificationElement::Channels(v) => format!("{}",v),
            NotificationElement::UploadImagesDataType(v) => format!("{:?}",v),
            NotificationElement::EventMode(v) => format!("{:?}",v),
        }
    }
}

//#[derive(Clone)]
pub struct HttpHostNotification {
    elements: HashMap<String, NotificationElement>,
}

impl HttpHostNotification {
    // pub fn insert(&mut self, el: NotificationElement) {
    //     self.elements.insert(el.getName(), el);
    // }

    pub fn new(els: Vec<NotificationElement>) -> HttpHostNotification {
        let h_els: HashMap<_,_> = els.into_iter().map(|v| (v.getName(), v)).collect();
        HttpHostNotification {
            elements: h_els,
        }
    }

    pub fn parse_list(&self) -> Result<Vec<u8>> {
        let a = XMLHTTP_HOST_NOTIFICATION_LIST;
        let mut reader = ParserConfig::new()
            .trim_whitespace(true)
            .ignore_comments(true)
            .create_reader(a);
        let dels: Vec<String> = Vec::new();
        /*
        let dels = vec![
            //"EventList".to_owned(),
            //"eventoMode".to_owned(),
            "ANPR".to_owned(),
            //"uploadImagesDataType".to_owned(),
            "SubscribeEvent".to_owned(),     
            ];
        */
        completeXml(&mut reader, &self.elements, dels)
        // let result = String::from_utf8(xml_result)
        // match result {
        //     Ok(v) => Ok(v),
        //     Err(err) => Err(Error::FromUtf8Error(err).into())
        // }
    }

    pub fn parse(&self) -> Result<Vec<u8>> {
        let a = XMLHTTP_HOST_NOTIFICATION;
        let mut reader = ParserConfig::new().trim_whitespace(true)
            .ignore_comments(true)
            .create_reader(a);
        let dels = vec![
            //"EventList".to_owned(),
            //"eventoMode".to_owned(),
            "ANPR".to_owned(),
            //"uploadImagesDataType".to_owned(),
            "SubscribeEvent".to_owned(),     
            ];
        completeXml(&mut reader, &self.elements, dels)
        // let result = String::from_utf8(xml_result)
        // match result {
        //     Ok(v) => Ok(v),
        //     Err(err) => Err(Error::FromUtf8Error(err).into())
        // }
    }
}

fn completeXml(reader: &mut EventReader<&[u8]>, 
    els: &HashMap<String, NotificationElement>,
    dels: Vec<String>) -> Result<Vec<u8>> {

    let h_dels: HashMap<_,_> = dels.iter().map(|v| (v, v)).collect();
    
    let mut target: Vec<u8> = Vec::new();
    let mut writer = EmitterConfig::new()
        .normalize_empty_elements(false)
        .perform_indent(true)
        .create_writer(&mut target);
    loop {
        let event = reader.next()?;
        // println!("debug: {:?}", event);
        match event {
            XmlEvent::StartElement{ref name, ..} => {
                if let Some(v) = h_dels.get(&name.local_name) {
                    loop {
                        let e = reader.next()?;
                        match e {
                            XmlEvent::EndElement{ref name} => {
                                if &name.local_name == v.as_str() {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    continue;                    
                };
            }
            XmlEvent::Whitespace(_) => {
                continue;
            }
            _ => {}
        }

        // println!("debug: {:?}", event);
        
        if let Some(er) = event.as_writer_event() {
            writer.write(er)?;
        }

        match event {
            XmlEvent::StartElement{ref name, ..} => {
                if let Some(res) = els.get(&name.local_name) {
                    writer.write(res.getValue().as_str())?;
                };
            }
            XmlEvent::EndDocument => {
                break;
            }
            _ => {}
        }
    }
    Ok(target)
}
