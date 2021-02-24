use webauthn_rs::ephemeral::WebauthnEphemeralConfig;
use webauthn_rs::error::WebauthnError;
use webauthn_rs::proto::{
    CreationChallengeResponse, Credential, CredentialID, PublicKeyCredential,
    RegisterPublicKeyCredential, RequestChallengeResponse, UserId, UserVerificationPolicy,
};
use webauthn_rs::{AuthenticationState, RegistrationState, Webauthn};

use async_std::sync::Mutex;
use lru::LruCache;
use std::collections::BTreeMap;

use crate::db;
use crate::models::CreateUser;

type WebauthnResult<T> = core::result::Result<T, WebauthnError>;

const CHALLENGE_CACHE_SIZE: usize = 256;

pub struct WebauthnActor {
    wan: Webauthn<WebauthnEphemeralConfig>,
    reg_chals: Mutex<LruCache<UserId, RegistrationState>>,
    auth_chals: Mutex<LruCache<UserId, AuthenticationState>>,
    creds: Mutex<BTreeMap<UserId, BTreeMap<CredentialID, Credential>>>,
}

impl WebauthnActor {
    pub fn new(config: WebauthnEphemeralConfig) -> Self {
        WebauthnActor {
            wan: Webauthn::new(config),
            reg_chals: Mutex::new(LruCache::new(CHALLENGE_CACHE_SIZE)),
            auth_chals: Mutex::new(LruCache::new(CHALLENGE_CACHE_SIZE)),
            creds: Mutex::new(BTreeMap::new()),
        }
    }

    // register returns the registered user's database id -> needed for creation of a list
    pub async fn challenge_register(
        &self,
        nick: String,
    ) -> WebauthnResult<CreationChallengeResponse> {
        log::info!("Webauthn: Challenge Register -> {:?}", nick);
        let (ccr, rs) = self
            .wan
            .generate_challenge_register(&nick, Some(UserVerificationPolicy::Discouraged))?;
        self.reg_chals.lock().await.put(nick.into_bytes(), rs);
        log::info!("Webauthn: Challenge Register Complete -> {:?}", ccr);

        return Ok(ccr);
    }

    pub async fn register(
        &self,
        user: CreateUser,
        reg: RegisterPublicKeyCredential,
        db_manager: db::DBManager,
    ) -> WebauthnResult<i64> {
        log::info!(
            "handle Register -> (nick: {:?}, email: {:?}, reg: {:?})",
            user.nick,
            user.email,
            reg
        );

        // check if a user with this email already exists in the database
        let result = db_manager.get_user_by_email(user.email.clone());
        let registered_user = match result {
            Ok(existing_user) => existing_user,
            Err(_) => {
                // if not, create a new user with the username
                match db_manager.create_user(user.clone()) {
                    Ok(new_user) => new_user,
                    Err(_) => Err(WebauthnError::InvalidUsername)?,
                }
            }
        };

        let username = user.nick.as_bytes().to_vec();

        let rs = self
            .reg_chals
            .lock()
            .await
            .pop(&username)
            .ok_or(WebauthnError::ChallengeNotFound)?;

        // TODO: add new credential to the database, replace the code below

        let mut creds = self.creds.lock().await;
        let r = match creds.get_mut(&username) {
            Some(ucreds) => self
                .wan
                .register_credential(&reg, rs, |cred_id| Ok(ucreds.contains_key(cred_id)))
                .map(|cred| {
                    let cred_id = cred.cred_id.clone();
                    ucreds.insert(cred_id, cred);
                }),
            None => {
                let r = self
                    .wan
                    .register_credential(&reg, rs, |_| Ok(false))
                    .map(|cred| {
                        let mut t = BTreeMap::new();
                        let credential_id = cred.cred_id.clone();
                        t.insert(credential_id, cred);
                        creds.insert(username, t);
                    });
                log::debug!("{:?}", self.creds);
                r
            }
        };

        log::info!("completed Register -> {:?}", r);
        log::info!("completed Register for user {:?}", registered_user);

        return Ok(registered_user.id);
    }

    pub async fn challenge_authenticate(
        &self,
        nick: &String,
    ) -> WebauthnResult<RequestChallengeResponse> {
        log::info!("handle ChallengeAuthenticate -> {:?}", nick);

        // TODO: get the creds from the database here

        let creds = match self.creds.lock().await.get(&nick.as_bytes().to_vec()) {
            Some(creds) => Some(creds.iter().map(|(_, v)| v.clone()).collect()),
            None => None,
        }
        .ok_or(WebauthnError::CredentialRetrievalError)?;

        let (acr, st) = self.wan.generate_challenge_authenticate(creds)?;
        self.auth_chals
            .lock()
            .await
            .put(nick.as_bytes().to_vec(), st);

        log::debug!("complete ChallengeAuthenticate -> {:?}", acr);
        Ok(acr)
    }
}
