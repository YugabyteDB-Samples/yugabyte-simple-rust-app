# Simple Rust Application for YugabyteDB

This application connects to your YugabyteDB instance via the 
[Rust](https://github.com/sfackler/rust-postgres) driver for PostgreSQL and performs basic SQL operations. The instructions below are provided for [Yugabyte Cloud](https://cloud.yugabyte.com/) deployments. 
If you use a different type of deployment, then update the `src/sample-app.rs` file with proper connection parameters.

## Prerequisite

* [Rust](https://www.rust-lang.org/tools/install) development environment. The sample application was created for Rust 1.58 but should work for earlier and later versions.
* Command line tool or your favourite IDE, such as Visual Studio Code.

## Start Yugabyte Cloud Cluster

* [Start YugabyteDB Cloud](https://docs.yugabyte.com/latest/yugabyte-cloud/cloud-quickstart/qs-add/) instance. You can use
the free tier at no cost.
* Add an IP address of your machine/laptop to the [IP allow list](https://docs.yugabyte.com/latest/yugabyte-cloud/cloud-secure-clusters/add-connections/#manage-ip-allow-lists)

## Clone Application Repository

Clone the repository and change dirs into it:

```bash
git clone https://github.com/yugabyte/yugabyte-simple-rust-app && cd yugabyte-simple-rust-app
```

## Provide Yugabyte Cloud Connection Parameters

Update the following connection parameters in the `src/sample-app.rs` file:
* `HOST` - the hostname of your Yugabyte Cloud instance.
* `USER` - the username for your instance.
* `PASSWORD` - the database password.
* `SSL_MODE`  - make sure it's set to `SslMode::Require`.
* `SSL_ROOT_CERT` - a full path to your CA root cert (for example, `/Users/dmagda/certificates/root.crt`). 

Note, you can easily find all the settings on the Yugabyte Cloud dashboard:

![image](resources/cloud_app_settings.png)

## Run the Application
 
Build and run the application:
```bash
cargo run
```

The Rust PostgreSQL driver will be installed automatically during the first execution of the command. The driver is listed in the dependencies list of the `Cargo.toml` file.

Upon successful execution, you will see output similar to the following:

```bash
>>>> Connecting to YugabyteDB!
>>>> Successfully connected to YugabyteDB!
>>>> Successfully created table DemoAccount.
>>>> Selecting accounts:
name = Jessica, age = 28, country = USA, balance = 10000
name = John, age = 28, country = Canada, balance = 9000
>>>> Transferred 800 between accounts
>>>> Selecting accounts:
name = Jessica, age = 28, country = USA, balance = 9200
name = John, age = 28, country = Canada, balance = 9800
```

## Explore Application Logic

Congrats! You've successfully executed a simple Rust app that works with Yugabyte Cloud.

Now, explore the source code of `src/sample-app.rs`:
1. `connect` function - establishes a connection with your cloud instance via the Rust PostgreSQL driver.
3. `create_database` function - creates a table and populates it with sample data.
4. `select_accounts` function - queries the data with SQL `SELECT` statements.
5. `transfer_money_between_accounts` function - updates records consistently with distributed transactions.

## Questions or Issues?

Having issues running this application or want to learn more from Yugabyte experts?

Join [our Slack channel](https://communityinviter.com/apps/yugabyte-db/register),
or raise a question on StackOverflow and tag the question with `yugabytedb`!