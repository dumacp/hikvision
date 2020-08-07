use people_counting::{BasicAuth, PeopleCounting, HttpHostConfig};
use std::process;
use std::net::{IpAddr, Ipv4Addr};


fn main() {

    let mut auth = BasicAuth::new("admin".to_string(), "Alpha.2.Beta".to_string())
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    let mut people = PeopleCounting::new(&mut auth, 1);

    /**/
    let res = people.get_capabilities().unwrap_or_else(|err| {
        eprintln!("errror: {}", err);
        process::exit(1);
    });
    let text = res.text().unwrap_or_else(|err| {
        eprintln!("errror: {}", err);
        process::exit(1);
    });
    println!("get_capabilities:\n{}", text);
    std::thread::sleep(std::time::Duration::from_secs(3));

    let res = people.get_people_capabilities(1).unwrap_or_else(|err| {
        eprintln!("errror: {}", err);
        process::exit(1);
    });

    let text = res.text().unwrap_or_else(|err| {
        eprintln!("errror: {}", err);
        process::exit(1);
    });
    println!("get_people_capabilities:\n{}", text);

    std::thread::sleep(std::time::Duration::from_secs(3));

    let res = people.get_configuration_counting(1).unwrap_or_else(|err| {
        eprintln!("errror: {}", err);
        process::exit(1);
    });
    let text = res.text().unwrap_or_else(|err| {
        eprintln!("errror: {}", err);
        process::exit(1);
    });
    println!("get_configuration_counting:\n{}", text);
    std::thread::sleep(std::time::Duration::from_secs(3));

    let res = people.get_http_host().unwrap_or_else(|err| {
        eprintln!("errror: {}", err);
        process::exit(1);
    });
    let text = res.text().unwrap_or_else(|err| {
        eprintln!("errror: {}", err);
        process::exit(1);
    });
    println!("get_http_host:\n{}", text);
    /**/

    // let els = vec![
    //         NotificationElement::Id("1".to_owned()),
    //         NotificationElement::Channels("1".to_owned()),
    //         NotificationElement::IpAddress(IpAddr::V4(Ipv4Addr::new(192,168,1,61))),
    //         NotificationElement::Port(8088),
    //         NotificationElement::Url("http://192.168.1.61:8088/".to_owned()),
    //         NotificationElement::EventMode(EventMode::all),
    //         NotificationElement::EventType(vec![EventType::PeopleCounting, EventType::scenechangedetection]),
    //         //NotificationElement::EventType("PeopleCounting,shelteralarm,scenechangedetection".to_owned()),
    //         //NotificationElement::EventType(EventType::PeopleCounting),
    //         NotificationElement::HttpAuthentication(HttpAuthenticationMethod::none),
    //         NotificationElement::ParameterFormat(ParameterFormatType::JSON),
    //         NotificationElement::Protocol(ProtocolType::HTTP),
    //         NotificationElement::AddressingFormat(AddressingFormatType::ipaddress),
    //         NotificationElement::UploadImagesDataType(UploadImagesDataType::URL),
    // ];

    let http_config = HttpHostConfig::new()
        .ipAddress(IpAddr::V4(Ipv4Addr::new(192,168,188,23)))
        .url("/".to_owned())
        .portNo(8088)
        .hostName("192.168.188.23".to_owned());

    let res = people.http_host_list(http_config).unwrap_or_else(|err| {
        eprintln!("errror: {}", err);
        process::exit(1);
    });
    let text = res.text().unwrap_or_else(|err| {
        eprintln!("errror: {}", err);
        process::exit(1);
    });
    println!("put_http_host:\n{}", text);

    let res = people.get_http_host().unwrap_or_else(|err| {
        eprintln!("errror: {}", err);
        process::exit(1);
    });
    let text = res.text().unwrap_or_else(|err| {
        eprintln!("errror: {}", err);
        process::exit(1);
    });
    println!("get_http_host:\n{}", text);







    

    

}
