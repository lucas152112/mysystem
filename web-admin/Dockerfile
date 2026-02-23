# Build stage
FROM node:18-alpine as builder

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci

# Copy source code
COPY . .

# Build the application
RUN npm run build

# Runtime stage
FROM node:18-alpine

WORKDIR /app

# Create non-root user
RUN addgroup -S mysystem && adduser -S mysystem -G mysystem

# Copy built application and node_modules
COPY --from=builder /app/.output /app/.output
COPY --from=builder /app/node_modules /app/node_modules

# Set permissions
RUN chown -R mysystem:mysystem /app

USER mysystem

# Expose port
EXPOSE 3000

# Environment variables
ENV NUXT_PUBLIC_API_BASE_URL="http://backend:3000/api"
ENV NUXT_PUBLIC_WEBSOCKET_URL="ws://backend:3000/api/ws"
ENV NODE_ENV=production

# Start the application
CMD ["node", ".output/server/index.mjs"]