const { Neo4jGraphQL } = require("@neo4j/graphql");
const { ApolloServer } = require("apollo-server");
const {ApolloServerPluginLandingPageGraphQLPlayground} = require("apollo-server-core")
const neo4j = require("neo4j-driver");

// (You may need to replace your connection details, username and password)
const AURA_ENDPOINT = 'neo4j+s://abaaaa0f.production-orch-0064.neo4j.io:7687';
const USERNAME = 'neo4j';
const PASSWORD = 'IBS_FgFoeia4BHO_q13dQ5PJmc14LNmePtrSJee5PNc';

// Create Neo4j driver instance
const driver = neo4j.driver(AURA_ENDPOINT, neo4j.auth.basic(USERNAME, PASSWORD) );

const typeDefs = `
  type User {
    username: String @unique(constraintName: "username_should_be_unique")
    password: String
    email: String @unique(constraintName: "one_email_could_only_bind_with_one_account")
    updatedRecords: [UpdatedRecord] @relationship(type: "Updated", direction: OUT)
    knownUsers: [User] @relationship(type: "Knows", direction: OUT)
    friendRequestList: [User] @cypher(statement: "match(users:User) where (users)-[:Knows]->(this) and not (users)<-[:Knows]-(this) return users")
    friends: [User] @cypher(statement: "match (users:User) where (this)-[:Knows]->(users)-[:Knows]->(this) return users")
    friendCounts: Int @cypher(statement:"return size((:User)-[:Knows]->(this))")
    updateHistorys: [UpdatedRecord] @relationship(type: "RecordsBy", direction: OUT)
  }
  """
  once one node
  if you update multiple nodes at same time
  you will get multiple UpdatedRecords
  """
  type UpdatedRecord {
    time: DateTime
    """
    will be a json object
    for example, if you changed one User
    set email from "" to "user@example.com"
    then you will get 
    {
      "User": {
        "email":["", "user@example.com"]
      }
    } from this field
    """
    detail: String
  }
  type Card {
    name: String
    description: String
    createdBy: User @relationship(type: "CreatedBy", direction: OUT)
  }
  type RaritySetting {
    title: String
    createdBy: User @relationship(type: "CreatedBy", direction: OUT)
    containRarities: [Rarity] @relationship(type: "Contains", direction: OUT)
    depth: Int @cypher(statement: "return size((:Rarity)<-[:Contains]-(this))")
  }
  type Rarity {
    name: String
    probability: Float
    sort: Int
  }
  enum CardPoolAlgorithm {
    """
    some other algorithms you like
    promise me, you only add algorithms you know how to implement here
    """
    Shuffle,
    RealTime
  }
  type CardPoolSetting {
    title: String
    createdBy: User @relationship(type: "CreatedBy", direction: OUT)
    algorithm: CardPoolAlgorithm!
    maxTimes: Int
  }
  type CardPool {
    currentCounts: Int
    cardSettingInUsing: CardPoolSetting @relationship(type: "Using", direction: IN)
    raritySettingInUsing: RaritySetting @relationship(type: "Using", direction: IN)
    cache: String
  }
`;

// Create instance that contains executable GraphQL schema from GraphQL type definitions
const neoSchema = new Neo4jGraphQL({
  typeDefs,
  driver
});

// Create ApolloServer instance to serve GraphQL schema
const server = new ApolloServer({
  schema: neoSchema.schema,
  context: (context) => {
    console.log(JSON.stringify(context.req.body))
  }
});

// Start ApolloServer
server.listen().then(({ url }) => {
  console.log(`GraphQL server ready at ${url}`);
});
