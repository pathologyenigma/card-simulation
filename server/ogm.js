const { OGM } = require("@neo4j/graphql-ogm");
const neo4j = require("neo4j-driver");
const {Neo4jGraphQL} = require("@neo4j/graphql");
const driver = neo4j.driver(
    "bolt://127.0.0.1:7687",
    neo4j.auth.basic("neo4j", "IBS_FgFoeia4BHO_q13dQ5PJmc14LNmePtrSJee5PNc")
);
const typeDefs = `
    type User @node{
        id: ID @id
        username: String! @unique
        password: String! @private
        email: String @unique
        updatedRecords: [UpdatedRecord!]! @relationship(type: "Updated", direction: OUT)
        knownUsers: [User!]! @relationship(type: "Knows", direction: OUT)
        friendRequestList: [User!]! @cypher(statement: "match(users:User) where (users)-[:Knows]->(this) and not (users)<-[:Knows]-(this) return users")
        friends: [User!]! @cypher(statement: "match (users:User) where (this)-[:Knows]->(users)-[:Knows]->(this) return users")
        friendCounts: Int @cypher(statement:"return size((:User)-[:Knows]->(this))")
        updateHistorys: [UpdatedRecord!]! @relationship(type: "RecordsBy", direction: OUT)
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
      containRarities: [Rarity!]! @relationship(type: "Contains", direction: OUT)
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
    type Mutation {
        signUp(username: String!, password: String!): String! ### JWT
        signIn(username: String!, password: String!): String! ### JWT
        addCard(name: String!, description: String!): Int! ### JWT
    }
`;

const ogm = new OGM({ typeDefs, driver });
const User = ogm.model("User");
const Card = ogm.model("Card");
module.exports = {
    ogm,
    neoSchema: new Neo4jGraphQL({ typeDefs, driver }),
    models: {User, Card},
}