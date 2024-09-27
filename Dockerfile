FROM scratch

ENV PORT 8080
ENV FINANCE_PORT $PORT

WORKDIR /usr/bin

COPY target/release/finance ./finance

CMD ["finance_service"]
