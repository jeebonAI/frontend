# Djibon PACT

Personal Assistance and Communication Tool built with Dioxus and Rust.

## Project Structure

```
project/
├─ assets/           # Static assets (images, CSS, JS, etc.)
├─ src/              # Source code
│  ├─ main.rs        # Application entry point
│  ├─ components/    # Reusable UI components
│  ├─ routes/        # Application routes and pages
├─ Cargo.toml        # Project dependencies
├─ Dioxus.toml       # Dioxus configuration
```

## Development

### Prerequisites

1. Install Rust: https://www.rust-lang.org/tools/install
2. Install WebAssembly target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. Install Dioxus CLI:
   ```bash
   cargo install dioxus-cli@0.6.0 --locked
   ```

### Serving Your App

Run the following command to start developing with the default platform:

```bash
dx serve --platform web
```

To run for a different platform, use the `--platform platform` flag:
```bash
dx serve --platform desktop
```

### Building for Production

```bash
dx build --release
```

This will create optimized files in the `dist` directory.

## Deployment

### Deploying to Cloudflare Pages

#### Using GitHub Actions (Automated)

1. Add these secrets to your GitHub repository:
   - `CF_API_TOKEN`: Your Cloudflare API token
   - `CF_ACCOUNT_ID`: Your Cloudflare account ID

2. Create a Cloudflare Pages project (one-time setup):
   - Log in to Cloudflare dashboard
   - Go to Pages
   - Click "Create a project"
   - Choose "Direct Upload" (not GitHub)
   - Name your project "djibon-web"
   - Click "Create project"

3. Push to the `dev` branch to trigger automatic deployment.

#### Using CLI (Manual)

1. Install Wrangler:
   ```bash
   npm install -g wrangler
   ```

2. Authenticate with Cloudflare:
   ```bash
   wrangler login
   ```

3. Create a Cloudflare Pages project (first time only):
   ```bash
   wrangler pages project create djibon-web
   ```

4. Build your app:
   ```bash
   dx build --release
   ```

5. Deploy to Cloudflare Pages:
   ```bash
   wrangler pages publish dist --project-name=djibon-web
   ```

## Service Worker

The application includes a service worker (`assets/sw.js`) for offline capabilities and improved performance. It caches essential assets and provides a better user experience when the network is unreliable.

