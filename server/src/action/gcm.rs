extern crate curl;
extern crate jsonway;
extern crate serde_json;

use action::iface::Action;

use std::io::Result;

use self::curl::http;
use self::serde_json::ser;

const GCM_KEY: &'static str = include_str!("../../gcm_key.txt");
const GCM_ENDPOINT: &'static str = "http://gcm-http.googleapis.com/gcm/send";
const GCM_TARGET: &'static str = "fsc3M87LMQo:APA91bH15EC140SxXruImHDFrr-7RDJQyvHow8_Zlxq7OiFZoE9tYNxtfX2hXCrhCsIp8KoJhz9HwWojSo3aGkfn7lUaRXuWf4Y9gcKM0jv-HZ7B4vUEsasZrXWmBoZ3GXE_z2fEnOm1";

pub struct GcmAction {
}

impl GcmAction {
    pub fn new() -> GcmAction {
        GcmAction {
        }
    }
}

impl Action for GcmAction {
    fn trigger(&mut self) -> Result<()> {
        let msg = format!("{}", ser::to_string(&jsonway::object(|json| {
            json.object("notification", |json| {
                json.set("title", "Klocka");
                json.set("text", "Open the door!");
            });

            json.set("to", GCM_TARGET);
        }).unwrap()).unwrap());
        println!("{}: {}", msg.len(), msg);
        let mut client = http::handle()
            .verbose();
        let request = client
            .post(GCM_ENDPOINT, &msg)
            .header("Authorization", format!("key={}", GCM_KEY).as_ref())
            .header("Content-Type", "application/json");
            //.header("Content-Length", format!("{}", msg.len()).as_ref());
        let result = request
            .exec()
            .unwrap();
        println!("{:?}", result);
        Ok(())
    }
}
