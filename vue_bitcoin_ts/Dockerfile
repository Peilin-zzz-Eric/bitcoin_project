# 1. 使用 Node.js 镜像作为构建阶段
FROM node:16.20.1 AS build-stage

# 2. 设置工作目录
WORKDIR /app

# 3. 复制依赖声明文件到镜像
# 只复制 package.json 和 package-lock.json，避免不必要的文件影响缓存
COPY package*.json ./

# 4. 安装依赖
# 使用 npm ci 确保依赖安装与 package-lock.json 一致，并提高安装速度
RUN npm ci

# 5. 复制项目文件到镜像
# 确保只复制项目代码，不包含本地的 node_modules 或其他不必要的文件
COPY . .

# 6. 构建静态文件
RUN npm run build

# 7. 使用 Nginx 作为生产镜像的基础
FROM nginx:alpine AS production-stage

# 8. 复制构建生成的静态文件到 Nginx 默认目录
COPY --from=build-stage /app/dist /usr/share/nginx/html

# 9. 暴露端口
EXPOSE 80

# 10. 启动 Nginx
CMD ["nginx", "-g", "daemon off;"]
