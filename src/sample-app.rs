use postgres::{Config, Client, error::SqlState, config::SslMode};
use postgres::error::Error as DBError;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use std::error::Error;


const HOST: &str = "";
const PORT: u16 = 5433;
const DB_NAME: &str = "yugabyte";
const USER: &str = "";
const PASSWORD: &str = "";
const SSL_MODE: SslMode = SslMode::Require;
const SSL_ROOT_CERT: &str = "";


fn main() {
    
    let mut client = connect().unwrap();

    create_database(&mut client).unwrap();
    select_accounts(&mut client).unwrap();
    transfer_money_between_accounts(&mut client, 800).unwrap();
    select_accounts(&mut client).unwrap();
}

fn connect() -> Result<Client, Box<dyn Error>> {
    println!(">>>> Connecting to YugabyteDB!");

    let mut cfg = Config::new();

    cfg.host(HOST).port(PORT).dbname(DB_NAME).
        user(USER).password(PASSWORD).ssl_mode(SSL_MODE);

    let mut builder = SslConnector::builder(SslMethod::tls())?;
    builder.set_ca_file(SSL_ROOT_CERT)?;
    let connector = MakeTlsConnector::new(builder.build());
    
    let client = cfg.connect(connector)?;
        
    println!(">>>> Successfully connected to YugabyteDB!");

    Ok(client)
}

fn create_database(client: &mut Client) -> Result<(), DBError> {
    client.execute("DROP TABLE IF EXISTS DemoAccount", &[])?;

    client.execute("CREATE TABLE DemoAccount (
                    id int PRIMARY KEY,
                    name varchar,
                    age int,
                    country varchar,
                    balance int)", &[])?;

    client.execute("INSERT INTO DemoAccount VALUES 
                    (1, 'Jessica', 28, 'USA', 10000),
                    (2, 'John', 28, 'Canada', 9000)", &[])?;

    println!(">>>> Successfully created table DemoAccount.");

    Ok(())
}

fn select_accounts(client: &mut Client) -> Result<(), DBError> {
    println!(">>>> Selecting accounts:");

    for row in client.query("SELECT name, age, country, balance FROM DemoAccount", &[])? {
        let name: &str = row.get("name");
        let age: i32 = row.get("age");
        let country: &str = row.get("country");
        let balance: i32 = row.get("balance");

        println!("name = {}, age = {}, country = {}, balance = {}", 
            name, age, country, balance);
    }

    Ok(())
}

fn transfer_money_between_accounts(client: &mut Client, amount: i32) -> Result<(), DBError> {
    let mut txn = client.transaction()?;

    let exec_txn = || -> Result<(), DBError> {
        txn.execute("UPDATE DemoAccount SET balance = balance - $1 WHERE name = \'Jessica\'", &[&amount])?;
        txn.execute("UPDATE DemoAccount SET balance = balance + $1 WHERE name = \'John\'", &[&amount])?;
        txn.commit()?;

        Ok(())
    };

    if let Err(err) = exec_txn() {
        if err.code() == Some(&SqlState::T_R_SERIALIZATION_FAILURE) {
            println!("The operation is aborted due to a concurrent transaction that is modifying the same set of rows.
                Consider adding retry logic for production-grade applications.");
        }

        Err(err)
    } else {
        println!(">>>> Transferred {} between accounts", amount);
        Ok(())
    }
}