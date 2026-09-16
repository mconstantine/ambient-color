FROM node:24

SHELL ["/bin/bash", "-c"]
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN mkdir -p ~/.local/share/bash-completion/completions && \
  rustup completions bash > ~/.local/share/bash-completion/completions/rustup

RUN rustup default stable
RUN cargo install wasm-pack
