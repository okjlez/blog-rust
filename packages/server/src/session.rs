//https://ipinfo.io/ detect vpns and such...
//<script type="application/javascript" src="http://ipinfo.io/?format=jsonp&callback=getIP"></script>
// https://api.pointless.ai/login/request
//^ header (project_key=323223, requested_method=login) pointless.ai will be running multiple sites not just my own at the moment projectkey tells it what api it should use.
// it locates the correct post url based on the method and project key.;

pub mod session_manager {
    use nanoid::nanoid;
    
    pub struct Session {
        id: String,
        user_agent: String,
        duration: u64
    }

    impl Session {
        pub fn new(user_agent: String) -> Session {
            Session { 
               id: nanoid!(),
               user_agent,
               duration: 604800
            }
        }
        pub fn store(&self) {}
    }
}

pub mod gets {
    use rocket::{Route, routes, get};

    #[get("/logout")]
    fn logout() -> &'static str {
        todo!()
    }
    
     pub fn routes() -> Vec<Route>{
        routes![logout]
     }
}

pub mod posts {
    use rocket::{Route, routes, post, http::CookieJar};

    #[post("/login/request")]
    fn login(_cookies: &CookieJar) -> &'static str {
        todo!()
    }
    
     //create session on login....
     pub fn routes() -> Vec<Route>{
        routes![login]
     } 
}

/*
   pub fn new<N, V>(name: N, value: V) -> Self
        where N: Into<Cow<'c, str>>,
              V: Into<Cow<'c, str>>
    {
        CookieBuilder { cookie: Cookie::new(name, value) }
    } */