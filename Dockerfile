FROM nginx:stable
COPY nginx.conf /etc/nginx/nginx.conf
COPY dist /usr/share/nginx/html
