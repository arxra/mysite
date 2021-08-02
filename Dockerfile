FROM arm64v8/nginx
COPY nginx.conf /etc/nginx/nginx.conf
COPY dist /usr/share/nginx/html
