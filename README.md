# Firebase Rust
Simple wrapper on top of Firebase Realtime Database REST API for rust. 

# Todo 
- [ ] Add auth
- [ ] Add real time updates
- [ ] Clean up code
- [ ] Be able to pass Json object to methods instead of a string json object
- [x] Add query parameters

# Usage

<pre>
  <code>
#[tokio::main]
async fn main() {
    let mut client = FirebaseClient::new("https://dinosaur-facts.firebaseio.com/".to_string()).unwrap();

    client.set_location("location1").unwrap().set_location("location1").unwrap();

    client.order_by("\"price\"").unwrap().limit_to_first("10").unwrap();
  
    client.get().await.unwrap();
  
    client.put("{ \"last\": \"Jones\" }").await.unwrap();
    
    client.post("{ \"last\": \"Jones\" }").await.unwrap();
    
    client.patch("{ \"last\": \"Jones\" }").await.unwrap();
  
    client.clear_locations().unwrap();
  
    client.set_location("new_location").unwrap();
    
    client.delete().await.unwrap();

}
  </code>
</pre>
