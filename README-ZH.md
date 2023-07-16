# use-client
[英文](https://github.com/coder-xiaotian/swc-useclient)

一个将React组件库自动转换为“React客户端组件“的 swc 插件，比如：你可以通过它自动将@mui中的组件自动转换为"React客户端组件"，而不用在外部包装一个使用了"use client"的组件。

## 安装
npm: 
```
npm install -D use-client
```

yarn:
```
yarn add -D use-client
```

pnpm:
```
pnpm i -D use-client
```

## 配置
+ include：["ui-library-path"]

## 案例
next.js配置：
```js
const nextConfig = {
  experimental: {
    swcPlugins: [
      [
        'use-client',
        {
          include: ["@mui/material", "@mui/icons-material"，"antd", "@ant-design/icons"]
        }
      ],
    ]
  }
}
```

swc配置：
```js
{
  "$schema": "https://json.schemastore.org/swcrc",
  "jsc": {
    "experimental": {
      "plugins": [
        ["use-client", {
          "include": ["@mui/material", "@mui/icons-material", "antd", "@ant-design/icons"]
        }]
      ]
    }
  }
}
```
