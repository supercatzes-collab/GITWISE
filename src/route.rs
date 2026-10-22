    //route.rs
    use crate::navbar::Navbar;
    use crate::pages::*;
    use dioxus::prelude::*;

    #[derive(Routable, Clone, PartialEq)]
    pub enum Route {
        #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/library")]
        Library {},
        #[route("/watching")]
        Watching {},
        #[route("/pending")]
        Pending {},
        #[route("/upload")]
        Upload {},
        #[route("/outgoing")]
        Outgoing {},
        #[route("/logout")]
        Settings {},
        #[route("/login")]
        Login {},
        #[route("/signin")]
        Signin,
        #[route("/view_incoming_request/:proposal_id")]
        IncomingRequest { proposal_id: String },
        #[route("/profile/:user_id")]
        Profile {user_id: String},
        #[route("/download/:item_id")]
        Download { item_id: String },
        #[route("/contribute/:item_id")]
        Contribute { item_id: String },
    }
