FROM alpine
COPY splash-server/target/release/run_server /home/run_server
COPY splash-server/target/release/create_user /home/create_user
COPY splash-web/dist /home/public

WORKDIR /home
CMD ['/home/run_server']