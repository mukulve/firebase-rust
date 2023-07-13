use regex::Regex;

pub struct FirebaseClient {
    endpoint: String, 
    location: Vec<&'static str>, 
}

impl FirebaseClient {
    // Create A New Firebase Client
    pub fn new(endpoint: String) -> Result<FirebaseClient, &'static str>  {
        let regex = Regex::new("https://[\\w,',\",-]+.firebaseio.com/").unwrap();
        let is_match = regex.is_match(endpoint.as_str());

        if !is_match {
            return Err("Invalid Endpoint");
        }

        Ok(Self {
            endpoint,
            location: vec![],
        })
    }

    // Set Location Of Data ex: https://dinosaur-facts.firebaseio.com/dinosaurs
    pub fn set_location(&mut self, location: &'static str) -> Result<&mut FirebaseClient, &'static str> {
        self.location.push(location);
        return Ok(self);
    }

    // Clear All Locations
    pub fn clear_locations(&mut self) -> Result<&mut FirebaseClient, &'static str> {
        self.location.clear();
        return Ok(self);
    }

    // GET - Reading Data
    pub async fn get(&mut self) -> Result<String, &'static str>{
        let url = format!("{}{}.json", self.endpoint, self.location.join("/"));

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
    pub async fn put(&mut self, text_to_put:  &'static str) -> Result<&mut FirebaseClient,  &'static str>{
        let url = format!("{}{}.json", self.endpoint, self.location.join("/"));

        let client = reqwest::Client::new();

        let res = client.put(url)
            .body(text_to_put)
            .send()
            .await;

        match res {
            Ok(_)=> return Ok(self),
            Err(_) => return Err("Unable To Put Data"),
        }
    }

    // POST - Pushing Data
    pub async fn post(&mut self, text_to_put:  &'static str) -> Result<&mut FirebaseClient,  &'static str>{
        let url = format!("{}{}.json", self.endpoint, self.location.join("/"));

        let client = reqwest::Client::new();

        let res = client.post(url)
            .body(text_to_put)
            .send()
            .await;

        match res {
            Ok(_)=> return Ok(self),
            Err(_) => return Err("Unable To Put Data"),
        }
    }

    // PATCH - Updating Data
    pub async fn patch(&mut self, text_to_put:  &'static str) -> Result<&mut FirebaseClient,  &'static str>{
        let url = format!("{}{}.json", self.endpoint, self.location.join("/"));

        let client = reqwest::Client::new();

        let res = client.post(url)
            .body(text_to_put)
            .send()
            .await;

        match res {
            Ok(_)=> return Ok(self),
            Err(_) => return Err("Unable To Put Data"),
        }
    }

    // DELETE - Removing Data
    pub async fn delete(&mut self) -> Result<&mut FirebaseClient,  &'static str>{
        let url = format!("{}{}.json", self.endpoint, self.location.join("/"));

        let client = reqwest::Client::new();

        let res = client.delete(url)
            .send()
            .await;

        match res {
            Ok(_)=> return Ok(self),
            Err(_) => return Err("Unable To Put Data"),
        }
    }


}

#[tokio::main]
async fn main() {

}
