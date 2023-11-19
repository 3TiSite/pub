use client::Client;
use conn::{
  fred::interfaces::{FunctionInterface, HashesInterface},
  KV,
};
use intbin::{bin_u64, u64_bin};
use t3::HeaderMap;

use crate::{
  api, db,
  db::{code, host_bin_mail_id},
  i18n, lua, throw, K,
};

pub async fn post(header: HeaderMap, client: Client, json: String) -> t3::msg!() {
  let (account, password, code): (String, String, String) = sonic_rs::from_str(&json)?;
  let account = xmail::norm(account);
  if !code::verify(i18n::RESET_PASSWORD, &account, &password, code) {
    throw!(header, code, CODE, INVALID)
  }
  let host = &t3::origin_tld(&header)?;
  let (host_bin, mail_id) = host_bin_mail_id(host, &account).await?;
  if let Some(mail_id) = mail_id {
    let passwd = passwd::hash(password.as_bytes());
    let mail_bin = u64_bin(mail_id);
    let uid: Option<Vec<u8>> = KV
      .fcall(
        lua::MAIL_UID_PASSWD_SET,
        &[&K::host_mail_uid(&host_bin), K::UID_PASSWD],
        [&mail_bin[..], &passwd],
      )
      .await?;
    if let Some(uid) = uid {
      let uid = &uid[..];
      let p = KV.pipeline();
      p.hget(K::NAME, uid).await?;
      p.hget(K::LANG, uid).await?;
      client.sign_in(&p, uid).await?;
      let (name, lang, ..): (String, _, (), ()) = p.all().await?;

      return Ok(api::User {
        id: bin_u64(uid),
        name,
        lang: db::lang::get(lang) as _,
      });
    }
  }
  throw!(header, account, ACCOUNT_NOT_EXIST)
}
