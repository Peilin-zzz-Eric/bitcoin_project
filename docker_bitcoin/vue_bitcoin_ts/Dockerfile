# 1. 使用Node.js作为基础镜像来编译Vue项目
FROM node:16 as build-stage

# 2. 设置工作目录
WORKDIR /app

# 3. 复制项目文件到容器
COPY package*.json ./
COPY . .

# 4. 安装依赖并构建项目
RUN npm install
RUN npm run build

# 5. 使用nginx作为基础镜像来托管静态文件
FROM nginx:alpine as production-stage
COPY --from=build-stage /app/dist /usr/share/nginx/html

# 6. 暴露端口
EXPOSE 80

# 7. 启动nginx
CMD ["nginx", "-g", "daemon off;"]
