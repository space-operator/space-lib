use crate::{http, common::Method, error::HostError};

pub struct Supabase {
    url: String,
    headers: Vec<(String, String)>,
}

impl Supabase {
    pub fn new<T: Into<String>>(url: T) -> Self {
        Self {
            url: url.into(),
            headers: Vec::new(),
        }
    }

    pub fn insert_header<T, U>(mut self, header: T, value: U) -> Self where T: Into<String>, U: Into<String> {
        self.headers.push((header.into(), value.into()));
        self
    }
    
    pub fn from<T: Into<String>>(&self, url: T) -> Builder {
        Builder {
            client: self,
            url: url.into(),
            method: Method::GET,
            body: None,
            headers: self.headers.clone(),
            queries: Vec::new(),
        }
    }
}

pub struct Builder<'a> {
    client: &'a Supabase,
    url: String,
    method: Method,
    body: Option<String>,
    headers: Vec<(String, String)>,
    queries: Vec<(String, String)>,
}

impl Builder<'_> {
    pub fn auth<T: Into<String>>(mut self, token: T) -> Self {
        self.headers.push((
            String::from("Authorization"),
            format!("Bearer {}", token.into()),
        ));
        self
    }

    pub fn select<T: Into<String>>(mut self, columns: T) -> Self {
        self.queries.push((String::from("select"), columns.into()));
        self
    }

    pub fn order<T: Into<String>>(mut self, columns: T) -> Self {
        self.queries.push((String::from("order"), columns.into()));
        self
    }

    pub fn limit(mut self, count: usize) -> Self {
        self.headers
            .push((String::from("Range-Unit"), String::from("items")));
        self.headers
            .push((String::from("Range"), format!("0-{}", count - 1)));
        self
    }

    pub fn range(mut self, low: usize, high: usize) -> Self {
        self.headers
            .push((String::from("Range-Unit"), String::from("items")));
        self.headers
            .push((String::from("Range"), format!("{low}-{high}")));
        self
    }

    fn count(mut self, method: &str) -> Self {
        self.headers
            .push((String::from("Range-Unit"), String::from("items")));
        self.headers.push((String::from("Range"), String::from("0-0")));
        self.headers
            .push((String::from("Insert"), format!("count={method}")));
        self
    }

    pub fn exact_count(self) -> Self {
        self.count("exact")
    }

    pub fn planned_count(self) -> Self {
        self.count("planned")
    }

    pub fn estimated_count(self) -> Self {
        self.count("estimated")
    }

    pub fn single(mut self) -> Self {
        self.headers.push((
            String::from("Accept"),
            String::from("application/vnd.pgrst.object+json"),
        ));
        self
    }

    pub fn insert<T: Into<String>>(mut self, body: T) -> Self {
        self.method = Method::POST;
        self.headers.push((String::from("Prefer"), String::from("return=representation")));
        self.body = Some(body.into());
        self
    }

    pub fn upsert<T: Into<String>>(mut self, body: T) -> Self {
        self.method = Method::POST;
        self.headers.push((String::from("Prefer"), String::from("return=representation,resolution=merge-duplicates")));
        self.body = Some(body.into());
        self
    }

    pub fn on_conflict<T: Into<String>>(mut self, columns: T) -> Self {
        self.queries.push((String::from("on_conflict"), columns.into()));
        self
    }

    pub fn update<T: Into<String>>(mut self, body: T) -> Self {
        self.method = Method::PATCH;
        self.headers.push((String::from("Prefer"), String::from("return=representation")));
        self.body = Some(body.into());
        self
    }

    pub fn delete(mut self) -> Self {
        self.method = Method::DELETE;
        self.headers.push((String::from("Prefer"), String::from("return=representation")));
        self
    }

    pub fn not<T, U, V>(mut self, operator: T, column: U, filter: V) -> Self
    where
        T: Into<String>,
        U: Into<String>,
        V: Into<String>,
    {
        self.queries.push((
            column.into(),
            format!("not.{}.{}", operator.into(), filter.into()),
        ));
        self
    }
    
    pub fn execute(self) -> Result<http::Response, HostError> {
        let url = format!("{}/rest/v1/{}", self.client.url, self.url);

        // Create request with api_key
        let mut request = http::Request::new(url, self.method);

        // Insert headers
        for (header, value) in self.headers {
            request = request.set(header, value);
        }

        // Insert queries
        for (param, value) in self.queries {
            request = request.query(param, value);
        }

        // Call request
        match self.body {
            Some(body) => request.send_string(body),
            None => request.call(),
        }
    }
}
