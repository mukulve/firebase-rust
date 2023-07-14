use regex::Regex;

pub struct FirebaseClient {
    endpoint: String,
    location: Vec<&'static str>,
    order_by: String,
    limit_to_first: String,
    limit_to_last: String,
    start_at: String,
    end_at: String,
    equal_to: String,
}

impl FirebaseClient {
    // Create A New Firebase Client
    pub fn new(endpoint: String) -> Result<FirebaseClient, &'static str> {
        let regex = Regex::new("https://[\\w,',\",-]+.firebaseio.com/").unwrap();
        let is_match = regex.is_match(endpoint.as_str());

        if !is_match {
            return Err("Invalid Endpoint");
        }

        Ok(Self {
            endpoint,
            location: vec![],
            order_by: String::new(),
            limit_to_first: String::new(),
            limit_to_last: String::new(),
            start_at: String::new(),
            end_at: String::new(),
            equal_to: String::new(),
        })
    }

    // Set Price First Parameter
    pub fn order_by(
        &mut self,
        parameter: &'static str,
    ) -> Result<&mut FirebaseClient, &'static str> {
        self.order_by = parameter.to_string();
        return Ok(self);
    }

    // Set Limit To First Parameter
    pub fn limit_to_first(
        &mut self,
        parameter: &'static str,
    ) -> Result<&mut FirebaseClient, &'static str> {
        self.limit_to_first = parameter.to_string();
        return Ok(self);
    }

    // Set Limit To Last Parameter
    pub fn limit_to_last(
        &mut self,
        parameter: &'static str,
    ) -> Result<&mut FirebaseClient, &'static str> {
        self.limit_to_last = parameter.to_string();
        return Ok(self);
    }

    // Set Start At Parameter
    pub fn start_at(
        &mut self,
        parameter: &'static str,
    ) -> Result<&mut FirebaseClient, &'static str> {
        self.start_at = parameter.to_string();
        return Ok(self);
    }

    // Set End At Parameter
    pub fn end_at(&mut self, parameter: &'static str) -> Result<&mut FirebaseClient, &'static str> {
        self.end_at = parameter.to_string();
        return Ok(self);
    }

    // Set Equal To Parameter
    pub fn equal_to(
        &mut self,
        parameter: &'static str,
    ) -> Result<&mut FirebaseClient, &'static str> {
        self.equal_to = parameter.to_string();
        return Ok(self);
    }

    // Clear All Parameters
    pub fn clear_parameters(&mut self) -> Result<&mut FirebaseClient, &'static str> {
        self.order_by.clear();
        self.limit_to_first.clear();
        self.limit_to_last.clear();
        self.start_at.clear();
        self.end_at.clear();
        self.equal_to.clear();
        return Ok(self);
    }

    // Set Location Of Data ex: https://dinosaur-facts.firebaseio.com/dinosaurs
    pub fn set_location(
        &mut self,
        location: &'static str,
    ) -> Result<&mut FirebaseClient, &'static str> {
        self.location.push(location);
        return Ok(self);
    }

    // Clear All Locations
    pub fn clear_locations(&mut self) -> Result<&mut FirebaseClient, &'static str> {
        self.location.clear();
        return Ok(self);
    }

    // GET - Reading Data
    pub async fn get(&mut self) -> Result<String, &'static str> {
        let mut url = format!("{}{}.json", self.endpoint, self.location.join("/"));

        if !self.order_by.is_empty() {
            url.push_str(&format!("?orderBy={}", &self.order_by));
        }

        if !self.limit_to_first.is_empty() {
            url.push_str(&format!("&limitToFirst={}", &self.limit_to_first));
        }

        if !self.limit_to_last.is_empty() {
            url.push_str(&format!("&limitToLast={}", &self.limit_to_last));
        }

        if !self.start_at.is_empty() {
            url.push_str(&format!("&startAt={}", &self.start_at));
        }

        if !self.end_at.is_empty() {
            url.push_str(&format!("&endAt={}", &self.end_at));
        }

        if !self.equal_to.is_empty() {
            url.push_str(&format!("&equalTo={}", &self.equal_to));
        }

        let resp = reqwest::get(url).await;

        match resp {
            Ok(response) => {
                let body = response.text().await;

                match body {
                    Ok(json) => return Ok(json),
                    Err(_) => return Err("Unable To Parse Body Of Get Response"),
                }
            }
            Err(_) => return Err("Unable To Send Get Request"),
        }
    }

    // PUT - Writing Data
    pub async fn put(
        &mut self,
        text_to_put: &'static str,
    ) -> Result<&mut FirebaseClient, &'static str> {
        let url = format!("{}{}.json", self.endpoint, self.location.join("/"));

        let client = reqwest::Client::new();

        let res = client.put(url).body(text_to_put).send().await;

        match res {
            Ok(_) => return Ok(self),
            Err(_) => return Err("Unable To Put Data"),
        }
    }

    // POST - Pushing Data
    pub async fn post(
        &mut self,
        text_to_put: &'static str,
    ) -> Result<&mut FirebaseClient, &'static str> {
        let url = format!("{}{}.json", self.endpoint, self.location.join("/"));

        let client = reqwest::Client::new();

        let res = client.post(url).body(text_to_put).send().await;

        match res {
            Ok(_) => return Ok(self),
            Err(_) => return Err("Unable To Post Data"),
        }
    }

    // PATCH - Updating Data
    pub async fn patch(
        &mut self,
        text_to_put: &'static str,
    ) -> Result<&mut FirebaseClient, &'static str> {
        let url = format!("{}{}.json", self.endpoint, self.location.join("/"));

        let client = reqwest::Client::new();

        let res = client.post(url).body(text_to_put).send().await;

        match res {
            Ok(_) => return Ok(self),
            Err(_) => return Err("Unable To Patch Data"),
        }
    }

    // DELETE - Removing Data
    pub async fn delete(&mut self) -> Result<&mut FirebaseClient, &'static str> {
        let url = format!("{}{}.json", self.endpoint, self.location.join("/"));

        let client = reqwest::Client::new();

        let res = client.delete(url).send().await;

        match res {
            Ok(_) => return Ok(self),
            Err(_) => return Err("Unable To Delete Data"),
        }
    }
}

#[tokio::main]
async fn main() {}
