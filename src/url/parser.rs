use std::collections::HashMap;

#[allow(dead_code)]
pub struct Url {
    pub path: String,
    pub query: Option<HashMap<String, String>>,
}

impl Url {
    pub fn new(url: &str) -> Url {
        let parts: Vec<&str> = url.splitn(2, '?').collect();
        let path = parts[0].to_string();
        let query = if parts.len() > 1 {
            Some(Self::parse_query(parts[1]))
        } else {
            None
        };

        Url {
            path,
            query,
        }
    }

    fn parse_query(query: &str) -> HashMap<String, String> {
        query
            .split('&')
            .filter_map(|s| {
                let mut pair = s.splitn(2, '=');
                let key = pair.next()?;
                let value = pair.next()?;

                Some((key.to_string(), value.to_string()))
            })
            .collect()
    }

    pub fn match_path(&self, pattern: &str) -> Option<HashMap<String, String>> {
        let url_segments = self.path.split('/').filter(|s| !s.is_empty()).collect::<Vec<&str>>();
        let pattern_segments = pattern.split('/').filter(|s| !s.is_empty()).collect::<Vec<&str>>();

        if url_segments.len() != pattern_segments.len() {
            return None;
        }

        let mut params = HashMap::new();

        for (url_seg, pattern_seg) in url_segments.iter().zip(pattern_segments.iter()) {
            if pattern_seg.starts_with(':') {
                let param_name = &pattern_seg[1..];
                params.insert(param_name.to_string(), url_seg.to_string());
            } else if url_seg != pattern_seg {
                return None;
            }
        }

        Some(params)
    }
}