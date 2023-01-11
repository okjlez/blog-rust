use rocket::{
    serde::{Deserialize, Serialize},
};

// id: String::from(SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_nanos().to_string())

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
//Account<S, D>
//Account<
pub struct Account {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: usize
}

impl Account {

}

/* 
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AccountDeserializer {
    input: String,
}

impl Account {
    pub fn create(id: String, username: String, password: String, email: String) -> Self {
        Self {
            id,
            username,
            password,
            email,
        }
    }
    pub fn serialize(&self) -> AccountDeserializer {
        AccountDeserializer {
            input: serde_json::to_string(&self).unwrap(),
        }
    }
}

impl AccountDeserializer {
    pub fn raw_value(&self) -> &String {
        &self.input
    }

    pub fn deserialize(&self) -> Result<Account, serde_json::Error> {
        serde_json::from_str(&self.input)
    }
}
*/
/*

   pub fn serialize(&self) -> AccountDeserializer {
        AccountDeserializer {
            input: serde_json::to_string(&self).unwrap()
        }
    }

#[derive(Debug, Default)]
pub struct AccountBuilder {
    pub id: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
}

impl AccountBuilder {
    pub fn new() -> AccountBuilder {
        Self {
            id: None,
            username: None,
            password: None,
            email: None
        }
    }

    pub fn id(mut self, new_id: String) -> AccountBuilder {
        self.id = Some(new_id);
        self
    }

    pub fn username(mut self, new_username: String) -> AccountBuilder {
        self.username = Some(new_username);
        self
    }

    pub fn password(mut self, new_password: String) -> AccountBuilder {
        self.password = Some(new_password);
        self
    }

    pub fn email(mut self,  new_email: String) -> AccountBuilder {
        self.email = Some(new_email);
        self
    }

    pub fn build(self) -> Account {
        Account {
            id: self.id.unwrap(),
            username: self.username.unwrap(),
            password: self.password.unwrap(),
            email: self.email.unwrap()
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AccountDeserializer {
    input: String
}

impl AccountDeserializer {
    pub fn raw_value(&self) -> &String {
        &self.input
    }

    pub fn deserialize(&self) -> Result<Account, serde_json::Error> {
        serde_json::from_str(&self.input)
    }
}
*/
