FROM postgres:latest
COPY ./vendors/init.sql /docker-entrypoint-initdb.d/
EXPOSE 5432