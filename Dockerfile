# Use a more recent Ubuntu version that includes libwebkit2gtk-4.1-dev
FROM ubuntu:24.04

# Install the necessary build tools and dependencies for a Tauri app
RUN apt-get update && apt-get install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libgtk-3-dev \
  x11-apps

# Install Rust (needed for Tauri)
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Node.js (needed for the frontend part of the Tauri app)
RUN curl -sL https://deb.nodesource.com/setup_18.x | bash -
RUN apt-get install -y nodejs

# Set the working directory in the container
WORKDIR /app

# Copy the Tauri project files into the container
COPY . .

# Install Node.js dependencies
RUN npm install

# Build the Tauri project
# RUN npm run tauri dev

# Expose the port where your app will be served (if applicable)
# EXPOSE 1420
ENV DISPLAY=:0

# Command to run the app (adjust based on your app's setup)
CMD ["npm", "run", "tauri", "dev"]
