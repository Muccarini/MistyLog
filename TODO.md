# GameReview — TODO

## Tech Stack (Decided)
- **Frontend:** Leptos 0.7 (WASM SPA via Trunk) + Tailwind CSS + leptos-shadcn-ui
- **Backend:** Rust + Actix Web 4 + SeaORM
- **Database:** PostgreSQL
- **Auth:** Zitadel (self-hosted) OIDC PKCE flow via backend, opaque session cookie to frontend
- **Design:** Letterboxd-inspired (dark theme, poster grids, clean review cards)
- **External API:** RAWG (game data)
- **Deployment:** Docker (multi-stage build + docker-compose)

## 🚀 Project Setup
- [x] Choose tech stack
- [ ] Initialise repository (git init, .gitignore, README.md)
- [ ] Set up project structure (Cargo workspace: backend + frontend)
- [ ] Configure dependencies (Cargo.toml workspace)
- [ ] Set up linter & formatter (clippy + rustfmt)
- [ ] Set up environment variables (.env.example)

## 🗄️ Database & Models
- [ ] Design data schema (Users, Games, Reviews)
- [ ] Set up PostgreSQL (docker-compose service)
- [ ] Create SeaORM migrations (users, games, reviews tables)
- [ ] Generate / write SeaORM entities

## 🔐 Authentication (Zitadel PKCE + Session)
- [ ] Zitadel self-hosted setup (docker-compose service)
- [ ] OIDC PKCE auth code flow in backend (openidconnect crate)
- [ ] Login redirect (GET /api/auth/login → Zitadel authorize)
- [ ] Callback handler (GET /api/auth/callback → exchange code, set session)
- [ ] Current user endpoint (GET /api/auth/me) — reads session
- [ ] Logout endpoint (POST /api/auth/logout) — clears session + Zitadel end_session
- [ ] Auto-provision local users from Zitadel claims (sub, email, name)
- [ ] AuthUser extractor middleware (session-based)
- [ ] Opaque httpOnly session cookie (actix-session)

## 🎮 Game Features
- [ ] Create game (POST /api/games) — admin/auth
- [ ] List games (GET /api/games) with search, genre & platform filters
- [ ] Get game detail (GET /api/games/:id)
- [ ] Update game (PUT /api/games/:id)
- [ ] Delete game (DELETE /api/games/:id)
- [ ] RAWG API integration service for auto-populating game data

## ⭐ Review Features
- [ ] Create review (POST /api/games/:id/reviews) — auth required
- [ ] Edit own review (PUT /api/reviews/:id) — auth required
- [ ] Delete own review (DELETE /api/reviews/:id) — auth required
- [ ] List reviews per game (GET /api/games/:id/reviews)
- [ ] Average rating calculation per game

## 🖥️ Frontend / UI (Leptos SPA)
- [ ] App shell + router setup
- [ ] Navbar component
- [ ] Homepage with featured / recent games
- [ ] Game listing page with filters
- [ ] Game detail + reviews page
- [ ] User profile page
- [ ] Login / register forms
- [ ] Responsive design (Tailwind mobile-first)

## 🧪 Testing
- [ ] Unit tests for core business logic
- [ ] Integration tests for API endpoints

## 📦 Deployment
- [ ] Dockerfile (multi-stage: build + runtime)
- [ ] docker-compose.yml (app + postgres)
- [ ] Configure production environment variables

## 📝 Documentation
- [ ] Write README.md (setup instructions, features, architecture)
- [ ] API endpoint documentation
