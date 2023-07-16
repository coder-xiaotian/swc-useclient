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

## 常见问题

+ swc 插件打断了tree shaking

目前这是一个意料之中的行为，[next团队会在未来改进它](https://github.com/vercel/next.js/issues/52679#issuecomment-1636807256)。可以通过配置modularizeImports暂时解决这个问题：
```javascript
const nextConfig = {
  modularizeImports: {
    "@mui/material": {
      transform: "@mui/material/{{member}}"
    },
  },
  experimental: {
    swcPlugins: [
      [
        "use-client",
        {
          include: ["@mui/material"],
        },
      ],
    ],
  },
};
```
注意有些函数是不遵循`@mui/material/{{member}}`规则的，比如：createTheme、ThemeProvider，需要单独处理成这样`import { createTheme, ThemeProvider } from "@mui/material/styles`

🌟🌟🌟🌟如果这个插件对您有用的话，就给作者一个star吧。🙏🙏🙏🌟🌟🌟🌟
