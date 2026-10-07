use dioxus::prelude::*;

use crate::navbar::Navbar;
use crate::pages::{Home, Library, Download, Watching, Pending, Profile, Contribute, Upload};

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
}