
#[derive(Debug)]
pub struct Timer {
    pub name: String,
    pub description: Option<String>,
    pub schedule: String,
    pub executable: Option<String>,
    pub exec_if_missed: bool,
    pub single_use: bool, // retained as the "promise"
    pub recurring: bool,
    pub on_calendar: bool,
    pub from_boot: bool,
    pub normal_service: bool,
    pub service: Option<String>,
    pub already_made_service: bool,
    pub enable_at_login: bool,
    pub start_after_create: bool,
}
