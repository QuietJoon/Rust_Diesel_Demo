use sn_logger::schema::core_infos::dsl::*;
use sn_logger::*;

fn main() {
    println!("Tx Generator");

    let connection = &mut establish_connection();
    let body: String = "test".to_string();
    let post = write_core_info(connection, 0, 0, true, body);
}
