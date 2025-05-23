mod home;
pub use home::Home;



mod login;
pub use login::Login;

mod register;
pub use register::Register;

mod profile;
pub use profile::Profile;

mod settings;
pub use settings::Settings;

mod create_edit;
pub use create_edit::Create_edit;

mod article;
pub use article::Article;

mod global_feed; // <-- New module
pub use global_feed::GlobalFeed;

mod your_feed;   // <-- New module
pub use your_feed::YourFeed;
