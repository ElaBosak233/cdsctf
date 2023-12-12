// Package docs Code generated by swaggo/swag. DO NOT EDIT
package docs

import "github.com/swaggo/swag"

const docTemplate = `{
    "schemes": {{ marshal .Schemes }},
    "swagger": "2.0",
    "info": {
        "description": "{{escape .Description}}",
        "title": "{{.Title}}",
        "contact": {},
        "version": "{{.Version}}"
    },
    "host": "{{.Host}}",
    "basePath": "{{.BasePath}}",
    "paths": {
        "/api/assets/games/cover/{id}": {
            "get": {
                "description": "通过比赛 Id 获取比赛封面",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "资源"
                ],
                "summary": "通过比赛 Id 获取比赛封面",
                "parameters": [
                    {
                        "type": "string",
                        "description": "比赛 Id",
                        "name": "id",
                        "in": "path",
                        "required": true
                    }
                ],
                "responses": {}
            },
            "post": {
                "description": "通过比赛 Id 设置比赛封面",
                "consumes": [
                    "multipart/form-data"
                ],
                "tags": [
                    "资源"
                ],
                "summary": "通过比赛 Id 设置比赛封面",
                "parameters": [
                    {
                        "type": "string",
                        "description": "比赛 Id",
                        "name": "id",
                        "in": "path",
                        "required": true
                    },
                    {
                        "type": "file",
                        "description": "封面文件",
                        "name": "avatar",
                        "in": "formData",
                        "required": true
                    }
                ],
                "responses": {}
            }
        },
        "/api/assets/games/writeups/{id}": {
            "get": {
                "description": "通过团队 Id 获取比赛 Writeup",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "资源"
                ],
                "summary": "通过团队 Id 获取比赛 Writeup",
                "parameters": [
                    {
                        "type": "string",
                        "description": "团队 Id",
                        "name": "id",
                        "in": "path",
                        "required": true
                    }
                ],
                "responses": {}
            }
        },
        "/api/assets/teams/avatar/{id}": {
            "get": {
                "description": "通过团队 Id 获取团队头像",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "资源"
                ],
                "summary": "通过团队 Id 获取团队头像",
                "parameters": [
                    {
                        "type": "string",
                        "description": "团队 Id",
                        "name": "id",
                        "in": "path",
                        "required": true
                    }
                ],
                "responses": {}
            },
            "post": {
                "description": "通过团队 Id 设置团队头像",
                "consumes": [
                    "multipart/form-data"
                ],
                "tags": [
                    "资源"
                ],
                "summary": "通过团队 Id 设置团队头像",
                "parameters": [
                    {
                        "type": "string",
                        "description": "团队 Id",
                        "name": "id",
                        "in": "path",
                        "required": true
                    },
                    {
                        "type": "file",
                        "description": "头像文件",
                        "name": "avatar",
                        "in": "formData",
                        "required": true
                    }
                ],
                "responses": {}
            }
        },
        "/api/assets/users/avatar/{id}": {
            "get": {
                "description": "通过用户 Id 获取用户头像",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "资源"
                ],
                "summary": "通过用户 Id 获取用户头像",
                "parameters": [
                    {
                        "type": "string",
                        "description": "用户 Id",
                        "name": "id",
                        "in": "path",
                        "required": true
                    }
                ],
                "responses": {}
            },
            "post": {
                "description": "通过用户 Id 设置用户头像",
                "consumes": [
                    "multipart/form-data"
                ],
                "tags": [
                    "资源"
                ],
                "summary": "通过用户 Id 设置用户头像",
                "parameters": [
                    {
                        "type": "string",
                        "description": "用户 Id",
                        "name": "id",
                        "in": "path",
                        "required": true
                    },
                    {
                        "type": "file",
                        "description": "头像文件",
                        "name": "avatar",
                        "in": "formData",
                        "required": true
                    }
                ],
                "responses": {}
            }
        },
        "/api/challenges": {
            "get": {
                "description": "题目全部查询",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "题目"
                ],
                "summary": "题目全部查询",
                "responses": {}
            },
            "put": {
                "description": "更新题目",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "题目"
                ],
                "summary": "更新题目",
                "parameters": [
                    {
                        "description": "ChallengeUpdateRequest",
                        "name": "data",
                        "in": "body",
                        "required": true,
                        "schema": {
                            "$ref": "#/definitions/request.ChallengeUpdateRequest"
                        }
                    }
                ],
                "responses": {}
            },
            "post": {
                "description": "创建题目",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "题目"
                ],
                "summary": "创建题目",
                "parameters": [
                    {
                        "description": "ChallengeCreateRequest",
                        "name": "data",
                        "in": "body",
                        "required": true,
                        "schema": {
                            "$ref": "#/definitions/request.ChallengeCreateRequest"
                        }
                    }
                ],
                "responses": {}
            },
            "delete": {
                "description": "删除题目",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "题目"
                ],
                "summary": "删除题目",
                "parameters": [
                    {
                        "description": "ChallengeDeleteRequest",
                        "name": "data",
                        "in": "body",
                        "required": true,
                        "schema": {
                            "$ref": "#/definitions/request.ChallengeDeleteRequest"
                        }
                    }
                ],
                "responses": {}
            }
        },
        "/api/challenges/{id}": {
            "get": {
                "description": "题目查询",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "题目"
                ],
                "summary": "题目查询",
                "parameters": [
                    {
                        "type": "string",
                        "description": "id",
                        "name": "id",
                        "in": "path",
                        "required": true
                    }
                ],
                "responses": {}
            }
        },
        "/api/configs": {
            "get": {
                "description": "配置全部查询",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "配置"
                ],
                "summary": "配置全部查询",
                "responses": {}
            },
            "put": {
                "description": "更新配置",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "配置"
                ],
                "summary": "更新配置",
                "responses": {}
            }
        },
        "/api/instances": {
            "get": {
                "description": "实例全部查询",
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "实例"
                ],
                "summary": "实例全部查询",
                "responses": {}
            },
            "post": {
                "description": "创建实例",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "实例"
                ],
                "summary": "创建实例",
                "parameters": [
                    {
                        "description": "InstanceCreateRequest",
                        "name": "input",
                        "in": "body",
                        "required": true,
                        "schema": {
                            "$ref": "#/definitions/request.InstanceCreateRequest"
                        }
                    }
                ],
                "responses": {}
            },
            "delete": {
                "description": "停止并删除容器",
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "实例"
                ],
                "summary": "停止并删除容器",
                "parameters": [
                    {
                        "type": "string",
                        "description": "InstanceId",
                        "name": "id",
                        "in": "query",
                        "required": true
                    }
                ],
                "responses": {}
            }
        },
        "/api/instances/renew": {
            "get": {
                "description": "容器续期",
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "实例"
                ],
                "summary": "容器续期",
                "parameters": [
                    {
                        "type": "string",
                        "description": "InstanceId",
                        "name": "id",
                        "in": "query",
                        "required": true
                    }
                ],
                "responses": {}
            }
        },
        "/api/instances/status": {
            "get": {
                "description": "获取实例状态",
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "实例"
                ],
                "summary": "获取实例状态",
                "parameters": [
                    {
                        "type": "string",
                        "description": "InstanceId",
                        "name": "id",
                        "in": "query",
                        "required": true
                    }
                ],
                "responses": {}
            }
        },
        "/api/instances/{id}": {
            "get": {
                "description": "实例查询",
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "实例"
                ],
                "summary": "实例查询",
                "parameters": [
                    {
                        "type": "string",
                        "description": "id",
                        "name": "id",
                        "in": "path",
                        "required": true
                    }
                ],
                "responses": {}
            }
        },
        "/api/teams": {
            "put": {
                "description": "更新团队",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "团队"
                ],
                "summary": "更新团队",
                "parameters": [
                    {
                        "description": "TeamUpdateRequest",
                        "name": "input",
                        "in": "body",
                        "required": true,
                        "schema": {
                            "$ref": "#/definitions/request.TeamUpdateRequest"
                        }
                    }
                ],
                "responses": {}
            },
            "post": {
                "description": "创建团队",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "团队"
                ],
                "summary": "创建团队",
                "parameters": [
                    {
                        "description": "TeamCreateRequest",
                        "name": "input",
                        "in": "body",
                        "required": true,
                        "schema": {
                            "$ref": "#/definitions/request.TeamCreateRequest"
                        }
                    }
                ],
                "responses": {}
            },
            "delete": {
                "description": "删除团队",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "团队"
                ],
                "summary": "删除团队",
                "parameters": [
                    {
                        "description": "TeamDeleteRequest",
                        "name": "input",
                        "in": "body",
                        "required": true,
                        "schema": {
                            "$ref": "#/definitions/request.TeamDeleteRequest"
                        }
                    }
                ],
                "responses": {}
            }
        },
        "/api/teams/": {
            "get": {
                "description": "查找所有团队",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "团队"
                ],
                "summary": "查找所有团队",
                "responses": {}
            }
        },
        "/api/teams/id/{id}": {
            "get": {
                "description": "查找团队",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "团队"
                ],
                "summary": "查找团队",
                "parameters": [
                    {
                        "type": "string",
                        "description": "id",
                        "name": "id",
                        "in": "path",
                        "required": true
                    }
                ],
                "responses": {}
            }
        },
        "/api/teams/members": {
            "post": {
                "description": "加入团队",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "团队"
                ],
                "summary": "加入团队",
                "parameters": [
                    {
                        "description": "TeamJoinRequest",
                        "name": "input",
                        "in": "body",
                        "required": true,
                        "schema": {
                            "$ref": "#/definitions/request.TeamJoinRequest"
                        }
                    }
                ],
                "responses": {}
            },
            "delete": {
                "description": "退出团队",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "团队"
                ],
                "summary": "退出团队",
                "parameters": [
                    {
                        "description": "TeamQuitRequest",
                        "name": "input",
                        "in": "body",
                        "required": true,
                        "schema": {
                            "$ref": "#/definitions/request.TeamQuitRequest"
                        }
                    }
                ],
                "responses": {}
            }
        },
        "/api/user/login": {
            "post": {
                "description": "用户登录",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "用户"
                ],
                "summary": "用户登录",
                "parameters": [
                    {
                        "description": "UserLoginRequest",
                        "name": "input",
                        "in": "body",
                        "required": true,
                        "schema": {
                            "$ref": "#/definitions/request.UserLoginRequest"
                        }
                    }
                ],
                "responses": {}
            }
        },
        "/api/user/logout": {
            "post": {
                "description": "用户登出",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "用户"
                ],
                "summary": "用户登出",
                "parameters": [
                    {
                        "description": "UserLogoutRequest",
                        "name": "input",
                        "in": "body",
                        "required": true,
                        "schema": {
                            "$ref": "#/definitions/request.UserLogoutRequest"
                        }
                    }
                ],
                "responses": {}
            }
        },
        "/api/user/register": {
            "post": {
                "description": "用户注册",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "用户"
                ],
                "summary": "用户注册",
                "parameters": [
                    {
                        "description": "UserRegisterRequest",
                        "name": "input",
                        "in": "body",
                        "required": true,
                        "schema": {
                            "$ref": "#/definitions/request.UserRegisterRequest"
                        }
                    }
                ],
                "responses": {}
            }
        },
        "/api/user/token/{token}": {
            "get": {
                "description": "Token 鉴定",
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "用户"
                ],
                "summary": "Token 鉴定",
                "parameters": [
                    {
                        "type": "string",
                        "description": "token",
                        "name": "token",
                        "in": "path",
                        "required": true
                    }
                ],
                "responses": {}
            }
        },
        "/api/users": {
            "get": {
                "description": "用户全部查询",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "用户"
                ],
                "summary": "用户全部查询",
                "responses": {}
            },
            "put": {
                "description": "用户更新（管理员）",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "用户"
                ],
                "summary": "用户更新 *",
                "parameters": [
                    {
                        "description": "UserUpdateRequest",
                        "name": "input",
                        "in": "body",
                        "required": true,
                        "schema": {
                            "$ref": "#/definitions/request.UserUpdateRequest"
                        }
                    }
                ],
                "responses": {}
            },
            "post": {
                "description": "用户创建（管理员）",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "用户"
                ],
                "summary": "用户创建 *",
                "parameters": [
                    {
                        "description": "UserCreateRequest",
                        "name": "input",
                        "in": "body",
                        "required": true,
                        "schema": {
                            "$ref": "#/definitions/request.UserCreateRequest"
                        }
                    }
                ],
                "responses": {}
            },
            "delete": {
                "description": "用户删除（管理员）",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "用户"
                ],
                "summary": "用户删除 *",
                "parameters": [
                    {
                        "description": "UserDeleteRequest",
                        "name": "input",
                        "in": "body",
                        "required": true,
                        "schema": {
                            "$ref": "#/definitions/request.UserDeleteRequest"
                        }
                    }
                ],
                "responses": {}
            }
        },
        "/api/users/id/{id}": {
            "get": {
                "description": "用户查询",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "用户"
                ],
                "summary": "用户查询（通过 Id）",
                "parameters": [
                    {
                        "type": "string",
                        "description": "id",
                        "name": "id",
                        "in": "path",
                        "required": true
                    }
                ],
                "responses": {}
            }
        },
        "/api/users/username/{username}": {
            "get": {
                "description": "用户查询",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "用户"
                ],
                "summary": "用户查询（通过 Username）",
                "parameters": [
                    {
                        "type": "string",
                        "description": "username",
                        "name": "username",
                        "in": "path",
                        "required": true
                    }
                ],
                "responses": {}
            }
        }
    },
    "definitions": {
        "request.ChallengeCreateRequest": {
            "type": "object",
            "properties": {
                "attachment_id": {
                    "type": "string"
                },
                "description": {
                    "type": "string"
                },
                "difficulty": {
                    "type": "integer"
                },
                "duration": {
                    "type": "integer"
                },
                "exposed_port": {
                    "type": "integer"
                },
                "flag": {
                    "type": "string"
                },
                "flag_env": {
                    "type": "string"
                },
                "image": {
                    "type": "string"
                },
                "is_dynamic": {
                    "type": "integer",
                    "enum": [
                        0,
                        1
                    ]
                },
                "memory_limit": {
                    "type": "integer"
                },
                "title": {
                    "type": "string"
                }
            }
        },
        "request.ChallengeDeleteRequest": {
            "type": "object",
            "required": [
                "id"
            ],
            "properties": {
                "id": {
                    "type": "string"
                }
            }
        },
        "request.ChallengeUpdateRequest": {
            "type": "object",
            "properties": {
                "attachment_id": {
                    "type": "string"
                },
                "description": {
                    "type": "string"
                },
                "difficulty": {
                    "type": "integer"
                },
                "duration": {
                    "type": "integer"
                },
                "exposed_port": {
                    "type": "integer"
                },
                "flag": {
                    "type": "string"
                },
                "flag_env": {
                    "type": "string"
                },
                "id": {
                    "type": "string"
                },
                "image": {
                    "type": "string"
                },
                "is_dynamic": {
                    "type": "integer",
                    "enum": [
                        0,
                        1
                    ]
                },
                "memory_limit": {
                    "type": "integer"
                },
                "title": {
                    "type": "string"
                }
            }
        },
        "request.InstanceCreateRequest": {
            "type": "object",
            "required": [
                "challenge_id"
            ],
            "properties": {
                "challenge_id": {
                    "type": "string"
                }
            }
        },
        "request.TeamCreateRequest": {
            "type": "object",
            "required": [
                "captain_id",
                "name"
            ],
            "properties": {
                "captain_id": {
                    "type": "string"
                },
                "name": {
                    "type": "string"
                }
            }
        },
        "request.TeamDeleteRequest": {
            "type": "object",
            "required": [
                "id"
            ],
            "properties": {
                "id": {
                    "type": "string"
                }
            }
        },
        "request.TeamJoinRequest": {
            "type": "object",
            "required": [
                "team_id",
                "user_id"
            ],
            "properties": {
                "team_id": {
                    "type": "string"
                },
                "user_id": {
                    "type": "string"
                }
            }
        },
        "request.TeamQuitRequest": {
            "type": "object",
            "required": [
                "team_id",
                "user_id"
            ],
            "properties": {
                "team_id": {
                    "type": "string"
                },
                "user_id": {
                    "type": "string"
                }
            }
        },
        "request.TeamUpdateRequest": {
            "type": "object",
            "required": [
                "captain_id",
                "id",
                "name"
            ],
            "properties": {
                "captain_id": {
                    "type": "string"
                },
                "id": {
                    "type": "string"
                },
                "name": {
                    "type": "string"
                }
            }
        },
        "request.UserCreateRequest": {
            "type": "object",
            "required": [
                "email",
                "password",
                "username"
            ],
            "properties": {
                "email": {
                    "type": "string"
                },
                "password": {
                    "type": "string"
                },
                "username": {
                    "type": "string"
                }
            }
        },
        "request.UserDeleteRequest": {
            "type": "object",
            "required": [
                "id"
            ],
            "properties": {
                "id": {
                    "type": "string"
                }
            }
        },
        "request.UserLoginRequest": {
            "type": "object",
            "required": [
                "password",
                "username"
            ],
            "properties": {
                "password": {
                    "type": "string"
                },
                "username": {
                    "type": "string"
                }
            }
        },
        "request.UserLogoutRequest": {
            "type": "object",
            "required": [
                "username"
            ],
            "properties": {
                "username": {
                    "type": "string"
                }
            }
        },
        "request.UserRegisterRequest": {
            "type": "object",
            "required": [
                "email",
                "password",
                "username"
            ],
            "properties": {
                "email": {
                    "type": "string"
                },
                "password": {
                    "type": "string"
                },
                "username": {
                    "type": "string"
                }
            }
        },
        "request.UserUpdateRequest": {
            "type": "object",
            "required": [
                "id",
                "password",
                "username"
            ],
            "properties": {
                "email": {
                    "type": "string"
                },
                "id": {
                    "type": "string"
                },
                "password": {
                    "type": "string",
                    "minLength": 6
                },
                "username": {
                    "type": "string",
                    "maxLength": 20,
                    "minLength": 3
                }
            }
        }
    }
}`

// SwaggerInfo holds exported Swagger Info so clients can modify it
var SwaggerInfo = &swag.Spec{
	Version:          "1.0",
	Host:             "",
	BasePath:         "",
	Schemes:          []string{},
	Title:            "PgsHub Backend API",
	Description:      "没有其他东西啦，仅仅是所有的后端接口，不要乱用哦",
	InfoInstanceName: "swagger",
	SwaggerTemplate:  docTemplate,
	LeftDelim:        "{{",
	RightDelim:       "}}",
}

func init() {
	swag.Register(SwaggerInfo.InstanceName(), SwaggerInfo)
}
