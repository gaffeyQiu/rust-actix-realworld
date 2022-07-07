FROM rust:1.62

RUN mkdir /.cargo && echo '[source.crates-io]\n\
registry = "https://github.com/rust-lang/crates.io-index"\n\
replace-with = "tuna"\n\
[source.tuna]\n\
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"\n\
[net]\n\
git-fetch-with-cli = true' > /.cargo/config
RUN rustup component add rustfmt
RUN cargo install diesel_cli

CMD [ "sleep", "infinity"]