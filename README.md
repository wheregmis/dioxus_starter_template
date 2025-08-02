# Dioxus Starter Template

A modern full-stack web application built with [Dioxus](https://dioxuslabs.com/), featuring a modular architecture with separate frontend, backend, API, and UI components.

## 🏗️ Project Architecture

This project uses a modular workspace structure with four main components:

```
dioxus_starter_template/
├── api/           # Shared API definitions and server functions
├── backend/       # Server implementation and session management
├── frontend/      # Client-side application entry point
├── ui/           # Shared UI components and routing
└── target/       # Build artifacts
```

## 📁 Folder Structure

### `api/` - Shared API Layer
- **Purpose**: Contains server functions and shared types used by both frontend and backend
- **Key Files**:
  - `src/server_fns/counter.rs` - Server function for counter operations
  - `src/backend_ext/req_parts_counter.rs` - Request extraction logic for sessions
- **Features**: 
  - Server functions with session management
  - Request/response type definitions
  - Shared between frontend and backend

### `backend/` - Server Implementation
- **Purpose**: Handles HTTP server, session management, and serves the Dioxus application
- **Key Files**:
  - `src/main.rs` - Server entry point with session configuration
- **Features**:
  - Axum-based HTTP server
  - Tower sessions for state management
  - Serves the Dioxus application

### `frontend/` - Client Application
- **Purpose**: Entry point for the client-side application
- **Key Files**:
  - `src/main.rs` - Launches the UI application
  - `Dioxus.toml` - Dioxus configuration
- **Features**:
  - Web, desktop, and mobile platform support
  - Hot reloading during development

### `ui/` - Shared UI Components
- **Purpose**: Contains the main application UI and routing logic
- **Key Files**:
  - `src/lib.rs` - Main App component and routing
  - `assets/` - Static assets (CSS, icons, images)
- **Features**:
  - Router-based navigation
  - Server-side rendering support
  - Shared across all platforms

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ 
- Dioxus CLI: `cargo install dioxus-cli`

### Development

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd dioxus_starter_template
   ```

2. **Run the development server**:
   ```bash
   just serve
   ```
   This command runs both the frontend and backend with hot reloading.

3. **Alternative: Run individual components**:
   ```bash
   # Frontend only
   dx serve --package frontend --platform web
   
   # Backend only
   cargo run --package backend
   ```

### Building for Production

```bash
# Build for web
dx build --package frontend --platform web

# Build for desktop
dx build --package frontend --platform desktop

# Build for mobile
dx build --package frontend --platform mobile
```

## 🔧 Key Features

### Session Management
- Uses Tower Sessions for server-side state
- Counter increments automatically on each request
- Session expiry after 10 seconds of inactivity

### Hot Reloading
- Frontend and backend hot reloading during development
- Automatic recompilation on file changes

### Cross-Platform Support
- **Web**: Standard web application
- **Desktop**: Native desktop application
- **Mobile**: Mobile application (iOS/Android)

### Modular Architecture
- **API Layer**: Shared server functions and types
- **Backend**: HTTP server and session management
- **Frontend**: Client entry point
- **UI**: Shared components and routing

## 🛠️ Development Workflow

1. **UI Changes**: Edit files in `ui/src/` for component changes
2. **API Changes**: Modify `api/src/server_fns/` for server functions
3. **Backend Changes**: Update `backend/src/` for server logic
4. **Frontend Changes**: Modify `frontend/src/` for client-specific logic

## 📦 Dependencies

### Core Dependencies
- **Dioxus**: 0.7.0-alpha.3 - UI framework
- **Axum**: 0.8.4 - HTTP server
- **Tokio**: 1.47.0 - Async runtime
- **Tower Sessions**: 0.14.0 - Session management
- **Serde**: 1.0 - Serialization

### Development Tools
- **Just**: Task runner for development commands
- **Dioxus CLI**: Build and development tools

## 🎯 Current Implementation

The application currently features:
- A simple counter that increments on each server request
- Session-based state management
- Hot reloading development environment
- Modular architecture ready for expansion

## 🔮 Future Enhancements

Potential areas for expansion:
- Database integration
- Authentication system
- More complex UI components
- API rate limiting
- Error handling improvements
- Testing infrastructure

## 📝 License

This project is licensed under the MIT License.
