# Component Architecture

## Authentication Components

### `AuthProvider`

- Context provider for authentication state
- Manages login status
- Provides auth-related methods to children

### `LoginForm`

- Master password input
- Login button
- Error handling display
- Uses shadcn/ui `Card`, `Input`, and `Button` components

### `PasswordChangeForm`

- Old password input
- New password input (with confirmation)
- Validation feedback
- Uses shadcn/ui `Form` components

## Layout Components

### `AppLayout`

- Main layout wrapper
- Navigation structure
- Authentication status display
- Uses shadcn/ui `Sheet` for mobile navigation

### `Sidebar`

- Category navigation
- Quick actions
- Favorites access
- Uses shadcn/ui `ScrollArea`

## Entry Management Components

### `EntryList`

- Displays entry overviews
- Sorting capabilities
- Selection handling
- Uses shadcn/ui `Table` component

### `EntryDetail`

- Shows full entry information
- Edit/view modes
- Password visibility toggle
- Uses shadcn/ui `Card` and form components

### `EntryForm`

- Create/edit entry form
- Category selection
- Password generation integration
- Uses shadcn/ui `Form` components

### `SearchBar`

- Text search input
- Category filter
- Favorites filter
- Uses shadcn/ui `Command` for command palette style search

## Category Management Components

### `CategoryList`

- Displays all categories
- Shows entry counts
- Edit/delete actions
- Uses shadcn/ui `Table`

### `CategoryForm`

- Create/edit category
- Validation feedback
- Uses shadcn/ui form components

## Password Generation Components

### `PasswordGenerator`

- Password generation controls
- Strength indicator
- Copy functionality
- Uses shadcn/ui `Slider` and `Switch` components

### `PasswordSettings`

- Configuration for password generation
- Length control
- Character type toggles
- Uses shadcn/ui form components

## Vault Management Components

### `VaultSettings`

- Key derivation settings
- Backup controls
- Path management
- Uses shadcn/ui `Card` and form components

## Custom Hooks

### `useAuth`

```typescript
interface UseAuth {
  isAuthenticated: boolean;
  login: (password: string) => Promise<void>;
  logout: () => Promise<void>;
  changePassword: (old: string, new: string, confirm: string) => Promise<void>;
}
```

### `useEntries`

```typescript
interface UseEntries {
  entries: EntryOverview[];
  totalCount: number;
  loading: boolean;
  search: (query: SearchQuery) => Promise<void>;
  createEntry: (data: EntryData) => Promise<void>;
  updateEntry: (id: UUID, data: EntryData) => Promise<void>;
  deleteEntry: (id: UUID) => Promise<void>;
  toggleFavorite: (id: UUID) => Promise<void>;
}
```

### `useCategories`

```typescript
interface UseCategories {
  categories: Category[];
  loading: boolean;
  createCategory: (name: string) => Promise<void>;
  deleteCategory: (id: UUID) => Promise<void>;
  renameCategory: (id: UUID, newName: string) => Promise<void>;
}
```

### `usePasswordGenerator`

```typescript
interface UsePasswordGenerator {
  settings: PasswordGeneratorSettings;
  updateSettings: (settings: PasswordGeneratorSettings) => Promise<void>;
  generatePassword: () => Promise<string>;
  entropy: number;
}
```

### `useVault`

```typescript
interface UseVault {
  paths: VaultPaths;
  createBackup: () => Promise<void>;
  restore: (path: string) => Promise<void>;
  keyDerivation: KeyDerivation;
  updateKeyDerivation: (settings: KeyDerivation) => Promise<void>;
}
```

## Error Handling Components

### `ErrorBoundary`

- Global error catching
- Error display
- Recovery options
- Uses shadcn/ui `Alert` components

### `ErrorDialog`

- Contextual error display
- Action retry options
- User guidance
- Uses shadcn/ui `AlertDialog` component

## Loading States

### `LoadingSpinner`

- Activity indicator
- Optional progress display
- Uses shadcn/ui `Progress` component

### `LoadingOverlay`

- Full-screen loading state
- Operation feedback
- Cancel options when applicable
