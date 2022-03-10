pub struct LogIn;
pub mod log_in {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "LogIn";
    pub const QUERY : & str = "query LogIn($account: String) {\n    users(where: {OR: [{username: $account},{email: $account}]}) {\n        friendCounts\n        friendRequestList {\n            username\n        }\n        friends {\n            friendCounts\n            email\n            username\n        }\n        email\n        password\n        username\n    }\n}" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        pub account: Option<String>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Vec<LogInUsers>,
    }
    #[derive(Deserialize, Debug)]
    pub struct LogInUsers {
        #[serde(rename = "friendCounts")]
        pub friend_counts: Option<Int>,
        #[serde(rename = "friendRequestList")]
        pub friend_request_list: Option<Vec<Option<LogInUsersFriendRequestList>>>,
        pub friends: Option<Vec<Option<LogInUsersFriends>>>,
        pub email: Option<String>,
        pub password: Option<String>,
        pub username: Option<String>,
    }
    #[derive(Deserialize, Debug)]
    pub struct LogInUsersFriendRequestList {
        pub username: Option<String>,
    }
    #[derive(Deserialize, Debug)]
    pub struct LogInUsersFriends {
        #[serde(rename = "friendCounts")]
        pub friend_counts: Option<Int>,
        pub email: Option<String>,
        pub username: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for LogIn {
    type Variables = log_in::Variables;
    type ResponseData = log_in::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: log_in::QUERY,
            operation_name: log_in::OPERATION_NAME,
        }
    }
}
