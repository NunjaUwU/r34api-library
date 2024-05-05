//! Disclaimer: I made this lil library just for fun, with only a few features,
//! so I wouldnt recommend using it. But if you use it anyway have fun :>.
//! 
//! 
//! There may be some parts where you need to know the R34 API, I probably forgot something 🤷‍♂️. So check out R34's API
//! 
//! 
//!  <https://api.rule34.xxx/>
//! 
//! 
//! # Lil example
//! ```
//! use std::fs;
//! use r34_api as r34;
//! use reqwest;
//! 
//! #[tokio::main]
//! async fn main() {
//!     // First we make a new Api Url.
//!     // We add the 'big_boobs' tag and a post limit of one so only one
//!     // post will be returned and convert the ApiUrl type into a String.
//!     let request_url = r34::ApiUrl::new().add_tag("big_boobs").set_limit(1).to_api_url();
//! 
//!     // Next we send our request to R34's API.
//!     let api_response = reqwest::get(request_url).await.unwrap();
//! 
//!     // We then parse the json response and get a Vector with Post's.
//!     let posts: Vec<r34::Post> = r34::R34JsonParser::default().from_api_response(api_response).unwrap();
//! 
//!     // Here we get the filename and url of the post's file.
//!     let post_file_url = &posts[0].file_url;
//!     let post_file_name = &posts[0].image;
//! 
//!     // Now we Download the file
//!     let file_as_bytes = reqwest::get(post_file).await.unwrap().bytes().await.unwrap();
//!     // Define its path
//!     let path = format!("./{}", post_file_name);
//!     // And save it.
//!     fs::File::create(path).unwrap().write_all(&file_as_bytes).unwrap();
//! }    
//! ```

#![allow(dead_code)]

use core::fmt;
use serde_json::Value;
use std::{collections::HashMap, fmt::Display, str::FromStr};

/// The ApiUrl Struct is used for easily generating API Urls.
/// 
/// # Example
/// ```
/// // It's very simple
/// // You first need to make a new ApiUrl struct
/// let api_url_struct = ApiUrl::new();
/// 
/// // Then u can set configurations like tags, the page id or if you wanna go crazy,
/// // disable the json response and work with html responses.
/// //!! This Crate has no implementation for html handling so if you want to work with html,
/// //!! you have to come up with something yourself.
/// // Lets add only one tag for the start and a request limit of 5.
/// 
/// api_url_struct.add_tag("big_boobs").set_limit(5);
/// 
/// // Now we have to convert our struct into a String that can be used as Url
/// 
/// let api_url = api_url_struct.to_api_url();
/// 
/// // This would be 'https://api.rule34.xxx/index.php?page=dapi&s=post&q=index&tags=big_boobs&limit=5&json=1'
/// ```
/// 
/// ## With Multiple Tags
/// ```
/// let tags: Vec<String> = vec!["big_boobs".to_string(), "big_ass".to_string(), "dark_skin".to_string()];
/// let api_url = ApiUrl::new().add_tags(tags).set_limit(5).to_api_url();
/// 
/// // And here we have it 'https://api.rule34.xxx/index.php?page=dapi&s=post&q=index&tags=big_boobs big_ass dark_skin&limit=5&json=1'
/// ```

pub struct ApiUrl {
    /// Default API Access URL "https://api.rule34.xxx/index.php?page=dapi&s=post&q=index"
    pub api_url: String,
    /// Limitter for max Post per APIUrls. R34s API limmits to max 1000 Posts per ApiUrl. Default Setting is 1000.
    pub req_limit: usize,
    /// All R34 tags should work when exactly taken over. Default is empty.
    pub tags: Vec<String>,
    /// Json Formatted or not Json Formatted: Default is true.
    pub json: bool,
    /// Hashmap with the three IDs:
    /// ID: Filter for ID of Post. (Recommended to use alone)!
    /// PID: Filter for Page ID.
    /// CID: Filter for Change ID (Not Recomended)!
    /// By Default all are empty.
    pub ids: HashMap<String, Option<usize>>,
}

impl ApiUrl {
    /// Creates a new ApiUrl with default settings
    pub fn new() -> ApiUrl {
        ApiUrl::default()
    }

    /// Sets the limit Filter for the ApiUrl
    pub fn set_limit(mut self, limit: usize) -> Self {
        self.req_limit = limit;
        self
    }

    /// Adds a Tag to the ApiUrl
    pub fn add_tag(mut self, tag: &'static str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    pub fn add_tags(mut self, mut tags: Vec<String>) -> Self {
        self.tags.append(&mut tags);
        self
    }

    /// sets the CID of the ApiUrl
    pub fn set_cid(mut self, cid: usize) -> Self {
        self.ids.insert("cid".to_string(), Some(cid));
        self
    }

    /// Sets the PID of the ApiUrl
    pub fn set_pid(mut self, pid: usize) -> Self {
        self.ids.insert("pid".to_string(), Some(pid));
        self
    }

    /// Sets the Post ID of the ApiUrl
    pub fn set_id(mut self, id: usize) -> Self {
        self.ids.insert("id".to_string(), Some(id));
        self
    }

    /// Activates a Json Formatted ApiUrl
    pub fn set_json_formatted(mut self, json: bool) -> Self {
        self.json = json;
        self
    }

    /// Returns the Final ApiUrl as a String
    pub fn to_api_url(&mut self) -> String {
        let api_url = self.api_url.clone();
        let req_limit = self.req_limit;
        let json = if self.json == true { r"&json=1" } else { "" };
        let tags = format!(r"&tags={}", self.tags.join(" "));

        let id = format_id(&self.ids, "id");
        let pid = format_id(&self.ids, "pid");
        let cid = format_id(&self.ids, "cid");

        let req_string = format!(
            r"{}{}&limit={}{}{}{}{}",
            api_url, tags, req_limit, json, id, pid, cid
        );

        req_string
    }
}

impl Default for ApiUrl {
    fn default() -> Self {
        let mut ids: HashMap<String, Option<usize>> = HashMap::new();
        ids.insert("id".to_string(), None);
        ids.insert("pid".to_string(), None);
        ids.insert("cid".to_string(), None);

        ApiUrl {
            api_url: String::from("https://api.rule34.xxx/index.php?page=dapi&s=post&q=index"),
            req_limit: 1000,
            tags: Vec::new(),
            json: true,
            ids,
        }
    }
}

fn format_id(ids: &HashMap<String, Option<usize>>, key: &str) -> String {
    match ids.get(key) {
        Some(Some(value)) => format!("&{}={}", key, value),
        _ => String::new(),
    }
}

/// A Parser for R34 API Json responses.
///
/// Holds a HashMap of config options that can all be tweaked with the `set_conf()` function.
/// See set_conf() for more information.
/// 
/// # Example
/// ```
/// let api_response: &str = ...;
/// 
/// // First we make a new Parser.
/// let r34_json_parser = r34::R34JsonParser::new();
/// 
/// // Then we take our parser and the api response and parse it.
/// // That will return a Vector with every Post of the api response.
/// let posts: Vec<Post> = r34_json_parser.from_api_response(api_response);
/// ```
pub struct R34JsonParser {
    pub conf: HashMap<&'static str, bool>,
}

impl Default for R34JsonParser {
    fn default() -> Self {
        let mut conf: HashMap<&str, bool> = HashMap::new();
        conf.insert("file_url", true);
        conf.insert("image", true);
        conf.insert("tags", true);
        conf.insert("width", true);
        conf.insert("height", true);
        conf.insert("sample", true);
        conf.insert("samlpe_url", true);
        conf.insert("sample_width", true);
        conf.insert("sample_height", true);
        conf.insert("source", true);
        conf.insert("id", true);
        conf.insert("score", true);
        conf.insert("parent_id", true);
        conf.insert("comment_count", true);
        conf.insert("preview_url", true);
        conf.insert("owner", true);
        conf.insert("rating", true);

        R34JsonParser { conf }
    }
}

impl R34JsonParser {
    pub fn new() -> R34JsonParser {
        R34JsonParser::default()
    }

    /// Takes the response of the r34 api and returns a Vector of the parsed Posts
    /// # Errors
    /// This function will return and Error if the response is empty.
    /// An empty response is most likley caused by wrong configurations like wrong Tags.
    pub fn from_api_response(&mut self, s: &str) -> Result<Vec<Post>, R34Error> {
        if s == "" {
            return Err(R34Error::R34EmptyReturn(String::from("One or more Tags didn't exist.")));
        }

        let value = match serde_json::Value::from_str(&s) {
            Ok(value) => value,
            Err(e) => {
                return Err(R34Error::JsonParseError(e));
            }
        };

        let pretty_json = serde_json::to_string_pretty(&value).unwrap();

        Ok(self.parse_json(&pretty_json).unwrap())
    }

    /// Takes a valid json string and returns a Vector of Posts with all information configured.
    pub fn parse_json(&mut self, s: &str) -> Result<Vec<Post>, R34Error> {
        if s == "" {
            return Err(R34Error::R34EmptyReturn(String::from("One or more Tags didn't exist.")));
        }

        let value: Value = match serde_json::Value::from_str(&s) {
            Ok(value) => value,
            Err(e) => {
                return Err(R34Error::JsonParseError(e));
            }
        };

        let mut post_vec: Vec<Post> = Vec::new();

        if let Value::Array(a) = value {
            for obj in a {
                let mut post = Post::default();

                for (k, b) in &mut self.conf {
                    match (k, b) {

                        (&"file_url", true) => {
                            if let Value::Object(ref map) = obj {
                                if let Some(Value::String(s)) = map.get("file_url") {
                                    post.file_url = s.clone();
                                }
                            }
                        }

                        (&"image", true) => {
                            if let Value::Object(ref map) = obj {
                                if let Some(Value::String(s)) = map.get("image") {
                                    post.image = s.clone();
                                }
                            }
                        }
                        (&"tags", true) => {
                            if let Value::Object(ref map) = obj {
                                if let Some(Value::String(s)) = map.get("tags") {
                                    post.tags = s
                                        .clone()
                                        .split_whitespace()
                                        .map(move |s| s.to_string())
                                        .collect();
                                }
                            }
                        }
                        (&"width", true) => {
                            if let Value::Object(ref map) = obj {
                                if let Some(Value::Number(n)) = map.get("width") {
                                    post.width = n.as_u64().unwrap();
                                }
                            }
                        }
                        (&"height", true) => {
                            if let Value::Object(ref map) = obj {
                                if let Some(Value::Number(n)) = map.get("height") {
                                    post.height = n.as_u64().unwrap();
                                }
                            }
                        }
                        (&"sample", true) => {
                            if let Value::Object(ref map) = obj {
                                if let Some(Value::Bool(b)) = map.get("sample") {
                                    post.sample = b.clone();
                                }
                            }
                        }
                        (&"sample_url", true) => {
                            if let Value::Object(ref map) = obj {
                                if let Some(Value::String(s)) = map.get("sample_url") {
                                    post.sample_url = s.clone();
                                }
                            }
                        }
                        (&"sample_width", true) => {
                            if let Value::Object(ref map) = obj {
                                if let Some(Value::Number(n)) = map.get("sample_width") {
                                    post.sample_width = n.as_u64().unwrap();
                                }
                            }
                        }
                        (&"sample_height", true) => {
                            if let Value::Object(ref map) = obj {
                                if let Some(Value::Number(n)) = map.get("sample_height") {
                                    post.sample_height = n.as_u64().unwrap();
                                }
                            }
                        }
                        (&"source", true) => {
                            if let Value::Object(ref map) = obj {
                                if let Some(Value::String(s)) = map.get("source") {
                                    post.source = s.clone();
                                }
                            }
                        }
                        (&"id", true) => {
                            if let Value::Object(ref map) = obj {
                                if let Some(Value::Number(n)) = map.get("id") {
                                    post.id = n.as_u64().unwrap();
                                }
                            }
                        }
                        (&"score", true) => {
                            if let Value::Object(ref map) = obj {
                                if let Some(Value::Number(n)) = map.get("score") {
                                    post.score = n.as_u64().unwrap();
                                }
                            }
                        }
                        (&"parent_id", true) => {
                            if let Value::Object(ref map) = obj {
                                if let Some(Value::Number(n)) = map.get("parent_id") {
                                    post.parent_id = n.as_u64().unwrap();
                                }
                            }
                        }
                        (&"comment_count", true) => {
                            if let Value::Object(ref map) = obj {
                                if let Some(Value::Number(n)) = map.get("comment_count") {
                                    post.comment_count = n.as_u64().unwrap();
                                }
                            }
                        }
                        (&"preview_url", true) => {
                            if let Value::Object(ref map) = obj {
                                if let Some(Value::String(s)) = map.get("preview_url") {
                                    post.preview_url = s.clone();
                                }
                            }
                        }
                        (&"owner", true) => {
                            if let Value::Object(ref map) = obj {
                                if let Some(Value::String(s)) = map.get("owner") {
                                    post.owner = s.clone();
                                }
                            }
                        }
                        (&"rating", true) => {
                            if let Value::Object(ref map) = obj {
                                if let Some(Value::String(s)) = map.get("rating") {
                                    match s.as_str() {
                                        "explicit" => post.rating = Some(Rating::Explicit),
                                        "safe" => post.rating = Some(Rating::Safe),
                                        "questionable" => post.rating = Some(Rating::Questionable),
                                        _ => post.rating = None,
                                    }
                                }
                            }
                        }
                        _ => (),
                    }
                }
                post_vec.push(post);
            }
        }
        Ok(post_vec)
    }

    /// Set Conifg options with the name of the field and a bool.
    ///
    /// Will be changed to enum or something
    /// All keys/fieldnames are &str's e.g. `"file_url"` `"id"`
    ///
    ///
    /// Possible Keys:
    /// ```
    /// "id";
    /// "parent_id";
    /// "image";
    /// "tags";
    /// "source";
    /// "owner";
    /// "score";
    /// "comment_count";
    /// "rating";
    /// "sample";
    /// "file_url";
    /// "sample_url";
    /// "preview_url";
    /// "width";
    /// "height";
    /// "sample_width";
    /// "sample_height";
    /// ```
    pub fn set_conf(mut self, key: &'static str, set: bool) -> Self {
        *self.conf.get_mut(&key).unwrap() = set;
        self
    }
}

/// Specifically used for Parsing of Json API Rseponses
#[derive(Debug)]
pub enum R34Error {
    /// Wrapper so i can use my own Error (Will be improved in future)
    JsonParseError(serde_json::Error),
    /// Returns when a Json file is empty
    R34EmptyReturn(String),
}

impl std::fmt::Display for R34Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            R34Error::JsonParseError(e) => write!(f, "{}", e),
            R34Error::R34EmptyReturn(e) => write!(f, "{}", e)
        }
    }
}

/// Rating of the Post as enum.
/// Will probably get removed in the future!
#[derive(Clone, Copy)]
pub enum Rating {
    Explicit,
    Safe,
    Questionable,
}

impl fmt::Display for Rating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = match *self {
            Self::Explicit => "explicit",
            Self::Safe => "safe",
            Self::Questionable => "questionable",
        };
        write!(f, "{}", r)
    }
}

/// Post struct. Holds all information about a Post
#[derive(Clone)]
pub struct Post {
    /// Url of Post's File
    pub file_url: String,
    /// Name of Post's File
    pub image: String,
    /// Post's Tags
    pub tags: Vec<String>,
    /// Width of Post's File
    pub width: u64,
    /// Height of Post's File
    pub height: u64,

    /// Tells if the Post has a Sample
    pub sample: bool,
    /// Url of Post's Sample
    pub sample_url: String,

    /// Height of Post's Sample File
    pub sample_width: u64,
    /// Height of Post's Sample File
    pub sample_height: u64,
    /// Source of Post e.g. Twitter url etc.
    pub source: String,
    /// ID of Post
    pub id: u64,
    /// Score of Post
    pub score: u64,
    /// ID of Post's Parent Post
    pub parent_id: u64,
    /// Amount of Comments on the Post
    pub comment_count: u64,
    /// Url of Post's Preview Image
    pub preview_url: String,
    /// Name of Post's Owner/Poster
    pub owner: String,
    /// Rating of Post e.g. Safe, Explicit or Questionable
    pub rating: Option<Rating>,
}

impl Display for Post {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "image: {}\nfile_url: {}\nwidth: {}\nheight: {}\ntags: {:?}\nid: {}\nowner: {}\nrating: {}\nsample_url: {}\nsample_width: {}\nsample_height: {}\nsource: {}\nscore: {}\nparent_id: {}\ncomment_count: {}\npreview_url: {}\n",
             self.image,  self.file_url, self.width,
             self.height, self.tags, self.id, 
             self.owner, self.rating.clone().unwrap(), self.sample_url,
            self.sample_width, self.sample_height,
            self.source, self.score, self.parent_id,
            self.comment_count, self.preview_url)
    }
}

impl Default for Post {
    fn default() -> Self {
        Post {
            file_url: String::new(),
            width: 0,
            height: 0,
            image: String::new(),
            tags: vec![],
            sample: false,
            sample_url: String::new(),
            sample_width: 0,
            sample_height: 0,
            source: String::new(),
            id: 1,
            score: 0,
            parent_id: 0,
            comment_count: 0,
            preview_url: String::new(),
            owner: String::new(),
            rating: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::{Read, Write},
        time,
    };

    #[test]
    fn bench_json_parse_this() {
        let mut buf = String::from("");
        fs::File::open("./response.json")
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
        let json = buf.as_str();

        let now = time::Instant::now();
        let _posts = super::R34JsonParser::default().parse_json(json);
        let elapsed = now.elapsed();
        println!("Full: {:?}", elapsed);

        // let _file = fs::OpenOptions::new()
        //     .write(true)
        //     .append(true)
        //     .open("./test.txt")
        //     .unwrap()
        //     .write(format!("Full: {}\n", elapsed.as_nanos()).as_bytes())
        //     .unwrap();
    }

    #[test]
    fn test_json_parse_this() {
        let mut buf = String::from("");
        fs::File::open("./response.json")
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
        let json = buf.as_str();

        let posts = super::R34JsonParser::default().parse_json(json).unwrap();
        let post1 = &posts[0].to_string();
        let post2 = &posts[1].to_string();

        let _file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("./test.txt")
            .unwrap()
            .write_all(format!("{}\n{}\n", post1, post2).as_bytes()).unwrap();
    }
}
