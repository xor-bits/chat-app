use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::local::{Db, Mem},
    Surreal,
};

use crate::Message;

//

/* pub struct Database {
    inner: Arc<LazyAwait<DatabaseInner>>,
} */

#[derive(Clone)]
pub struct Database {
    db: Surreal<Db>,
}

//

impl Database {
    pub async fn new() -> anyhow::Result<Self> {
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("chat-app").use_db("chat-app").await?;

        Self { db }.init().await
    }

    pub async fn send_msg(&self, msg: &str) -> anyhow::Result<Message> {
        #[derive(Serialize)]
        struct CreateMsg<'a> {
            date: DateTime<Utc>,
            content: &'a str,
        }

        let res: Message = self
            .db
            .create("messages")
            .content(CreateMsg {
                date: Utc::now(),
                content: msg,
            })
            .await?;

        Ok(res)
    }

    pub async fn get_msg(&self) -> anyhow::Result<Vec<Message>> {
        const Q: &str = "SELECT * FROM messages ORDER BY date DESC LIMIT 10";

        let msg: Vec<Message> = self.db.query(Q).await?.take(0)?;

        Ok(msg)
    }

    async fn init(self) -> anyhow::Result<Self> {
        /* const Q: &str = r#"CREATE TABLE messages (
            id UUID NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY
        )"#;

        self.exec(Q, None).await?; */

        Ok(self)
    }

    /* async fn exec(
        &self,
        query: &str,
        vars: Option<BTreeMap<String, Value>>,
    ) -> anyhow::Result<Vec<Response>> {
        let res = self.ds.execute(query, &self.se, vars, false).await?;
        Ok(res)
    } */
}
