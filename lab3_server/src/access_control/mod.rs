use std::error::Error;

use casbin;
use casbin::CoreApi;
use serde::Serialize;
use strum_macros::{Display, EnumString};

use crate::user::UserAccount;

static MODEL: &str = "src/access_control/model.conf";
static POLICY: &str = "src/access_control/policy.csv";

#[derive(Clone, Debug, Display, EnumString, Serialize, Hash)]
pub enum AccessObject {
    #[strum(serialize = "show_users")]
    ShowUsers,
    #[strum(serialize = "change_own_phone")]
    ChangeOwnPhone,
    #[strum(serialize = "change_phone")]
    ChangePhone,
    #[strum(serialize = "add_user")]
    AddUser,
}

pub struct AccessController {
    enforcer: casbin::Enforcer,
}

impl AccessController {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let mut e = casbin::Enforcer::new(MODEL, POLICY).await?;
        e.enable_log(true); // On pourrait faire en sorte de mieux l'intégrer avec simplelog
        Ok(Self { enforcer: e })
    }

    pub fn enforce(&self, req: Request) -> Result<bool, Box<dyn Error>> {
        let res = self
            .enforcer
            .enforce((&req.username, &req.object.to_string(), "access"));
        Ok(res?)
    }
}

#[derive(Clone, Debug, Serialize, Hash)]
pub struct Request {
    pub username: String,
    pub object: AccessObject,
}

impl Request {
    pub fn new(user: &UserAccount, object: AccessObject) -> Self {
        let username = user.username().to_string();
        Self { username, object }
    }
}
