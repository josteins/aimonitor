# Frontend Component Documentation

## Overview

The AIMonitor project includes frontend components for both desktop (React/TypeScript) and mobile (Flutter/Dart) applications. This document provides detailed information about the component architecture, props, and usage patterns.

## Desktop Application (React/TypeScript)

The desktop application is built with React 18, TypeScript, and Tauri for native integration.

### Component Architecture

```
src/
├── components/
│   ├── Dashboard.tsx          # Main dashboard container
│   ├── ProviderCard.tsx       # Individual provider display
│   ├── AddProviderModal.tsx   # Modal for adding providers
│   └── Sparkline.tsx          # Usage trend visualization
├── styles/                    # CSS stylesheets
├── hooks/                     # Custom React hooks
└── utils/                     # Utility functions
```

### Core Components

#### Dashboard Component

**File:** `src/components/Dashboard.tsx`

The main container component that orchestrates the entire dashboard interface.

**Props:** None (root component)

**State:**
```typescript
interface DashboardState {
  providers: ProviderUsage[];
  loading: boolean;
  showAddProvider: boolean;
  refreshInterval: NodeJS.Timeout | null;
}
```

**Key Features:**
- Auto-refresh every 60 seconds
- Real-time usage data display
- Provider management (add/remove/toggle)
- Cost aggregation and totals

**Methods:**
- `fetchUsage()`: Retrieves usage data from Rust backend
- `handleAddProvider()`: Adds new AI provider
- `handleRemoveProvider()`: Removes existing provider
- `handleToggleProvider()`: Enables/disables provider monitoring
- `getTotalMTDCost()`: Calculates month-to-date costs
- `getTotalTodayCost()`: Calculates today's costs

**Usage:**
```jsx
import { Dashboard } from './components/Dashboard';

function App() {
  return <Dashboard />;
}
```

#### ProviderCard Component

**File:** `src/components/ProviderCard.tsx`

Displays individual AI provider information and usage statistics.

**Props:**
```typescript
interface ProviderCardProps {
  usage: ProviderUsage;
  onToggle: (enabled: boolean) => void;
  onRemove: () => void;
}

interface ProviderUsage {
  provider: Provider;
  today_tokens: number;
  today_cost: number;
  mtd_tokens: number;
  mtd_cost: number;
  balance?: number;
  credits?: number;
  budget_used_percentage?: number;
}
```

**Features:**
- Provider status indicator
- Usage statistics display
- Cost tracking
- Balance/credits information
- Toggle enable/disable
- Remove provider action

**Usage:**
```jsx
<ProviderCard
  usage={providerUsage}
  onToggle={(enabled) => handleToggle(provider.id, enabled)}
  onRemove={() => handleRemove(provider.id)}
/>
```

#### AddProviderModal Component

**File:** `src/components/AddProviderModal.tsx`

Modal dialog for adding new AI providers.

**Props:**
```typescript
interface AddProviderModalProps {
  onAdd: (providerData: AddProviderData) => Promise<void>;
  onClose: () => void;
}

interface AddProviderData {
  provider_type: string;
  api_key: string;
  name: string;
}
```

**Features:**
- Provider type selection (OpenAI, Anthropic, OpenRouter)
- API key input with validation
- Custom name assignment
- Form validation
- Error handling

**Usage:**
```jsx
{showAddProvider && (
  <AddProviderModal
    onAdd={handleAddProvider}
    onClose={() => setShowAddProvider(false)}
  />
)}
```

#### Sparkline Component

**File:** `src/components/Sparkline.tsx`

Miniature chart for displaying usage trends.

**Props:**
```typescript
interface SparklineProps {
  data: number[];
  width?: number;
  height?: number;
  color?: string;
  strokeWidth?: number;
}
```

**Features:**
- SVG-based rendering
- Responsive design
- Customizable styling
- Smooth line interpolation

**Usage:**
```jsx
<Sparkline
  data={usageHistory}
  width={100}
  height={30}
  color="#3B82F6"
/>
```

### Type Definitions

#### Provider Types

```typescript
interface Provider {
  id: string;
  name: string;
  provider_type: 'openai' | 'anthropic' | 'openrouter';
  enabled: boolean;
}

interface ProviderUsage {
  provider: Provider;
  today_tokens: number;
  today_cost: number;
  mtd_tokens: number;
  mtd_cost: number;
  balance?: number;
  credits?: number;
  budget_used_percentage?: number;
}
```

### Styling

The application uses CSS modules for styling with the following structure:

```css
/* Dashboard.css */
.dashboard {
  display: flex;
  flex-direction: column;
  height: 100vh;
  padding: 20px;
}

.dashboard-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 30px;
}

.providers-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
}
```

### Custom Hooks

#### useProviders Hook

```typescript
interface UseProvidersReturn {
  providers: ProviderUsage[];
  loading: boolean;
  error: string | null;
  refresh: () => Promise<void>;
  addProvider: (data: AddProviderData) => Promise<void>;
  removeProvider: (id: string) => Promise<void>;
  toggleProvider: (id: string, enabled: boolean) => Promise<void>;
}

const useProviders = (): UseProvidersReturn => {
  // Implementation
};
```

## Mobile Application (Flutter/Dart)

The mobile application is built with Flutter using Material 3 design principles.

### Widget Architecture

```
lib/
├── main.dart              # App entry point
├── screens/               # Screen widgets
│   ├── dashboard_screen.dart
│   ├── settings_screen.dart
│   └── provider_details_screen.dart
├── widgets/               # Reusable widgets
│   ├── provider_card.dart
│   ├── usage_chart.dart
│   └── add_provider_fab.dart
├── models/                # Data models
├── services/              # Business logic
└── utils/                 # Utility functions
```

### Core Widgets

#### AIMonitorApp

**File:** `lib/main.dart`

The root application widget that sets up theming and navigation.

**Features:**
- Material 3 theming
- Light/dark mode support
- System theme detection
- Route configuration

```dart
class AIMonitorApp extends StatelessWidget {
  const AIMonitorApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'AI Monitor',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: const Color(0xFF3B82F6),
          brightness: Brightness.light,
        ),
        useMaterial3: true,
      ),
      darkTheme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: const Color(0xFF3B82F6),
          brightness: Brightness.dark,
        ),
        useMaterial3: true,
      ),
      themeMode: ThemeMode.system,
      home: const DashboardScreen(),
    );
  }
}
```

#### DashboardScreen

**File:** `lib/screens/dashboard_screen.dart`

The main screen displaying provider usage overview.

**Features:**
- Pull-to-refresh functionality
- Grid layout for provider cards
- Floating action button for adding providers
- Real-time data updates

```dart
class DashboardScreen extends StatefulWidget {
  const DashboardScreen({super.key});

  @override
  State<DashboardScreen> createState() => _DashboardScreenState();
}
```

#### ProviderCard Widget

**File:** `lib/widgets/provider_card.dart`

Displays individual provider information in a card format.

**Properties:**
```dart
class ProviderCard extends StatelessWidget {
  final Provider provider;
  final ProviderUsage usage;
  final VoidCallback? onTap;
  final VoidCallback? onToggle;

  const ProviderCard({
    super.key,
    required this.provider,
    required this.usage,
    this.onTap,
    this.onToggle,
  });
}
```

**Features:**
- Material 3 card design
- Usage statistics display
- Status indicators
- Tap gesture handling
- Toggle switch for enabling/disabling

### State Management

The Flutter app uses Provider pattern for state management:

```dart
class ProviderState extends ChangeNotifier {
  List<Provider> _providers = [];
  bool _loading = false;
  String? _error;

  List<Provider> get providers => _providers;
  bool get loading => _loading;
  String? get error => _error;

  Future<void> fetchProviders() async {
    _loading = true;
    notifyListeners();

    try {
      _providers = await ApiService.getProviders();
      _error = null;
    } catch (e) {
      _error = e.toString();
    } finally {
      _loading = false;
      notifyListeners();
    }
  }
}
```

### Navigation

The app uses named routes for navigation:

```dart
class AppRoutes {
  static const String dashboard = '/';
  static const String settings = '/settings';
  static const String providerDetails = '/provider-details';
  static const String addProvider = '/add-provider';
}
```

### Theming

Material 3 theme configuration:

```dart
class AppTheme {
  static ThemeData lightTheme = ThemeData(
    colorScheme: ColorScheme.fromSeed(
      seedColor: const Color(0xFF3B82F6),
      brightness: Brightness.light,
    ),
    useMaterial3: true,
    cardTheme: CardTheme(
      elevation: 2,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
      ),
    ),
  );

  static ThemeData darkTheme = ThemeData(
    colorScheme: ColorScheme.fromSeed(
      seedColor: const Color(0xFF3B82F6),
      brightness: Brightness.dark,
    ),
    useMaterial3: true,
  );
}
```

## Cloud Poller (TypeScript)

**File:** `apps/cloud-poller/src/index.ts`

Cloudflare Worker providing cloud-based monitoring services.

### API Endpoints

#### POST /poll

Manually trigger provider polling.

**Request:**
```typescript
interface PollRequest {
  providers: string[];
  userId: string;
}
```

**Response:**
```typescript
interface PollResponse {
  success: boolean;
  results: ProviderUsage[];
  errors: string[];
}
```

#### GET /status

Health check endpoint.

**Response:**
```typescript
interface StatusResponse {
  status: 'ok' | 'error';
  timestamp: string;
  version: string;
}
```

### Authentication

Uses JWT tokens for API authentication:

```typescript
import jwt from '@tsndr/cloudflare-worker-jwt';

const authenticateRequest = async (request: Request): Promise<boolean> => {
  const token = request.headers.get('Authorization')?.replace('Bearer ', '');
  if (!token) return false;

  return await jwt.verify(token, SECRET_KEY);
};
```

## Testing

### React Components

```typescript
import { render, screen, fireEvent } from '@testing-library/react';
import { Dashboard } from '../components/Dashboard';

describe('Dashboard Component', () => {
  test('renders loading state', () => {
    render(<Dashboard />);
    expect(screen.getByText('Loading usage data...')).toBeInTheDocument();
  });

  test('handles add provider click', async () => {
    render(<Dashboard />);
    const addButton = screen.getByText('+ Add Provider');
    fireEvent.click(addButton);
    expect(screen.getByText('Add New Provider')).toBeInTheDocument();
  });
});
```

### Flutter Widgets

```dart
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:mobile/widgets/provider_card.dart';

void main() {
  testWidgets('ProviderCard displays provider information', (WidgetTester tester) async {
    final provider = Provider(
      id: '1',
      name: 'Test Provider',
      type: ProviderType.openai,
      enabled: true,
    );

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: ProviderCard(provider: provider),
        ),
      ),
    );

    expect(find.text('Test Provider'), findsOneWidget);
    expect(find.byType(Switch), findsOneWidget);
  });
}
```

## Performance Considerations

### React Application

- **Memoization**: Use `React.memo` for expensive components
- **Virtual Scrolling**: Implement for large provider lists
- **Debounced Updates**: Prevent excessive API calls
- **Code Splitting**: Lazy load heavy components

### Flutter Application

- **Widget Rebuilds**: Minimize unnecessary rebuilds with `const` constructors
- **State Management**: Use Provider for efficient state updates
- **Image Caching**: Cache provider logos and icons
- **List Performance**: Use `ListView.builder` for long lists

### Accessibility

Both applications implement comprehensive accessibility features:

- **Semantic Labels**: All interactive elements have proper labels
- **Keyboard Navigation**: Full keyboard support for desktop
- **Screen Reader Support**: ARIA labels and Flutter semantics
- **High Contrast**: Support for high contrast mode
- **Focus Management**: Proper focus handling for modals and navigation

## Internationalization

The applications support multiple languages through:

**React (i18next):**
```typescript
import { useTranslation } from 'react-i18next';

const { t } = useTranslation();
return <h1>{t('dashboard.title')}</h1>;
```

**Flutter (flutter_localizations):**
```dart
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

Text(AppLocalizations.of(context)!.dashboardTitle)
```

## Build and Deployment

### React Application

```bash
# Development
npm run dev

# Production build
npm run build

# Type checking
npm run type-check

# Linting
npm run lint
```

### Flutter Application

```bash
# Development (iOS)
flutter run -d ios

# Development (Android)
flutter run -d android

# Production builds
flutter build ios --release
flutter build apk --release
flutter build appbundle --release
```

### Code Quality

Both applications enforce code quality through:

- **ESLint/Dart Analysis**: Static code analysis
- **Prettier/dart format**: Code formatting
- **TypeScript/Strong Mode**: Type safety
- **Husky/Git Hooks**: Pre-commit checks
- **Jest/Flutter Test**: Unit and integration testing