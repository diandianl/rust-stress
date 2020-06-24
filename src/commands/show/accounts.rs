use clap::{App, ArgMatches, SubCommand};
use serde::{Deserialize, Serialize};

use crate::context::Context;
use crate::rpc;

pub fn cli<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("accounts")
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ResultAccounts {
    BlockHeight: u64,
    Accounts: Vec<Account>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    Address: String,
    Balance: u64,
    EVMCode: String
}

pub fn exec(ctx: &Context, _args: &ArgMatches) {
    let resp = ctx.http.get("http://localhost:26658/accounts").send();
    let resp: reqwest::blocking::Response = resp.unwrap();
    let ret = resp.json::<rpc::JsonRPC<ResultAccounts>>().unwrap();
    // let resp = resp.json::<HashMap<String, String>>().unwrap();
    println!("accounts exec {:?}", ret.result)
}