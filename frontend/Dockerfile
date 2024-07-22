FROM node:latest AS tailwind

# Install tailwindcss
RUN npm install -g tailwindcss


WORKDIR /code/frontend

# Copy the frontend code
COPY ./src ./src
COPY Cargo.toml .
COPY index.html .
COPY tailwind.config.js .


# Generate the css file
RUN tailwindcss -c tailwind.config.js -o tailwind.css

FROM rust:1.79 AS builder

# Install wasm and trunk
RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked trunk

WORKDIR /code/common

# Copy the common code
COPY ../common .



WORKDIR /code/frontend

COPY ./src ./src
COPY index.html .
COPY Cargo.toml .
COPY --from=tailwind /frontend/tailwind.css ./tailwind.css


# Build the frontend
RUN trunk build --release

FROM nginx:latest

COPY --from=builder /frontend/dist/ /usr/share/nginx/html/
COPY ./nginx.conf /etc/nginx/nginx.conf