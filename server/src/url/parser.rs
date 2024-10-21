use std::collections::HashMap;

#[derive(PartialEq, Debug)]
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

                if key.len() > 0 {
                    return Some((key.to_string(), value.to_string()));
                }

                None
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_url_new_without_query() {
        let url = Url::new("/home");
        assert_eq!(url.path, "/home");
        assert!(url.query.is_none());
    }

    #[test]
    fn test_url_new_with_multiple_queries() {
        let url = Url::new("/search?q=rust&sort=asc&limit=10");
        assert_eq!(url.path, "/search");
        let mut expected_query = HashMap::new();
        expected_query.insert("q".to_string(), "rust".to_string());
        expected_query.insert("sort".to_string(), "asc".to_string());
        expected_query.insert("limit".to_string(), "10".to_string());
        assert_eq!(url.query, Some(expected_query));
    }

    #[test]
    fn test_url_new_with_empty_query() {
        let url = Url::new("/path?");
        assert_eq!(url.path, "/path");
        let expected_query = Some(HashMap::new());
        assert_eq!(url.query, expected_query);
    }

    #[test]
    fn test_url_new_with_invalid_query() {
        let url = Url::new("/path?key1=value1&key2");
        assert_eq!(url.path, "/path");
        let mut expected_query = HashMap::new();
        expected_query.insert("key1".to_string(), "value1".to_string());
        assert_eq!(url.query, Some(expected_query));
    }

    #[test]
    fn test_parse_query_empty() {
        let query = "";
        let parsed = Url::parse_query(query);
        let expected: HashMap<String, String> = HashMap::new();
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_query_multiple_params() {
        let query = "key1=value1&key2=value2&key3=value3";
        let mut expected = HashMap::new();
        expected.insert("key1".to_string(), "value1".to_string());
        expected.insert("key2".to_string(), "value2".to_string());
        expected.insert("key3".to_string(), "value3".to_string());
        let parsed = Url::parse_query(query);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_query_with_missing_value() {
        let query = "key1=value1&key2=&key3=value3";
        let mut expected = HashMap::new();
        expected.insert("key1".to_string(), "value1".to_string());
        expected.insert("key2".to_string(), "".to_string());
        expected.insert("key3".to_string(), "value3".to_string());
        let parsed = Url::parse_query(query);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_query_with_missing_key() {
        let query = "=value1&key2=value2";
        let mut expected = HashMap::new();
        expected.insert("key2".to_string(), "value2".to_string());
        assert_eq!(Url::parse_query(query), expected);
    }

    #[test]
    fn test_match_path_exact_match() {
        let url = Url::new("/home");
        let pattern = "/home";
        let result = url.match_path(pattern);
        assert_eq!(result, Some(HashMap::new()));
    }

    #[test]
    fn test_match_path_with_multiple_parameters() {
        let url = Url::new("/user/123/profile/456");
        let pattern = "/user/:user_id/profile/:profile_id";
        let mut expected = HashMap::new();
        expected.insert("user_id".to_string(), "123".to_string());
        expected.insert("profile_id".to_string(), "456".to_string());
        let result = url.match_path(pattern);
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_match_path_non_matching_pattern() {
        let url = Url::new("/user/123");
        let pattern = "/admin/:id";
        let result = url.match_path(pattern);
        assert_eq!(result, None);
    }

    #[test]
    fn test_match_path_different_segment_count() {
        let url = Url::new("/user/123");
        let pattern = "/user/:id/details";
        let result = url.match_path(pattern);
        assert_eq!(result, None);
    }

    #[test]
    fn test_match_path_with_no_parameters() {
        let url = Url::new("/about");
        let pattern = "/about";
        let result = url.match_path(pattern);
        assert_eq!(result, Some(HashMap::new()));
    }
}