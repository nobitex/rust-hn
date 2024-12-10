use std::sync::Arc;

use anyhow::Result;
use structopt::StructOpt;
use tokio::sync::Mutex;

use crate::{
    db::{PostgresDB, PostgresOpt, DB},
    server::{jsonrpc_server, rest_server, Context, JsonrpcOpt, RestOpt},
};

#[derive(Debug, StructOpt)]
struct StartOpt {
    #[structopt(flatten)]
    pub postgres_opt: PostgresOpt,
    #[structopt(flatten)]
    pub jsonrpc_opt: JsonrpcOpt,
    #[structopt(flatten)]
    pub rest_opt: RestOpt,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "satoshi", about = "Satoshi Family")]
enum Opt {
    Start(StartOpt),
    Debug,
}

pub async fn cli() -> Result<()> {
    dotenvy::dotenv().ok();
    let opt = Opt::from_args();
    match opt {
        Opt::Start(opt) => {
            println!("Start! {:?}", opt);
            let pg_db = PostgresDB::new(opt.postgres_opt).await?;
            let users = pg_db.get_users().await?;
            println!("Users: {:?}", users);
            // pg_db.add_user("Alice").await?;
            // let users = pg_db.get_users().await?;

            let ctx = Arc::new(Mutex::new(Context { db: pg_db }));
            let jsonrpc_server_fut = jsonrpc_server(ctx.clone(), opt.jsonrpc_opt);
            let rest_server_fut = rest_server(ctx.clone(), opt.rest_opt);
            tokio::try_join!(jsonrpc_server_fut, rest_server_fut)?;
        }
        Opt::Debug => {
            println!("Debug!");
        }
    };

    Ok(())
}
