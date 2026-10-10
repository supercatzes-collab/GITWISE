use dioxus::prelude::*;

use crate::navbar::Navbar;
use crate::pages::{Home, Library, Download, Watching, Pending, Profile, Contribute, Upload, Outgoing, Settings, Login, Signin, IncomingRequest};

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/library")]
    Library {},
    #[route("/download")]
    Download {},
    #[route("/watching")]
    Watching {},
    #[route("/pending")]
    Pending {},
    #[route("/profile")]
    Profile {},
    #[route("/contribute")]
    Contribute {},
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
}