use std::sync::Arc;

use anyhow::Result;
use minijinja::Environment;
use structopt::StructOpt;
use tokio::sync::Mutex;

use crate::{
    db::{PostgresDB, PostgresOpt},
    jwt::{Jwt, JwtOpts},
    server::{rest_server, Context, RestOpt},
};

#[derive(Debug, StructOpt)]
struct StartOpt {
    #[structopt(flatten)]
    pub postgres_opt: PostgresOpt,
    #[structopt(flatten)]
    pub rest_opt: RestOpt,
    #[structopt(flatten)]
    pub jwt_opt: JwtOpts,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "satoshi", about = "Satoshi Family")]
enum Opt {
    Start(StartOpt),
}

pub async fn cli() -> Result<()> {
    dotenvy::dotenv().ok();
    let opt = Opt::from_args();
    match opt {
        Opt::Start(opt) => {
            let db = PostgresDB::new(opt.postgres_opt).await?;
            db.migrate().await?;

            let jwt = Jwt::new(opt.jwt_opt);

            let mut env = Environment::new();
            env.add_template("login.html", &include_str!("../templates/login.html"))?;
            env.add_template("item.html", &include_str!("../templates/item.html"))?;
            env.add_template("submit.html", &include_str!("../templates/submit.html"))?;
            env.add_template("profile.html", &include_str!("../templates/profile.html"))?;
            env.add_template("index.html", &include_str!("../templates/index.html"))?;
            env.add_template("comment.html", &include_str!("../templates/comment.html"))?;
            env.add_template("template.html", &include_str!("../templates/template.html"))?;

            let ctx = Arc::new(Mutex::new(Context {
                db,
                jwt,
                environment: env,
            }));
            rest_server(ctx.clone(), opt.rest_opt).await?;
        }
    };

    Ok(())
}
