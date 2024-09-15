FROM rust:1.81-alpine3.20 AS builder

# Can be release or debug
ARG ASSEMBLY_MODE=release
ENV ASSEMBLY_MODE=${ASSEMBLY_MODE}

WORKDIR /app

COPY . .

RUN if [ ${ASSEMBLY_MODE} = "release" ]; then \
    cargo build --release; \
    elif [ ${ASSEMBLY_MODE} = "debug" ]; then \
    cargo build; \
    else \
    echo "Invalid ASSEMBLY_MODE: ${ASSEMBLY_MODE}. Must be set to either 'release' or 'debug'"; \
    fi

FROM alpine:3.20 AS runtime

ARG ASSEMBLY_MODE
ENV ASSEMBLY_MODE=${ASSEMBLY_MODE}

WORKDIR /app
COPY --from=builder /app/target/${ASSEMBLY_MODE}/graph-rs /app

ENTRYPOINT [ "/app/graph-rs" ]
CMD [ "--help" ]