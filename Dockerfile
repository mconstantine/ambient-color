FROM node:24 AS begin

COPY . /app
WORKDIR /app
SHELL ["/bin/bash", "-c"]
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN mkdir -p ~/.local/share/bash-completion/completions && \
  rustup completions bash > ~/.local/share/bash-completion/completions/rustup

RUN rustup default stable
RUN cargo install wasm-pack

FROM begin AS dev

WORKDIR /app
SHELL ["/bin/bash", "-c"]

RUN curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim-linux-x86_64.tar.gz && \
  rm -rf /opt/nvim-linux-x86_64 && \
  tar -C /opt -xzf nvim-linux-x86_64.tar.gz && \
  rm nvim-linux-x86_64.tar.gz

ENV PATH="/opt/nvim-linux-x86_64/bin:${PATH}"

RUN curl -fsSL https://yazi-rs.github.io/builds/yazi-keyring.gpg | tee /usr/share/keyrings/yazi-keyring.gpg >/dev/null && \
  echo 'deb [signed-by=/usr/share/keyrings/yazi-keyring.gpg] https://yazi-rs.github.io/builds/ stable main' | tee /etc/apt/sources.list.d/yazi.list >/dev/null && \
  apt update && \
  apt install -y yazi

CMD ["bash", "-c", "while sleep 1000; do :; done"]
