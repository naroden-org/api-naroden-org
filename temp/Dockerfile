
#Dockerfile
FROM surrealdb/surrealdb:v2.0.4-dev

ADD host.cert /usr/local/share/ca-certificates/cacert.crt
USER root
RUN apk update
RUN apk upgrade --available
RUN apk add ca-certificates
RUN update-ca-certificates




# sudo docker build -t naroden/db-naroden-org .
# sudo docker run --rm --pull always -p 8000:8000 --user admin --pass Plemena_2024
# sudo docker run --rm --pull always -p 8000:8000 --user admin --pass Plemena_2024  -v $(pwd)/naroden:/naroden naroden/db-naroden-org start rocksdb:/naroden/api-naroden-org.db

# sudo docker run --rm --pull always -p 8000:8000 surrealdb/surrealdb:latest start --user admin --pass Plemena_2024 rocksdb://api-naroden-org.db
#mkdir naroden # Create a directory to store the database, owned by the current user
#docker run --rm -p 8000:8000 --user $(id -u) -v $(pwd)/naroden:/naroden naroden/db-naroden-org start --user admin --pass Plemena_2024 rocksdb:/naroden/api-naroden-org.db