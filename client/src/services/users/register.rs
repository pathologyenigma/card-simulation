pub struct Resgister;
pub mod resgister {
    #![allow(dead_code)]

    pub const OPERATION_NAME: &str = "Resgister";
    pub const QUERY : & str = "mutation Resgister($username: String, $password: String, $email: String) {\r\n  createUsers(input: [{\r\n    username: $username,\r\n    password: $password,\r\n    email: $email\r\n  }]) {\r\n    info {\r\n      bookmark\r\n      nodesCreated\r\n      relationshipsCreated\r\n    }\r\n    users {\r\n      username\r\n      password\r\n      email\r\n      friendCounts\r\n    }\r\n  }\r\n}" ;

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
        pub username: Option<String>,
        pub password: Option<String>,
        pub email: Option<String>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "createUsers")]
        pub create_users: ResgisterCreateUsers,
    }
    #[derive(Deserialize)]
    pub struct ResgisterCreateUsers {
        pub info: ResgisterCreateUsersInfo,
        pub users: Vec<ResgisterCreateUsersUsers>,
    }
    #[derive(Deserialize)]
    pub struct ResgisterCreateUsersInfo {
        pub bookmark: Option<String>,
        #[serde(rename = "nodesCreated")]
        pub nodes_created: Int,
        #[serde(rename = "relationshipsCreated")]
        pub relationships_created: Int,
    }
    #[derive(Deserialize)]
    pub struct ResgisterCreateUsersUsers {
        pub username: Option<String>,
        pub password: Option<String>,
        pub email: Option<String>,
        #[serde(rename = "friendCounts")]
        pub friend_counts: Option<Int>,
    }
}
impl graphql_client::GraphQLQuery for Resgister {
    type Variables = resgister::Variables;
    type ResponseData = resgister::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: resgister::QUERY,
            operation_name: resgister::OPERATION_NAME,
        }
    }
}
