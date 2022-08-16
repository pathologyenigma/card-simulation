const { Neo4jGraphQL } = require("@neo4j/graphql");
const { ogm, neoSchema } = require("./ogm.js");
// const { createJWT, comparePassword } = require("./utils"); // example util functions
const express = require("express");
const { version } = require("./package.json");
const swaggerJSDoc = require("swagger-jsdoc");
const swaggerUi = require("swagger-ui-express");
const routes = require("./routes.js");

const options = {
    definition: {
        openapi: "3.0.0",
        info: {
            title: "card-server",
            version,
        },
        components: {
            securitySchemes: {
                bearerAuth: {
                    type: "http",
                    scheme: "bearer",
                    bearerFormat: "JWT"
                }
            },
            schemas: {
                User: {
                    type: "object",
                    required: ["id", "username"],
                    properties: {
                        id: {
                            type: "string",
                            format: "uuid",
                            example: "d290f1ee-6c54-4b01-90e6-d701748f0851"
                        },
                        username: {
                            type: "string",
                            example: "enigma"
                        },
                        email: {
                            type: "string",
                            format: "email",
                            example: "enigma@example.com"
                        }
                    }
                },
                UserRegister: {
                    type: "object",
                    required: ["username", "password", "confirmPassword"],
                    properties: {
                        username: {
                            type: "string",
                            example: "enigma"
                        },
                        email: {
                            type: "string",
                            format: "email",
                            example: "enigma@example.com",
                        },
                        password: {
                            type: "string",
                            format: "password",
                            example: "password"
                        },
                        confirmPassword: {
                            type: "string",
                            format: "password",
                            example: "password"
                        }
                    }
                },
                UserLogin: {
                    type: "object",
                    required: ["account", "password"],
                    properties: {
                        account: {
                            type: "string",
                            example: "enigma@example.com",
                            description: "username or email address"
                        },
                        password: {
                            type: "string",
                            format: "password",
                            example: "password",
                        }
                    }
                },
                Token: {
                    type: "object",
                    required: ["token"],
                    properties: {
                        token: {
                            type: "string",
                            format: "JWT",
                        }
                    }
                }
            }
        },
        security: [
            {
                bearerAuth: []
            }
        ]
    },
    apis: ["./routes.js"]
}
const swaggerSpec = swaggerJSDoc(options)
const app = express();
app.use(express.json())

app.use('/docs', swaggerUi.serve, swaggerUi.setup(swaggerSpec))

app.get("/docs.json", async (req, res) => {
    res.setHeader('Content-Type', 'application/json')
    res.send(swaggerSpec)
})
routes(app);
const port = 4000;

ogm.init().then(() => {
    
    neoSchema.getSchema()
    neoSchema.assertIndexesAndConstraints({ options: { create: true }});

    app.listen(port, () => {
        console.log(`Example app listening at http://localhost:${port}`)
    });
});

