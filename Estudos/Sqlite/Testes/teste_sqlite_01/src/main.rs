use sqlite::*;

fn main() {
    let connection = sqlite::open("sharks.db").unwrap();

    let querry_view_table_sharks = "SELECT * FROM sharks";

    connection.iterate(querry_view_table_sharks, |pairs| {
        println!("{:?}", pairs.unwrap())
        true
    }).unwrap();
}