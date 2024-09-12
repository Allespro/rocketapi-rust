use std::time::Duration;
use crate::api::RocketAPI;
use crate::errors::RocketAPIError;
use serde_json::{json, Value};

pub struct InstagramAPI {
    pub api: RocketAPI,
    pub last_response: Value,
    pub counter: u32
}

impl InstagramAPI {
    /*
    Instagram API client.
    
    Args:
        token (String): Your RocketAPI token (https://rocketapi.io/dashboard/)
        max_timeout (std::time::Duration): Maximum timeout for requests. Please, don't use values lower than 15 seconds, it may cause problems with API.
        
    For debugging purposes you can use the following variables:
        last_response (serde_json::Value): contains the last response from the API.
        counter (u32): contains the number of requests made in the current session.
        
    For more information, see documentation: https://docs.rocketapi.io/api/
    */
    pub fn new(token: String, max_timeout: Duration) -> Self {
        InstagramAPI {
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

    
    pub async fn search(&mut self, query: &str) -> Result<Value, RocketAPIError> {
        /*
        Search for a specific user, hashtag or place.
    
        Args:
            query (str): The search query
    
        For more information, see documentation: https://docs.rocketapi.io/api/instagram/search
        */
        let payload = json!({ "query": query });
        self.request("instagram/search", payload).await
    }

    
    pub async fn get_user_info(&mut self, username: &str) -> Result<Value, RocketAPIError> {
        /*
        Retrieve user information by username.
    
        Args:
            username (str): Username
    
        For more information, see documentation: https://docs.rocketapi.io/api/instagram/user/get_info
        */
        let payload = json!({ "username": username });
        self.request("instagram/user/get_info", payload).await
    }


    pub async fn get_user_info_by_id(&mut self, user_id: &u64) -> Result<Value, RocketAPIError> {
        /*
        Retrieve user information by id.
    
        Args:
            user_id (u64): User id
    
        For more information, see documentation: https://docs.rocketapi.io/api/instagram/user/get_info_by_id
        */
        let payload = json!({ "id": user_id });
        self.request("instagram/user/get_info_by_id", payload).await
    }


    pub async fn get_user_media(&mut self, user_id: &u64, count: Option<u8>, max_id: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve user media by id.
    
        Args:
            user_id (u64): User id
            count (u8): Number of media to retrieve (max: 50)
            max_id (str): Use for pagination
    
        You can use the `max_id` parameter to paginate through the media (take from the `next_max_id` field of the response).
    
        For more information, see documentation: https://docs.rocketapi.io/api/instagram/user/get_media
        */
        let mut payload = json!({ "id": user_id, "count": count.unwrap_or(12) });
        if let Some(max_id) = max_id {
            payload["max_id"] = json!(max_id);
        }
        self.request("instagram/user/get_media", payload).await
    }

    
    pub async fn get_user_clips(&mut self, user_id: &u64, count: Option<u8>, max_id: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve user clips (videos from "Reels" section) by id.
    
        Args:
            user_id (u64): User id
            count (u8): Number of media to retrieve (max: 50)
            max_id (str): Use for pagination
    
        You can use the `max_id` parameter to paginate through the media (take from the `max_id` (!) field of the response).
    
        For more information, see documentation: https://docs.rocketapi.io/api/instagram/user/get_clips
        */
        let mut payload = json!({ "id": user_id, "count": count.unwrap_or(12) });
        if let Some(max_id) = max_id {
            payload["max_id"] = json!(max_id);
        }
        self.request("instagram/user/get_clips", payload).await
    }

    
    pub async fn get_user_guides(&mut self, user_id: &u64, max_id: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve user guides by id.

        Args:
            user_id (u64): User id
            max_id (str): Use for pagination

        You can use the `max_id` parameter to paginate through the media (take from the `next_max_id` field of the response).

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/user/get_guides
        */
        let mut payload = json!({ "id": user_id });
        if let Some(max_id) = max_id {
            payload["max_id"] = json!(max_id);
        }
        self.request("instagram/user/get_guides", payload).await
    }

    
    pub async fn get_user_tags(&mut self, user_id: &u64, count: Option<u8>, max_id: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve user tags by id.

        Args:
            user_id (u64): User id
            count (u8): Number of media to retrieve (max: 50)
            max_id (str): Use for pagination

        You can use the `max_id` parameter to paginate through the media (take from the `end_cursor` (!) field of the response).

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/user/get_tags
        */
        let mut payload = json!({ "id": user_id, "count": count.unwrap_or(12) });
        if let Some(max_id) = max_id {
            payload["max_id"] = json!(max_id);
        }
        self.request("instagram/user/get_tags", payload).await
    }

    pub async fn get_user_following(&mut self, user_id: &u64, count: Option<u16>, max_id: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve user following by user id.

        Args:
            user_id (u64): User id
            count (u16): Number of users to return (max: 200)
            max_id (str): Use for pagination

        You can use the `max_id` parameter to paginate through following (take from the `next_max_id` field of the response).

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/user/get_following
        */
        let mut payload = json!({ "id": user_id, "count": count.unwrap_or(12) });
        if let Some(max_id) = max_id {
            payload["max_id"] = json!(max_id);
        }
        self.request("instagram/user/get_following", payload).await
    }
    
    pub async fn search_user_following(&mut self, user_id: &u64, query: &str) -> Result<Value, RocketAPIError> {
        /*
        Search user following by user id.

        Args:
            user_id (u64): User id
            query (str): Search query

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/user/get_following
        */
        let payload = json!({ "id": user_id, "query": query });
        self.request("instagram/user/get_following", payload).await
    }

    pub async fn get_user_followers(&mut self, user_id: &u64, count: Option<u8>, max_id: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve user followers by user id.

        Args:
            user_id (u64): User id
            count (u8): Number of users to return (max: 100)
            max_id (str): Use for pagination

        You can use the `max_id` parameter to paginate through followers (take from the `next_max_id` field of the response).

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/user/get_followers
        */
        let mut payload = json!({ "id": user_id, "count": count.unwrap_or(12) });
        if let Some(max_id) = max_id {
            payload["max_id"] = json!(max_id);
        }
        self.request("instagram/user/get_followers", payload).await
    }
        
    pub async fn search_user_followers(&mut self, user_id: &u64, query: &str) -> Result<Value, RocketAPIError> {
        /*
        Search user followers by user id.

        Args:
            user_id (u64): User id
            query (str): Search query

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/user/get_followers
        */
        let payload = json!({ "id": user_id, "query": query });
        self.request("instagram/user/get_followers", payload).await
    }
        
    pub async fn get_user_stories_bulk(&mut self, user_ids: Vec<&u64>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve user(s) stories by user id(s).
        You can retrieve up to 4 user ids per request.

        Args:
            user_ids (list): List of user ids

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/user/get_stories
        */
        let payload = json!({
            "ids": user_ids
        });
        self.request("instagram/user/get_stories", payload).await
    }
    
    pub async fn get_user_stories(&mut self, user_id: &u64) -> Result<Value, RocketAPIError> {
        /*
        Retrieve user stories by user id.

        Args:
            user_id (u64): User id

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/user/get_stories
        */
        self.get_user_stories_bulk(vec![user_id]).await
    }
    
    pub async fn get_user_highlights(&mut self, user_id: &u64) -> Result<Value, RocketAPIError> {
        /*
        Retrieve user highlights by user id.

        Args:
            user_id (u64): User id

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/user/get_highlights
        */
        let payload = json!({ "id": user_id });
        self.request("instagram/user/get_highlights", payload).await
    }
    
    pub async fn get_user_live(&mut self, user_id: &u64) -> Result<Value, RocketAPIError> {
        /*
        Retrieve user live broadcast by id.

        Args:
            user_id (u64): User id

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/user/get_live
        */
        let payload = json!({ "id": user_id });
        self.request("instagram/user/get_live", payload).await
    }
    
    pub async fn get_user_similar_accounts(&mut self, user_id: &u64) -> Result<Value, RocketAPIError> {
        /*
        Lookup for user similar accounts by id. Typically, up to 80 accounts will be returned.

        Args:
            user_id (u64): User id

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/user/get_similar_accounts
        */
        let payload = json!({ "id": user_id });
        self.request("instagram/user/get_similar_accounts", payload).await
    }
    
    pub async fn get_media_info(&mut self, media_id: &u64) -> Result<Value, RocketAPIError> {
        /*
        Retrieve media information by media id.

        Args:
            media_id (u64): Media id

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/media/get_info
        */
        let payload = json!({ "id": media_id });
        self.request("instagram/media/get_info", payload).await
    }
    
    pub async fn get_media_info_by_shortcode(&mut self, shortcode: &str) -> Result<Value, RocketAPIError> {
        /*
        Retrieve media information by media shortcode. This method provides the same information as the `get_media_info`.

        Args:
            shortcode (str): Media shortcode

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/media/get_info_by_shortcode
        */
        let payload = json!({ "shortcode": shortcode });
        self.request("instagram/media/get_info_by_shortcode", payload).await
    }
    
    pub async fn get_media_likes(&mut self, shortcode: &str, count: Option<u8>, max_id: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve media likes by media shortcode.

        Args:
            shortcode (str): Media shortcode
            count (u8): Number of likers to return (max: 50)
            max_id (str): Use for pagination

        You can use the `max_id` parameter to paginate through likers (take from the `next_max_id` field of the response).

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/media/get_likes
        */
        let mut payload = json!({ "shortcode": shortcode, "count": count.unwrap_or(12) });
        if let Some(max_id) = max_id {
            payload["max_id"] = json!(max_id);
        }
        self.request("instagram/media/get_likes", payload).await
    }
    
    pub async fn get_media_comments(&mut self, media_id: &u64, can_support_threading: Option<bool>, min_id: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve media comments by media id.

        Args:
            media_id (u64): Media id
            can_support_threading (bool): Set `False` if you want chronological order
            min_id (str): Use for pagination

        You can use the `min_id` parameter to paginate through comments (take from the `next_min_id` field of the response).

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/media/get_comments
        */
        let mut payload = json!({ "media_id": media_id, "can_support_threading": can_support_threading.unwrap_or(true) });
        if let Some(min_id) = min_id {
            payload["min_id"] = json!(min_id);
        }
        self.request("instagram/media/get_comments", payload).await
    }
    
    pub async fn get_media_shortcode_by_id(&mut self, media_id: &u64) -> Result<Value, RocketAPIError> {
        /*
        Get media shortcode by media id. This endpoint is provided free of charge.

        Args:
            media_id (u64): Media id

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/media/get_shortcode_by_id
        */
        let payload = json!({ "id": media_id });
        self.request("instagram/media/get_shortcode_by_id", payload).await
    }
    
    pub async fn get_media_id_by_shortcode(&mut self, shortcode: &str) -> Result<Value, RocketAPIError> {
        /*
        Get media id by media shortcode. This endpoint is provided free of charge.

        Args:
            shortcode (str): Media shortcode

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/media/get_id_by_shortcode
        */
        let payload = json!({ "shortcode": shortcode });
        self.request("instagram/media/get_id_by_shortcode", payload).await
    }
    
    pub async fn get_guide_info(&mut self, guide_id: &u64) -> Result<Value, RocketAPIError> {
        /*
        Retrieve guide information by guide id.

        Args:
            guide_id (u64): Guide id

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/guide/get_info
        */
        let payload = json!({ "id": guide_id });
        self.request("instagram/guide/get_info", payload).await
    }
    
    pub async fn get_location_info(&mut self, location_id: &u64) -> Result<Value, RocketAPIError> {
        /*
        Retrieve location information by location id.

        Args:
            location_id (u64): Location id

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/location/get_info
        */
        let payload = json!({ "id": location_id });
        self.request("instagram/location/get_info", payload).await
    }
    
    pub async fn get_location_media(&mut self, location_id: &u64, page: Option<&u64>, max_id: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve location media by location id.

        Args:
            location_id (u64): Location id
            page (u64): Page number
            max_id (str): Use for pagination

        In order to use pagination, you need to use both the `max_id` and `page` parameters. You can obtain these values from the response's `next_page` and `next_max_id` fields.

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/location/get_media
        */
        let mut payload = json!({ "id": location_id });
        if let Some(page) = page {
            payload["page"] = json!(page);
        }
        if let Some(max_id) = max_id {
            payload["max_id"] = json!(max_id);
        }
        self.request("instagram/location/get_media", payload).await
    }
    
    pub async fn get_hashtag_info(&mut self, name: &str) -> Result<Value, RocketAPIError> {
        /*
        Retrieve hashtag information by hashtag name.

        Args:
            name (str): Hashtag name

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/hashtag/get_info
        */
        let payload = json!({ "name": name });
        self.request("instagram/hashtag/get_info", payload).await
    }
    
    pub async fn get_hashtag_media(&mut self, name: &str, page: Option<&u64>, max_id: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve hashtag media by hashtag name.

        Args:
            name (str): Hashtag name
            page (u64): Page number
            max_id (str): Use for pagination

        In order to use pagination, you need to use both the `max_id` and `page` parameters. You can obtain these values from the response's `next_page` and `next_max_id` fields.

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/hashtag/get_media
        */
        let mut payload = json!({ "name": name });
        if let Some(page) = page {
            payload["page"] = json!(page);
        }
        if let Some(max_id) = max_id {
            payload["max_id"] = json!(max_id);
        }
        self.request("instagram/hashtag/get_media", payload).await
    }
    
    pub async fn get_highlight_stories_bulk(&mut self, highlight_ids: Vec<&u64>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve highlight(s) stories by highlight id(s).

        Args:
            highlight_ids (Vec<u64>): Highlight id(s)

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/highlight/get_stories
        */
        let payload = json!({ "ids": highlight_ids });
        self.request("instagram/highlight/get_stories", payload).await
    }
    
    pub async fn get_highlight_stories(&mut self, highlight_id: &u64) -> Result<Value, RocketAPIError> {
        /*
        Retrieve highlight stories by highlight id.

        Args:
            highlight_id (u64): Highlight id

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/highlight/get_stories
        */
        self.get_highlight_stories_bulk(vec![highlight_id]).await
    }
    
    pub async fn get_comment_likes(&mut self, comment_id: &u64, max_id: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve comment likes by comment id.

        Args:
            comment_id (u64): Comment id
            max_id (str): Use for pagination

        You can use the `max_id` parameter to paginate through likes (take from the `next_max_id` field of the response).

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/comment/get_likes
        */
        let mut payload = json!({ "id": comment_id });
        if let Some(max_id) = max_id {
            payload["max_id"] = json!(max_id);
        }
        self.request("instagram/comment/get_likes", payload).await
    }
    
    pub async fn get_comment_replies(&mut self, comment_id: &u64, media_id: &u64, max_id: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve comment replies by comment id and media id.

        Args:
            comment_id (u64): Comment id
            media_id (u64): Media id
            max_id (str): Use for pagination

        You can use the `max_id` parameter to paginate through replies (take from the `next_max_child_cursor` field of the response).

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/comment/get_replies
        */
        let mut payload = json!({ "id": comment_id, "media_id": media_id });
        if let Some(max_id) = max_id {
            payload["max_id"] = json!(max_id);
        }
        self.request("instagram/comment/get_replies", payload).await
    }
    
    pub async fn get_audio_media(&mut self, audio_id: &u64, max_id: Option<&str>) -> Result<Value, RocketAPIError> {
        /*
        Retrieve audio media by audio id.

        Args:
            audio_id (u64): Audio id
            max_id (str): Use for pagination

        You can use the `max_id` parameter to paginate through media (take from the `next_max_id` field of the response).

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/audio/get_media
        */
        let mut payload = json!({ "id": audio_id });
        if let Some(max_id) = max_id {
            payload["max_id"] = json!(max_id);
        }
        self.request("instagram/audio/get_media", payload).await
    }
    
    pub async fn get_user_about(&mut self, user_id: &u64) -> Result<Value, RocketAPIError> {
        /*
        Obtain user details from «About this Account» section.

        ⭐️ This method is exclusively available to our Enterprise+ clients.
        If you wish to enable it for your account, please get in touch with our support team: https://t.me/rocketapi

        Args:
            user_id (u64): User id

        For more information, see documentation: https://docs.rocketapi.io/api/instagram/user/get_about
        */
        let payload = json!({ "id": user_id });
        self.request("instagram/user/get_about", payload).await
    }
    
}
