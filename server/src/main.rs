use rocket::{http::Status, response::content::Html, *};
use std::{net::*, sync::RwLock};
use chrono::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(Config {
            address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: 6969,
            ..Config::default()
        })
        .manage(RwLock::new(Vec::<TelemetryRecord>::new()))
        .mount("/", routes![index])
        .mount("/telemetry", routes![telemetry])
}

struct TelemetryRecord {
    pub date_time: DateTime<Local>,
    pub address: IpAddr,
    pub macro_name: String,
    pub crate_name: String,
    pub crate_version: String,
    pub os_name: String,
    pub ts_hash: String,
}

impl TelemetryRecord {
    pub fn format_html(&self) -> String {
        format!(
            r##"
                <tr>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                </tr>
            "##,
            self.date_time,
            self.address,
            self.macro_name,
            self.crate_name,
            self.crate_version,
            self.os_name,
            self.ts_hash
        )
    }
}

#[get("/")]
fn index(records_lock: &State<RwLock<Vec<TelemetryRecord>>>) -> Result<Html<String>, Status> {
    if let Ok(records) = records_lock.read() {
        const TEMPLATE: &str = include_str!("index.html");
        Ok(Html(
            TEMPLATE.replace(
                "${data_goes_here}",
                &records
                    .iter()
                    .map(TelemetryRecord::format_html)
                    .collect::<String>(),
            ),
        ))
    } else {
        Err(Status::InternalServerError)
    }
}

#[get("/<macro_name>/<crate_name>/<crate_version>/<os_name>/<ts_hash>")]
fn telemetry(
    macro_name: String,
    crate_name: String,
    crate_version: String,
    os_name: String,
    ts_hash: String,
    address: SocketAddr,
    records_lock: &State<RwLock<Vec<TelemetryRecord>>>,
) -> Status {
    if let Ok(mut records) = records_lock.write() {
        records.push(TelemetryRecord {
            date_time: Local::now(),
            address: address.ip(),
            macro_name,
            crate_name,
            crate_version,
            os_name,
            ts_hash,
        });
        Status::Ok
    } else {
        Status::InternalServerError
    }
}
