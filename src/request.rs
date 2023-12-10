#[derive(Debug)]
pub struct Request {
    pub user_agent: String,
    pub resource: String,
    pub mode: String,
}

impl Request {
    pub fn new(raw: Vec<String>) -> Result<Self, String> {
        let user_agent = raw
            .iter()
            .find(|x| x.contains("User-Agent: "))
            .ok_or("No user agent.")?
            .strip_prefix("User-Agent: ")
            .ok_or("Failed to strip.")?
            .to_string();

        let resource = raw
            .first()
            .ok_or("No mode")?
            .split(' ')
            .nth(1)
            .ok_or("Failed to get resource.")?
            .to_string();

        let mode = raw
            .first()
            .ok_or("No mode")?
            .split(' ')
            .nth(0)
            .ok_or("Failed to get resource.")?
            .to_string();

        Ok(Self {
            user_agent,
            resource,
            mode,
        })
    }
}
