FROM alpine:latest AS builder
ARG TARGETARCH
WORKDIR /builder
COPY . .
RUN ./scripts/binary.sh $TARGETARCH

FROM scratch
COPY --from=builder --chmod=755 /builder/polipo polipo

ENV RUST_LOG=info
ENV OCTOPUS_EMAIL=your-email@example.com
ENV OCTOPUS_PASSWORD=your-password

ENV SMTP_SERVER=smtp.gmail.com
ENV SMTP_PORT=587
ENV SMTP_USERNAME=your-smtp-username@gmail.com
ENV SMTP_PASSWORD=your-app-password

ENV CRON_SCHEDULE="0 9 * * *"


ENV CACHE_FILE_PATH=./cache.json

CMD ["polipo"]