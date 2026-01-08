# Frontend - Next.js Application

This is a [Next.js](https://nextjs.org/) project bootstrapped with Next.js 14+ using the App Router architecture.

## Tech Stack

- **Framework**: Next.js 14+ (App Router)
- **Language**: TypeScript
- **Runtime**: Node.js 20+
- **Package Manager**: pnpm

## Getting Started

### Prerequisites

- Node.js 18.17 or higher
- pnpm (or npm/yarn)

### Installation

First, install the dependencies:

```bash
pnpm install
# or
npm install
```

### Development Server

Run the development server:

```bash
pnpm dev
# or
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

The application uses Next.js App Router with:
- `app/layout.tsx` - Root layout component
- `app/page.tsx` - Homepage component

### Build

Create a production build:

```bash
pnpm build
# or
npm run build
```

### Production Server

Start the production server:

```bash
pnpm start
# or
npm run start
```

### Linting

Run ESLint to check code quality:

```bash
pnpm lint
# or
npm run lint
```

## Project Structure

```
frontend/
├── app/                # App Router directory
│   ├── layout.tsx     # Root layout (required)
│   ├── page.tsx       # Homepage
│   └── globals.css    # Global styles
├── public/            # Static assets
├── .eslintrc.json     # ESLint configuration
├── .gitignore         # Git ignore rules
├── next.config.js     # Next.js configuration
├── package.json       # Dependencies and scripts
├── tsconfig.json      # TypeScript configuration
└── README.md          # This file
```

## App Router

This project uses the Next.js App Router (not the legacy Pages Router). Key concepts:

- **Server Components**: Components are Server Components by default
- **Client Components**: Add `"use client"` directive for client-side interactivity
- **File Conventions**: `page.tsx`, `layout.tsx`, `loading.tsx`, `error.tsx`
- **Nested Routing**: Create folders with `page.tsx` for nested routes

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial
- [Next.js App Router](https://nextjs.org/docs/app) - App Router documentation

## Port Information

- **Development**: http://localhost:3000
- **Production**: http://localhost:3000 (configurable via PORT environment variable)

If port 3000 is in use, Next.js will automatically try ports 3001, 3002, etc.
