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
    ) -> WebauthnResult<()> {
        log::debug!(
            "handle Register -> (nick: {:?}, email: {:?}, reg: {:?})",
            user.nick,
            user.email,
            reg
        );

        // TODO: register needs to return the user's id -> for creation of the list

        // check if a user with this email already exists in the database
        let result = db_manager.get_user_by_email(user.email.clone());
        let registered_user = match result {
            Ok(existing_user) => existing_user,
            Err(_) => {
                // if not, create a new user with the username
                match db_manager.create_user(user.clone()) {
                    Ok(new_user) => new_user,
                    Err(_) => panic!(
                        "Database Error: Could not create new user {} ({})",
                        user.nick, user.email
                    ),
                }
            }
        };

        let username = registered_user.nick.as_bytes().to_vec();

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

        log::debug!("complete Register -> {:?}", r);
        return r;
    }
}
