# Plain Word - English Vocabulary Notebook

A beautifully crafted vocabulary notebook application designed to make learning English words feel natural and intuitive, just like writing in a paper notebook.

## Tech Stack

- **Backend**: Axum (Rust)
- **Frontend**: Vue 3 + TypeScript + TailwindCSS
- **Database**: SQLite (SeaORM)
- **Authentication**: Session-based (tower-sessions)

## Features

- User registration, login, and logout
- Multiple wordbooks per user
- Chapters within wordbooks for organization
- Words with English, Chinese, and optional phonetic notation
- Global tag system for word categorization
- Three display modes: Original, Translation, Bilingual
- Tag filtering and shuffle functionality
- Organic design style with smooth animations

## Project Structure

```
plain_word/
  Cargo.toml          # Workspace configuration
  entity/             # SeaORM entities (auto-generated)
  migration/          # Database migrations
  server/             # Axum backend
    src/
      auth/           # Authentication (session, password)
      handlers/       # API handlers
      main.rs         # Server entry point
      routes.rs       # Route definitions
      static_files.rs # Embedded static files
  web/                # Vue 3 frontend
    src/
      api/            # API client
      components/     # Vue components
      router/         # Vue Router
      stores/         # Pinia stores
      types/          # TypeScript types
      views/          # Page views
```

## Quick Start

### Prerequisites

- Rust (latest stable)
- Node.js (v18+)
- sea-orm-cli (`cargo install sea-orm-cli`)

### Development

1. Clone the repository

2. Initialize the database:
```bash
cargo run -p migration
```

3. Generate entities (if needed):
```bash
sea-orm-cli generate entity -o entity/src --lib
```

4. Build the frontend:
```bash
cd web
npm install
npm run build
```

5. Run the server:
```bash
cargo run -p server
```

6. Visit http://127.0.0.1:3000

### Environment Variables

Create a `.env` file in the project root:

```
DATABASE_URL=sqlite:./plain_word.db?mode=rwc
```

## API Endpoints

### Authentication
- `POST /api/auth/register` - Register new user
- `POST /api/auth/login` - User login
- `POST /api/auth/logout` - User logout
- `GET /api/auth/me` - Get current user

### Tags
- `GET /api/tags` - List all tags
- `POST /api/tags` - Create tag
- `PUT /api/tags/:id` - Update tag
- `DELETE /api/tags/:id` - Delete tag

### Wordbooks
- `GET /api/wordbooks` - List wordbooks
- `POST /api/wordbooks` - Create wordbook
- `GET /api/wordbooks/:id` - Get wordbook
- `PUT /api/wordbooks/:id` - Update wordbook
- `DELETE /api/wordbooks/:id` - Delete wordbook

### Chapters
- `GET /api/wordbooks/:id/chapters` - List chapters
- `POST /api/wordbooks/:id/chapters` - Create chapter
- `PUT /api/wordbooks/:wid/chapters/:cid` - Update chapter
- `DELETE /api/wordbooks/:wid/chapters/:cid` - Delete chapter

### Words
- `GET /api/chapters/:id/words` - List words in chapter
- `POST /api/chapters/:id/words` - Create word
- `PUT /api/words/:id` - Update word
- `DELETE /api/words/:id` - Delete word
- `GET /api/words/:id/tags` - Get word tags
- `POST /api/words/:wid/tags/:tid` - Add tag to word
- `DELETE /api/words/:wid/tags/:tid` - Remove tag from word

## Design Style

The application follows an organic design philosophy:
- Soft, flowing shapes with multi-radius corners
- Gentle gradients with natural colors (Raw Walnut, Washed Linen, Iron Hardware, Natural Oak)
- Smooth animations with 6-10s blob morphing
- Paper-like notebook aesthetic
- Frosted glass effects with backdrop blur

## License

MIT