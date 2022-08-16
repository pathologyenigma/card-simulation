const {models} = require("./ogm.js");
const bcrypt = require("bcrypt");
const jwt = require("jsonwebtoken");
module.exports = (app) => {
    /**
     * @openapi
     * /users:
     *  get:
     *    responses:
     *      200:
     *        content:
     *         application/json:
     *           schema:
     *             $ref: "#/components/schemas/User"
     *        description: "search user using username or email"
     *      400:
     *         content:
     *           application/json:
     *             schema:
     *               type: object
     *               properties:
     *                 message:
     *                   type: string
     *         description: bad request response
     *      404:
     *        content:
     *          application/json:
     *            schema:
     *              type: object
     *              properties:
     *                message:
     *                  type: string
     *        description: no users found
     *        
     *    parameters:
     *        - name: search
     *          in: query
     *          required: true
     *          description: "the username or the email you want to search"
     *          schema:
     *            type: "string"
     *          example: "username"
     *        - name: offset
     *          in: query
     *          required: false
     *          description: "feel like page param"
     *          schema:
     *            type: "integer"
     *        - name: limit
     *          in: query
     *          required: false
     *          description: "how many results to return"
     *          schema:
     *            type: "integer"
     *        - name: sort
     *          in: query
     *          required: false
     *          description: "sort direction"
     *          schema:
     *            type: "array"
     *            items:
     *              type: "object"
     *          example: { username: "DESC"}
     */
    app.get("/users", async (req, res) => {
        const { search, offset, limit, sort } = req.query;
        if (search.trim().length === 0) {
            return res.status(400).json({ message: 'empty search is not allowed.' }).end();
        }
        const users = await models.User.find({
            where: { OR: [{
                username_CONTAINS: search,
            },{
                email_CONTAINS: search
            }] },
            options: {
                offset: offset,
                limit: limit,
                sort: sort ? JSON.parse(sort) : [{"username": "ASC"}],
            },
            selectionSet: `
            {
                id
                username
                email
            }`
        });
        if(users.length === 0) {
            return res.json({message: "No users found"}).end();
        }
        return res.json(users).end();
    });
    /** 
    * @openapi
    * /users/register:
    *   post:
    *     responses:
    *       200:
    *         content:
    *           application/json:
    *             schema:
    *               $ref: '#/components/schemas/User'
    *         description: Registration succeeded
    *       400:
    *         content:
    *           application/json:
    *             schema:
    *               type: object
     *               properties:
     *                 message:
     *                   type: string
     *     requestBody:
     *       content:
     *         application/json:
     *           schema:
     *             $ref: '#/components/schemas/UserRegister'
    */
    app.post("/users/register", async (req, res) => {
        const {username, email, password, confirmPassword} = req.body;
        if (!username || username.trim().length === 0) {
            return res.status(400).json({message: "username cannot be empty"})
        }
        if (!password || password.trim().length === 0) {
            return res.status(400).json({message: "password cannot be empty"})
        }
        if (password !== confirmPassword) {
            return res.status(400).json({message: "passwords do not match"})
        }
        const salt = bcrypt.genSaltSync(10)
        try {
            let {users} = await models.User.create({
                input: [{
                    username,
                    password: bcrypt.hashSync(password, salt),
                    email: email? email: null
                }]
            })
            return res.status(200).json({...users[0], password: undefined})
        } catch (err) {
            // console.error(err)
            if(err.message == "Constraint validation failed") {
                return res.status(400).json({message: "email or username is taken"})
            }
            return res.status(500).json({message: err.message})
        }
    })
    /** 
    * @openapi
    * /users/login:
    *   post:
    *     responses:
    *       200:
    *         content:
    *           application/json:
    *             schema:
    *               $ref: '#/components/schemas/Token'
    *         description: Registration succeeded
    *       400:
    *         content:
    *           application/json:
    *             schema:
    *               type: object
    *               properties:
    *                 message:
    *                   type: string
    *     requestBody:
    *       content:
    *         application/json:
    *           schema:
    *             $ref: '#/components/schemas/UserLogin'
    */
    app.post('/users/login', async (req, res) => {
        const {account, password} = req.body
        if (!account || account.trim().length === 0) {
            return res.status(400).json({message: "account cannot be empty"}).end();
        }
        if (!password || password.trim().length === 0) {
            return res.status(400).json({message: "password cannot be empty"}).end();
        }
        let [user] = await models.User.find({
            where: {
                OR: [{
                    username: account
                }, {
                    email: account
                }]
            }
        })
        if(!user) {
            return res.status(404).json({message: "user not found"}).end();
        }
        if(bcrypt.compareSync(password, user.password)) {
            return res.json({token:jwt.sign({
                ...user,
                password: undefined
            }, "secret")}).end()
        } else {
            return res.json({message: "incorrect password"}).end();
        }
    })
}