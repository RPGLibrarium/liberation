// mod test {
//     use actix_web::{actix, client};
//     use futures::Future;
//
//     fn connection_check_cert() {
//         actix::run(|| {
//             client::get("http://127.0.0.1:8081/realms/liberation/protocol/openid-connect/certs")
//                 .finish()
//                 .unwrap()
//                 .send()
//                 .map_err(|err| (panic!(err)))
//                 .and_then(|res| {
//                     println!("Cert Connect response: {:?}", res);
//                     Ok(())
//                 })
//         });
//     }
// }

pub struct Token {}
