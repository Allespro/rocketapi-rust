use std::time::Duration;
use crate::api::RocketAPI;
use crate::errors::RocketAPIError;
use serde_json::{json, Value};

pub struct ThreadsAPI {
    pub api: RocketAPI,
    pub last_response: Value,
    pub counter: u32
}

impl ThreadsAPI {
    /*
    Threads API client.

    Args:
        token (String): Your RocketAPI token (https://rocketapi.io/dashboard/)
        max_timeout (std::time::Duration): Maximum timeout for requests. Please, don't use values lower than 15 seconds, it may cause problems with API.

    For debugging purposes you can use the following variables:
        last_response (serde_json::Value): contains the last response from the API.
        counter (u32): contains the number of requests made in the current session.

    For more information, see documentation: https://docs.rocketapi.io/api/
    */
    pub fn new(token: String, max_timeout: Duration) -> Self {
        ThreadsAPI {
            api: RocketAPI::new(token, max_timeout),
            last_response: Value::Null,
            counter: 0
        }
    }

    async fn request(&mut self, method: &str, data: Value) -> Result<Value, RocketAPIError> {
        match self.api.request(method, data).await {
            Ok(response) => {
                self.last_response = response.clone();
                self.counter += 1;
                if response["status"] == "done" {
                    let response_body = &response["response"];
                    let status_code = response_body["status_code"].as_i64().unwrap_or(0);
                    let content_type = response_body["content_type"].as_str().unwrap_or("");
        
                    if status_code == 200 && content_type == "application/json" {
                        Ok(response_body["body"].clone())
                    } else if status_code == 404 {
                        Err(RocketAPIError::NotFound(response))
                    } else {
                        Err(RocketAPIError::BadResponse(response))
                    }
                } else {
                    Err(RocketAPIError::BadResponse(response))
                }
            }
            Err(e) => {
                Err(RocketAPIError::RequestError(e))
            }
        }
    }
    
    pub async fn search_users(&mut self, query: &str, rank_token: Option<&str>, page_token: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Search for a specific user in Threads

        Args:
            query (str): Username to search for

        For more information, see documentation: https://docs.rocketapi.io/api/threads/search_users
        */
        let mut payload = json!({ "query": query });
        if let Some(rank) = rank_token {
            payload["rank_token"] = json!(rank);
        }
        if let Some(page) = page_token {
            payload["page_token"] = json!(page);
        }
        self.request("threads/search_users", payload).await
    }

    pub async fn get_user_info(&mut self, user_id: &u64) -> Result<Value, RocketAPIError> {
        /*
        Retrieve Threads user information by id.

        Args:
            user_id (u64): User id

        For more information, see documentation: https://docs.rocketapi.io/api/threads/user/get_info
        */
        let payload = json!({ "id": user_id });
        self.request("threads/user/get_info", payload).await
    }

    pub async fn get_user_feed(&mut self, user_id: &u64, max_id: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve Threads user feed by id.

        Args:
            user_id (u64): User id
            max_id (str): Use for pagination

        You can use the `max_id` parameter to paginate through the media (take from the `next_max_id` field of the response).

        For more information, see documentation: https://docs.rocketapi.io/api/threads/user/get_feed
        */
        let mut payload = json!({ "id": user_id });
        if let Some(max) = max_id {
            payload["max_id"] = json!(max);
        }
        self.request("threads/user/get_feed", payload).await
    }

    pub async fn get_user_replies(&mut self, user_id: &u64, max_id: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve Threads user replies by id.

        Args:
            user_id (u64): User id
            max_id (str): Use for pagination

        You can use the `max_id` parameter to paginate through the media (take from the `next_max_id` field of the response).

        For more information, see documentation: https://docs.rocketapi.io/api/threads/user/get_replies
        */
        let mut payload = json!({ "id": user_id });
        if let Some(max) = max_id {
            payload["max_id"] = json!(max);
        }
        self.request("threads/user/get_replies", payload).await
    }

    pub async fn get_user_followers(&mut self, user_id: &u64, max_id: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve Threads user followers by id.

        Args:
            user_id (u64): User id
            max_id (str): Use for pagination

        You can use the `max_id` parameter to paginate through followers (take from the `next_max_id` field of the response).

        For more information, see documentation: https://docs.rocketapi.io/api/threads/user/get_followers
        */
        let mut payload = json!({ "id": user_id });
        if let Some(max) = max_id {
            payload["max_id"] = json!(max);
        }
        self.request("threads/user/get_followers", payload).await
    }
    
    pub async fn search_user_followers(&mut self, user_id: &u64, query: &str) -> Result<Value, RocketAPIError> {
        /*
        Search Threads user followers by user id.

        Args:
            user_id (u64): User id
            query (str): Search query

        For more information, see documentation: https://docs.rocketapi.io/api/threads/user/get_followers
        */
        let payload = json!({
            "id": user_id,
            "query": query,
        });
        self.request("threads/user/get_followers", payload).await
    }

    pub async fn get_user_following(&mut self, user_id: &u64, max_id: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve Threads user following by id.

        Args:
            user_id (u64): User id
            max_id (str): Use for pagination

        You can use the `max_id` parameter to paginate through followers (take from the `next_max_id` field of the response).

        For more information, see documentation: https://docs.rocketapi.io/api/threads/user/get_following
        */
        let mut payload = json!({ "id": user_id });
        if let Some(max) = max_id {
            payload["max_id"] = json!(max);
        }
        self.request("threads/user/get_following", payload).await
    }

    pub async fn search_user_following(&mut self, user_id: &u64, query: &str) -> Result<Value, RocketAPIError> {
        /*
        Search Threads user following by user id.

        Args:
            user_id (u64): User id
            query (str): Search query

        For more information, see documentation: https://docs.rocketapi.io/api/threads/user/get_following
        */
        let payload = json!({
            "id": user_id,
            "query": query,
        });
        self.request("threads/user/get_following", payload).await
    }

    pub async fn get_thread_replies(&mut self, thread_id: &u64, max_id: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve thread replies by id.

        Args:
            thread_id (u64): Thread id
            max_id (str): Use for pagination

        You can use the `max_id` parameter to paginate through the media (take from the `paging_tokens["downwards"]` field of the response).

        For more information, see documentation: https://docs.rocketapi.io/api/threads/thread/get_replies
        */
        let mut payload = json!({ "id": thread_id });
        if let Some(max) = max_id {
            payload["max_id"] = json!(max);
        }
        self.request("threads/thread/get_replies", payload).await
    }

    pub async fn get_thread_likes(&mut self, thread_id: &u64) -> Result<Value, RocketAPIError> {
        /*
        Retrieve thread likes by id.

        Args:
            thread_id (u64): Thread id

        For more information, see documentation: https://docs.rocketapi.io/api/threads/thread/get_likes
        */
        let payload = json!({ "id": thread_id });
        self.request("threads/thread/get_likes", payload).await
    }
}
