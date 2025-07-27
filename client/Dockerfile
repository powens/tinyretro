# Build stage
FROM node:22-alpine AS builder

# Install pnpm
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

WORKDIR /app

# Copy package files first for better layer caching
COPY package.json pnpm-lock.yaml ./

# Install dependencies
RUN pnpm install --frozen-lockfile

# Copy source code
COPY . .

# Build the SvelteKit app
RUN pnpm run build

# Production stage
FROM nginx:alpine

# Copy custom nginx config
COPY ./default.conf /etc/nginx/conf.d/default.conf

# Remove default nginx content
RUN rm -rf /usr/share/nginx/html/*

# Copy built SvelteKit app from builder stage
COPY --from=builder /app/build /usr/share/nginx/html

# Expose port 80
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]
