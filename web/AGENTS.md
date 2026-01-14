# AGENTS.md - AI Agent Context for Telnyx Web Client

## Project Overview

**Project Name:** Telnyx Web Client (web-react)
**Framework:** React 19.2.0 + TypeScript
**Build Tool:** Vite (rolldown-vite 7.2.5)
**UI Library:** shadcn/ui (New York style, neutral base)
**State Management:** Inertia.js + React State
**Styling:** Tailwind CSS v4.1.18

## Tech Stack & Dependencies

### Core Framework
- **React:** 19.2.0 (latest with React Router support via Inertia.js)
- **TypeScript:** 5.9.3 (strict type checking)
- **Vite:** rolldown-vite 7.2.5 (faster bundling)

### UI Components (shadcn/ui + Radix UI)
- Style: "new-york" with CSS variables enabled
- Base color: neutral
- Icon library: lucide-react
- All components located in: `src/components/ui/`

Available components:
- accordion, alert, alert-dialog, avatar, badge, button, button-group, calendar
- card, checkbox, collapsible, command, context-menu, dialog, drawer, dropdown-menu
- form, hover-card, input, input-group, item, kbd, label, menubar, navigation-menu
- popover, progress, radio-group, resizable, scroll-area, select, separator, sheet
- sidebar, skeleton, slider, sonner, spinner, switch, table, tabs, textarea, toggle
- toggle-group, tooltip, empty

### Navigation & Routing
- **Inertia.js:** 2.3.6 - SPA-like navigation without client-side routing
- Pages located in: `src/Pages/`
- Page resolution: `import.meta.glob("./Pages/**/*.tsx", { eager: true })`

### Form Handling
- **react-hook-form:** 7.71.1
- **@hookform/resolvers:** 5.2.2
- **zod:** 4.3.5 - Schema validation

### Other Key Libraries
- **lucide-react:** 0.562.0 - Icons
- **sonner:** 2.0.7 - Toast notifications
- **recharts:** 2.15.4 - Charts
- **date-fns:** 4.1.0 - Date utilities
- **next-themes:** 0.4.6 - Dark mode (if used)

## Project Structure

```
web/
├── public/                 # Static assets
├── src/
│   ├── Pages/              # Inertia.js pages (route handlers)
│   │   └── App.tsx         # Default/example page
│   ├── components/
│   │   └── ui/             # shadcn/ui components
│   ├── hooks/              # Custom React hooks
│   │   └── use-mobile.ts   # Mobile detection hook
│   ├── lib/
│   │   └── utils.ts        # Utility functions (cn, etc.)
│   ├── App.css             # Global styles
│   ├── index.css           # Tailwind CSS imports
│   └── main.tsx            # Inertia.js app initialization
├── components.json         # shadcn/ui configuration
├── vite.config.ts          # Vite configuration
├── tsconfig.json           # TypeScript config
└── package.json            # Dependencies
```

## Configuration Files

### components.json (shadcn/ui config)
```json
{
  "style": "new-york",
  "rsc": false,
  "tsx": true,
  "tailwind": {
    "css": "src/index.css",
    "baseColor": "neutral",
    "cssVariables": true,
    "prefix": ""
  },
  "aliases": {
    "components": "@/components",
    "utils": "@/lib/utils",
    "ui": "@/components/ui",
    "lib": "@/lib",
    "hooks": "@/hooks"
  }
}
```

### Path Aliases
- `@/components` → `src/components`
- `@/lib/utils` → `src/lib/utils`
- `@/components/ui` → `src/components/ui`
- `@/lib` → `src/lib`
- `@/hooks` → `src/hooks`

## Inertia.js Integration

### Page Creation Pattern
Pages are React components in `src/Pages/`. Inertia.js automatically resolves and renders them based on the page name returned from the backend.

Example page structure:
```tsx
// src/Pages/YourPage.tsx
import { useState } from "react";
import { Button } from "@/components/ui/button";

function YourPage() {
  return (
    <div>
      <h1>Your Page Title</h1>
      <Button>Click me</Button>
    </div>
  );
}

export default YourPage;
```

### Inertia.js Configuration (main.tsx)
- Page resolution: `import.meta.glob("./Pages/**/*.tsx", { eager: true })`
- Form default recently successful duration: 5000ms
- Prefetch cache: 1 minute
- Prefetch hover delay: 150ms
- Custom headers: `X-Custom-Header: value`

### Proxy Configuration
API proxy configured in vite.config.ts:
- `/inertia` → `http://127.0.0.1:8080`
- `/inertia-version` → `http://127.0.0.1:8080`

## Code Patterns & Conventions

### Component Styling
- Use Tailwind CSS utility classes
- Follow shadcn/ui New York style
- CSS variables enabled for theming
- Neutral base color scheme

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

const formSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8),
});

function YourForm() {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      email: "",
      password: "",
    },
  });

  function onSubmit(values: z.infer<typeof formSchema>) {
    console.log(values);
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)}>
        {/* Form fields */}
      </form>
    </Form>
  );
}
```

### UI Component Usage
Always import from the alias paths:
```tsx
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
```

### Icon Usage
```tsx
import { User, Lock } from "lucide-react";
```

## Build & Development Scripts

```bash
npm run dev          # Start development server
npm run build        # Build for production (tsc -b && vite build)
npm run lint         # Run ESLint
npm run preview      # Preview production build
```

## TypeScript Configuration

- **Strict mode:** Enabled
- **Target:** ES2020
- **Module:** ESNext
- **Module resolution:** bundler (via vite-tsconfig-paths)
- Path aliases configured in `tsconfig.app.json`

## Linting

ESLint configured with:
- ESLint 9.39.1
- typescript-eslint 8.46.4
- React hooks plugin
- React refresh plugin

## Key Considerations for AI Agents

### When Creating New Pages:
1. Create the file in `src/Pages/`
2. Follow the existing page pattern (functional component, export default)
3. Use shadcn/ui components for UI elements
4. Apply proper TypeScript typing
5. Import from alias paths (@/components, @/lib, etc.)

### When Adding Features:
1. Check if shadcn/ui component already exists before creating custom
2. Use react-hook-form + zod for form validation
3. Use lucide-react for icons
4. Follow the "new-york" style and neutral color scheme
5. Ensure type safety - no `as any` or `@ts-ignore`

### When Modifying UI:
1. VISUAL changes (styling, layout, animation) → Delegate to frontend-ui-ux-engineer agent
2. LOGIC changes (state, API calls, business logic) → Can handle directly

### Backend Integration:
- API calls should use Inertia.js for page navigation
- Proxy configured for `/inertia` routes to backend at `http://127.0.0.1:8080`
- Use Inertia.js visit method for programmatic navigation

## Testing & Verification

Before completing tasks:
1. Run `lsp_diagnostics` on changed files
2. Ensure TypeScript compilation succeeds
3. Check that the dev server runs without errors
4. Verify no type errors or linting issues

## Common Imports Reference

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
import { User, Lock, Mail, Eye, EyeOff } from "lucide-react";

// Utilities
import { cn } from "@/lib/utils";
```

## Notes

- This is a modern React 19 project with full TypeScript support
- Uses rolldown-vite for significantly faster builds
- shadcn/ui is fully configured with all standard components available
- Inertia.js provides SPA-like experience without client-side routing complexity
- The codebase follows modern best practices and is well-structured
