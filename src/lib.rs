//! # Intro
//! 
//! Hi my english isnt the best so sorry if there are some errors in my grammer. I made this lil crate, just for fun, with only a few features,
//! so I wouldnt recommend using it but of course I would be happy if you use it none the less.
//! 
//! 
//! There may be some parts where you need to know the R34 API cause i forgot something. So check out their API
//! 
//! 
//!  <https://api.rule34.xxx/>

#![allow(dead_code)]

use core::fmt;
use serde_json::Value;
use std::{collections::HashMap, fmt::Display};

pub struct Request {
    /// Default API Access URL "https://api.rule34.xxx/index.php?page=dapi&s=post&q=index"
    pub api_url: String,
    /// Limitter for max Post per requests. R34s API limmits to max 1000 Posts per request. Default Setting is 1000.
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

fn format_id(ids: &HashMap<String, Option<usize>>, key: &str) -> String {
    match ids.get(key) {
        Some(Some(value)) => format!("&{}={}", key, value),
        _ => String::new(),
    }
}

impl Request {
    /// Creates a new Request by default settings
    pub fn new() -> Request {
        Request::default()
    }

    /// Sets the limit Filter for the request
    pub fn set_limit(mut self, limit: usize) -> Self {
        self.req_limit = limit;
        self
    }

    /// Adds a Tag to the request
    pub fn add_tag(mut self, tag: &'static str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    pub fn add_tags(mut self, mut tags: Vec<String>) -> Self {
        self.tags.append(&mut tags);
        self
    }

    /// sets the CID of the request
    pub fn set_cid(mut self, cid: usize) -> Self {
        self.ids.insert("cid".to_string(), Some(cid));
        self
    }

    /// Sets the PID of the Request
    pub fn set_pid(mut self, pid: usize) -> Self {
        self.ids.insert("pid".to_string(), Some(pid));
        self
    }

    /// Sets the Post ID of the Request
    pub fn set_id(mut self, id: usize) -> Self {
        self.ids.insert("id".to_string(), Some(id));
        self
    }

    /// Activates a Json Formatted Request
    pub fn set_json_formatted(mut self, json: bool) -> Self {
        self.json = json;
        self
    }

    /// Returns the Final Request URL as a String
    pub fn to_req_url(&mut self) -> String {
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

impl Default for Request {
    fn default() -> Self {
        let mut ids: HashMap<String, Option<usize>> = HashMap::new();
        ids.insert("id".to_string(), None);
        ids.insert("pid".to_string(), None);
        ids.insert("cid".to_string(), None);

        Request {
            api_url: String::from("https://api.rule34.xxx/index.php?page=dapi&s=post&q=index"),
            req_limit: 1000,
            tags: Vec::new(),
            json: true,
            ids,
        }
    }
}

/// A Parser for R34 Json responses.
///
/// Holds a HashMap of config options that can all be tweaked with the `set_conf()` function.
/// Explenation and possible inputs are found in the functions description.
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
    /// Takes a valid json string and returns a Vector of Posts with all information configured.
    pub fn parse_json(&mut self, s: &str) -> Vec<Post> {
        let v: Value = serde_json::from_str(s).unwrap();

        let mut post_vec: Vec<Post> = Vec::new();

        if let Value::Array(a) = v {
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
                                    post.file_url = s.clone();
                                }
                            }
                        }
                        _ => (),
                    }
                }

                post_vec.push(post);
            }
        }
        post_vec
    }

    /// Set Conifg options with the name of the field and a bool.
    ///
    /// Will be changed to enum or something
    /// All keys/fieldnames are &str's e.g. `"file_url"` `"id"`
    ///
    ///
    /// Possible Keys:
    /// ```
    /// "id"
    /// "parent_id"
    /// "image"
    /// "tags"
    /// "source"
    /// "owner"
    /// "score"
    /// "comment_count"
    /// "rating"
    /// "sample"
    /// "file_url"
    /// "sample_url"
    /// "preview_url"
    /// "width"
    /// "height"
    /// "sample_width"
    /// "sample_height"
    /// ```
    pub fn set_conf(mut self, key: &'static str, set: bool) -> Self {
        *self.conf.get_mut(&key).unwrap() = set;
        self
    }
}

/// Rating of the Post as enum.
/// Will maybe get removed in future!
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
    /// Source of Post e.g. Twitter User etc.
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

        let posts = super::R34JsonParser::default().parse_json(json);
        let post = &posts[0];

        let _file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("./test.txt")
            .unwrap()
            .write(format!("Full: {}\n", &post.to_string()).as_bytes())
            .unwrap();
    }
}
