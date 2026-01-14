# AGENTS.md - AI Agent Context for Telnyx Web Client

## Project Overview

**Project Name:** Telnyx Web Client
**Purpose:** Webapp to send and receive messages from Telnyx
**Architecture:** Full-stack monorepo with Rust backend and React frontend

## Tech Stack

### Backend (Rust)
- **Language:** Rust (Edition 2024)
- **Web Framework:** Actix-web 4.x with actix-inertia 0.1.0
- **Async Runtime:** Tokio 1.49.0
- **Security:** actix-web with rustls and secure-cookies
- **CORS:** actix-cors 0.7.1
- **Rate Limiting:** actix-governor 0.10.0
- **File Handling:** actix-multipart 0.7.2, actix-files 0.6.9, actix-embed 0.1.0
- **Serialization:** serde 1.0.228, serde_json
- **Logging:** tracing 0.1.43 with color-eyre
- **Config:** dotenvy 0.15.7

### Frontend (React + TypeScript)
- **Framework:** React 19.2.0 + TypeScript 5.9.3
- **Build Tool:** Vite (rolldown-vite 7.2.5)
- **Navigation:** Inertia.js 2.3.6
- **UI Library:** shadcn/ui (New York style, neutral base)
- **State Management:** React State + Inertia.js
- **Styling:** Tailwind CSS v4.1.18
- **Forms:** react-hook-form 7.71.1 + zod 4.3.5
- **Icons:** lucide-react 0.562.0

## Project Structure

```
.
├── Cargo.toml              # Rust project config
├── justfile                # Task runner commands
├── .env / .env.example     # Environment variables
├── src/                    # Rust source
│   ├── main.rs             # Actix-web server entry point
│   ├── server.rs           # Web service creation
│   ├── config.rs           # Configuration management
│   └── types.rs           # Type definitions
├── web/                    # Frontend Vite project
│   ├── package.json        # Frontend dependencies
│   ├── components.json     # shadcn/ui config
│   ├── vite.config.ts      # Vite configuration
│   ├── tsconfig.json      # TypeScript config
│   └── src/
│       ├── Pages/          # Inertia.js pages
│       │   ├── App.tsx    # Dashboard page
│       │   ├── Login.tsx   # Login page
│       │   ├── Signup.tsx  # Admin signup (one-time)
│       │   ├── ForgotPassword.tsx
│       │   └── ResetPassword.tsx
│       ├── components/
│       │   └── ui/        # shadcn/ui components
│       ├── hooks/         # Custom React hooks
│       ├── lib/           # Utility functions
│       ├── App.css        # Global styles
│       ├── index.css      # Tailwind CSS imports
│       └── main.tsx      # Inertia.js app initialization
└── target/               # Rust build output
```

## Environment Variables

### Backend (.env)
- `MODE`: `development` or `production` (default: development)
- `VITE_ORIGIN`: Vite dev server URL (default: `http://localhost:5173`)
- `VITE_ENTRY`: Vite entry point (default: `/src/main.ts`)
- Server runs on port 8080

### Frontend (web/)
- Proxy configured in vite.config.ts:
  - `/inertia` → `http://127.0.0.1:8080`
  - `/inertia-version` → `http://127.0.0.1:8080`

## Development Commands

### Using just (recommended)
```bash
just run           # Run both Vite dev server and Rust server concurrently
```

### Manual commands
```bash
# Backend
cargo run                # Run Rust server
cargo build --release    # Build production binary

# Frontend
cd web
npm run dev             # Start Vite dev server
npm run build           # Build for production (tsc -b && vite build)
npm run lint            # Run ESLint
```

## Backend Patterns (Rust)

### Server Setup
- Uses Actix-web with 5 workers
- Binds to config address (default: 127.0.0.1:8080)
- Graceful shutdown on SIGINT/SIGTERM
- Inertia.js integration via actix-inertia

### Configuration
- Loaded from environment variables using WebConfig
- Supports dotenv for .env file loading

### Logging
- Uses tracing crate with structured logging
- color-eyre for better error reporting
- Filter: `info,<crate_name>=debug,tokio=info`

## Frontend Patterns (React)

### Page Creation Pattern
Pages are React components in `web/src/Pages/`. Inertia.js automatically resolves and renders them.

```tsx
// web/src/Pages/YourPage.tsx
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";

function YourPage() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Your Page</CardTitle>
      </CardHeader>
      <CardContent>
        {/* Page content */}
      </CardContent>
    </Card>
  );
}

export default YourPage;
```

### Available Pages
- **App.tsx** - Dashboard page with card grid
- **Login.tsx** - Login with email/password, forgot password link
- **Signup.tsx** - One-time admin signup (hidden after created)
- **ForgotPassword.tsx** - Request password reset link
- **ResetPassword.tsx** - Reset password with new/confirm fields

### Form Validation Pattern
```tsx
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import * as z from "zod";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";

const formSchema = z.object({
  email: z.string().email("Invalid email"),
  password: z.string().min(8, "Password too short"),
});

function YourForm() {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: { email: "", password: "" },
  });

  function onSubmit(values: z.infer<typeof formSchema>) {
    console.log(values);
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)}>
        <FormField
          control={form.control}
          name="email"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Email</FormLabel>
              <FormControl>
                <Input type="email" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
      </form>
    </Form>
  );
}
```

### Inertia.js Configuration (main.tsx)
- Page resolution: `import.meta.glob("./Pages/**/*.tsx", { eager: true })`
- Form default recently successful duration: 5000ms
- Prefetch cache: 1 minute
- Prefetch hover delay: 150ms
- Custom headers: `X-Custom-Header: value`

### shadcn/ui Components
Located in `web/src/components/ui/`:
- accordion, alert, alert-dialog, avatar, badge, button, button-group, calendar
- card, checkbox, collapsible, command, context-menu, dialog, drawer, dropdown-menu
- form, hover-card, input, input-group, item, kbd, label, menubar, navigation-menu
- popover, progress, radio-group, resizable, scroll-area, select, separator, sheet
- sidebar, skeleton, slider, sonner, spinner, switch, table, tabs, textarea, toggle
- toggle-group, tooltip, empty

### Path Aliases
- `@/components` → `src/components`
- `@/lib/utils` → `src/lib/utils`
- `@/components/ui` → `src/components/ui`
- `@/lib` → `src/lib`
- `@/hooks` → `src/hooks`

## Build & Deployment

### Development
```bash
just run
# Visit http://localhost:8080
```

### Production Build
```bash
npm run build --prefix ./web    # Build frontend
cargo build --release           # Build Rust binary
mkdir -p output
mv web/dist output/
mv target/release/server output/
# Final: output/dist/ and output/server
```

### Production Run
```bash
./output/server
```

## Key Considerations for AI Agents

### Backend (Rust) Work:
1. Files are in `src/` directory
2. Uses Actix-web framework with async/await
3. Inertia.js integration for server-side rendering
4. Follows Rust naming conventions (snake_case)
5. Uses Result<T, eyre::Error> for error handling
6. Logging with tracing macros (info!, debug!, error!)

### Frontend (React) Work:
1. Files are in `web/src/` directory
2. Pages go in `web/src/Pages/`
3. Use shadcn/ui components from `@/components/ui/*`
4. Form validation: react-hook-form + zod
5. Icons: lucide-react
6. Follow "new-york" style and neutral color scheme
7. No `as any` or `@ts-ignore` - maintain type safety

### When Modifying UI:
1. VISUAL changes (styling, layout, animation) → Delegate to frontend-ui-ux-engineer agent
2. LOGIC changes (state, API calls, business logic) → Can handle directly

### Backend Integration:
- Frontend communicates via Inertia.js, not direct fetch
- API proxy: `/inertia` routes to backend at `http://127.0.0.1:8080`
- Use Inertia.js visit method for programmatic navigation

## Common Imports Reference

### Backend (Rust)
```rust
// Actix-web
use actix_web::{HttpServer, web, App, HttpResponse, Responder};
use actix_cors::Cors;

// Inertia
use actix_inertia::Inertia;

// Tracing
use tracing::{info, debug, error};

// Error handling
use color_eyre::eyre;
```

### Frontend (React)
```tsx
// Core React
import { useState, useEffect } from "react";

// shadcn/ui Components
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from "@/components/ui/form";

// Form Handling
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import * as z from "zod";

// Icons
import { User, Lock, Mail, Eye, EyeOff, ShieldCheck } from "lucide-react";

// Utilities
import { cn } from "@/lib/utils";
```

## Testing & Verification

Before completing tasks:
1. Backend: Run `cargo check` or `cargo build`
2. Frontend: Run `npm run build` in web/ directory
3. Check TypeScript compilation succeeds
4. Verify no type errors or linting issues

## Notes

- **Monorepo architecture**: Both backend and frontend in one repository
- **Inertia.js**: Provides SPA-like experience without client-side routing complexity
- **Actix-web**: High-performance Rust web framework
- **shadcn/ui**: Full suite of accessible, customizable React components
- **Modern stack**: Rust 2024 edition, React 19, TypeScript 5.9, Tailwind CSS v4
- **Strict typing**: Both Rust and TypeScript with strict mode enabled
- **Security**: TLS support, secure cookies, CORS configuration, rate limiting
