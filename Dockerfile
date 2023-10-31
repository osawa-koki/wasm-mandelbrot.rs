FROM rust:1.73 as wasm-builder
WORKDIR /src/
RUN cargo install cargo-generate wasm-pack
COPY ./ ./
RUN wasm-pack build --release --out-dir ./pkg/

FROM node:18 as web-builder
WORKDIR /src/www/
COPY ./www/package.json ./www/yarn.lock ./
COPY --from=wasm-builder /src/pkg/ ../pkg/
RUN yarn install --frozen-lockfile --non-interactive --production
COPY ./www/ ./
RUN yarn build

FROM nginx:1.25 as web-server
EXPOSE 80
COPY --from=web-builder /src/www/dist/ /usr/share/nginx/html/
