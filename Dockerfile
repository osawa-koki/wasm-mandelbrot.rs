FROM rust:1.73
WORKDIR /src/

RUN cargo install cargo-generate wasm-pack

RUN apt update && \
    apt install -y ca-certificates curl gnupg && \
    mkdir -p /etc/apt/keyrings && \
    curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg
RUN echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_18.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list
RUN apt update && apt install nodejs -y && npm install --global yarn

COPY ./ ./

RUN wasm-pack build --release

WORKDIR /src/www/
RUN yarn install
RUN yarn build

# `/src/www/dist/` is the directory that contains the static files that will be served by the server.
