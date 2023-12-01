use client::Client;
use intbin::u64_bin;
use r::{fred::interfaces::HashesInterface, KV};

use crate::db::name;

pub async fn post(client: Client, json: String) -> t3::msg!() {
  let (uid, name): (u64, String) = sonic_rs::from_str(&json)?;
  let name = name::truncate(name);
  client.uid_logined(uid).await?;

  let uid_bin = u64_bin(uid);
  let uid_bin_ = &uid_bin[..];
  let p = KV.pipeline();
  p.hincrby(user::K::VER, uid_bin_, 1).await?;
  p.hset(user::K::NAME, (uid_bin_, name.as_bytes())).await?;
  p.all().await?;

  let client_id = client.id;
  tokio::spawn(async move {
    m::authNameLog(uid, name).await?;
    let user = ws::msg_user_by_uid_bin(uid_bin).await?;
    ws::to_other(client_id, uid, user).await;
    Ok::<_, anyhow::Error>(())
  });
  Ok(())
}
