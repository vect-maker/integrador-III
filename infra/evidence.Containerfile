
FROM node:24-slim

RUN apt-get update && apt-get install -y \
    python3 \
    make \
    g++ \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app



EXPOSE 3000

CMD ["sh", "-c", "npm run dev -- --host 0.0.0.0"]
