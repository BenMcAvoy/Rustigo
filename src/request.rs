/// Request struct
#[derive(Debug)]
pub struct Request {
    pub user_agent: String,
    pub resource: String,
    pub mode: String,
    pub content_length: Option<usize>,
}

impl Request {
    /// Create a new Request struct.
    /// This will "parse" the raw request and create a Request struct.
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

        let content_length = raw
            .iter()
            .find(|x| x.contains("Content-Length: "))
            .and_then(|x| {
                x.strip_prefix("Content-Length: ")
                    .and_then(|x| x.parse::<usize>().ok())
            });

        Ok(Self {
            user_agent,
            resource,
            mode,
            content_length,
        })
    }
}
