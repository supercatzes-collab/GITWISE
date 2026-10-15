use dioxus::prelude::*;
use crate::navbar::Navbar;
use crate::pages::*;

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
    #[route("/view_incoming_request")]
    IncomingRequest,
    #[route("/profile/:user_id")]
    Profile { user_id: String },
    #[route("/download/:item_id")]
    Download { item_id: String },
    #[route("/contribute/:item_id")]
    Contribute { item_id: String },
}